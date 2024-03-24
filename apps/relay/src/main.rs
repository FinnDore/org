mod org_client;

use std::{borrow::BorrowMut, sync::Arc};

use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Path,
    },
    response::Response,
    routing::get,
    Extension, Router,
};
use org_client::Orgs;
use tokio::sync::Mutex;

use crate::org_client::{add_client_to_org, Org};
struct State {
    pub orgs: Orgs,
}

type SharedState = Arc<Mutex<State>>;

async fn client_handler(
    ws: WebSocketUpgrade,
    Path(org_id): Path<String>,
    Extension(state): Extension<SharedState>,
) -> Response {
    println!("New client connected to org: {:?}", org_id);
    ws.on_upgrade(|socket| handle_client_socket(socket, org_id, state))
}

async fn handle_client_socket(socket: WebSocket, org_id: String, state: SharedState) {
    let mut state = state.lock().await;
    let orgs = state.orgs.borrow_mut();
    add_client_to_org(orgs, org_id, socket);
}

async fn game_handler(
    ws: WebSocketUpgrade,
    Path(org_id): Path<String>,
    Extension(state): Extension<SharedState>,
) -> Response {
    println!("New game server connected to org: {:?}", org_id);
    ws.on_upgrade(|socket| handle_game_socket(socket, org_id, state))
}

async fn handle_game_socket(mut socket: WebSocket, org_id: String, state: SharedState) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };
        let mut state = state.lock().await;
        let orgs = state.orgs.borrow_mut();

        if let Some(current_org) = orgs.orgs.get_mut(&org_id) {
            for client in &mut current_org.clients {
                if let Err(err) = client.send(msg.clone()).await {
                    println!("Error sending message: {:?}", err);
                };
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/sub/:org", get(client_handler))
        .route("/game/:org", get(game_handler));
    let port = std::env::var("PORT").unwrap_or("3001".to_string());
    let host = format!("0.0.0.0:{:}", port);
    println!("Running server on {:}", host);

    let state = Arc::new(Mutex::new(State { orgs: Orgs::new() }));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app.layer(Extension(state)).into_make_service())
        .await
        .unwrap();
}
