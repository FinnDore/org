use crate::{util::ErrorFormatter, SharedState};
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Path, State,
    },
    http::{status, HeaderMap},
    response::IntoResponse,
};
use tracing::{error, info};

pub async fn game_handler(
    ws: WebSocketUpgrade,
    Path(org_id): Path<String>,
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let auth_header = headers
        .get("authorization")
        .map(|header| header.to_str().ok())
        .flatten();

    let current_state = state.read().await;
    if auth_header.is_none() || current_state.auth_token != auth_header.unwrap() {
        info!(
            org_id,
            "Failed to connect auth header is {}",
            if auth_header.is_none() {
                "missing"
            } else {
                "invalid"
            }
        );
        return status::StatusCode::UNAUTHORIZED.into_response();
    }

    drop(current_state);

    info!(org_id, "New game server connected");
    ws.on_upgrade(|socket| handle_game_socket(socket, org_id, state))
}

pub async fn handle_game_socket(mut socket: WebSocket, org_id: String, state: SharedState) {
    while let Some(msg) = socket.recv().await {
        if let Err(err) = msg {
            error!("Error receiving message {}", err);
            return;
        }

        let msg = msg.unwrap();
        let orgs = &mut state.write().await.orgs;
        let mut current_orgs = orgs.lock().await;
        if let Some(org) = current_orgs.get_mut(&org_id) {
            for client in &mut org.clients {
                if let Err(err) = client.tx.send(msg.clone()) {
                    error!(
                        client_id = client.client_id,
                        client.client_id,
                        error = ErrorFormatter::format_ws_send_error(err),
                        "Error producing message to client"
                    );
                }
            }
        } else {
            info!(org_id, "Cannot send message no connected clients found");
        }
    }
}
