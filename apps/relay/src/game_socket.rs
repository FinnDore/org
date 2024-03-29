

use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Path, State,
    },
    http::{status, HeaderMap},
    response::{IntoResponse},
};


use crate::org_client::{SharedState};

pub async fn game_handler(
    ws: WebSocketUpgrade,
    Path(org_id): Path<String>,
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization");
    if auth_header.is_none() {
        return status::StatusCode::UNAUTHORIZED.into_response();
    }

    let auth_token = auth_header.unwrap().to_str();
    if auth_token.is_err() {
        return status::StatusCode::UNAUTHORIZED.into_response();
    }

    // let current_state = state.read().await;
    if std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN env var set") != auth_token.unwrap() {
        return status::StatusCode::UNAUTHORIZED.into_response();
    }

    println!("New game server connected to org: {:?}", org_id);
    ws.on_upgrade(|socket| handle_game_socket(socket, org_id, state))
}

pub async fn handle_game_socket(mut socket: WebSocket, org_id: String, state: SharedState) {
    while let Some(msg) = socket.recv().await {
        if let Err(err) = msg {
            println!("Error receiving message: {:?}", err);
            return;
        }

        let msg = msg.unwrap();
        let orgs = &mut state.write().await.orgs;
        let mut current_orgs = orgs.lock().await;
        if let Some(org) = current_orgs.get_mut(&org_id) {
            // TODO dont block here
            for client in &mut org.clients {
                if let Err(err) = client.tx.send(msg.clone()) {
                    println!(
                        "Error producing message to client {}: {:?}",
                        client.client_id, err
                    );
                }
            }
        } else {
            println!("Cannot send message org not found: {:?}", org_id);
        }
    }
}
