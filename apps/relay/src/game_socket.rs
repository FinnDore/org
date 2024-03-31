use crate::{
    scene::{self, SceneUpdate},
    util::ErrorFormatter,
    SharedState,
};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    http::{status, HeaderMap},
    response::IntoResponse,
};
use rand::{random, seq::SliceRandom, Rng};
use tokio::time::sleep;
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
    loop {
        // while let Some(msg) = socket.recv().await {
        // if let Err(err) = msg {
        //     error!("Error receiving message {}", err);
        //     return;
        // }

        let state_gaurd = &mut state.write().await;
        let mut current_orgs = state_gaurd.orgs.lock().await;
        if let Some(org) = current_orgs.get_mut(&org_id) {
            let mut scene = state_gaurd.scene.write().await;
            let mut rng = rand::thread_rng();
            let item_index_to_update = rng.gen_range(0..scene.items.len());

            let item_to_update = scene.items.get_mut(item_index_to_update);
            if item_to_update.is_none() {
                info!(org_id, "No items in scene");
                return;
            }

            let item = item_to_update.unwrap();
            item.rotation.1 += 0.2;
            let update_msg = Message::Text(
                serde_json::to_string(&SceneUpdate {
                    object_id: item_index_to_update.to_string(),
                    path: "rotation".into(),
                    value: item.rotation,
                })
                .unwrap(),
            );

            for client in &mut org.clients {
                if let Err(err) = client.tx.send(update_msg.clone()) {
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
        sleep(std::time::Duration::from_millis(5)).await;
    }
}
