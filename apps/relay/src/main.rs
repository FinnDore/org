mod org_client;

use std::{
    borrow::{Borrow, BorrowMut},
    sync::Arc,
    time::Duration,
};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    http::{status, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use org_client::{add_client_to_org, Orgs};
use tokio::sync::{Mutex, RwLock};

use crate::org_client::Org;
struct TheState {
    pub orgs: Orgs,
    pub auth_token: String,
}

type SharedState = Arc<RwLock<TheState>>;

async fn client_handler(
    ws: WebSocketUpgrade,
    Path(org_id): Path<String>,
    State(state): State<SharedState>,
) -> Response {
    println!("New client connected to org: {:?}", org_id);
    ws.on_upgrade(|socket| handle_client_socket(socket, org_id, state))
}

async fn handle_client_socket(socket: WebSocket, org_id: String, state: SharedState) {
    let current_state = &mut state.write().await.orgs;
    let mut current_orgs = current_state.orgs.lock().await;
    if let Some(org) = current_orgs.get_mut(&org_id) {
        org.clients.push(socket);
    } else {
        let org = Org::new(vec![socket]);
        current_orgs.insert(org_id, org);
    }
}

async fn game_handler(
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

async fn handle_game_socket(mut socket: WebSocket, org_id: String, state: SharedState) {
    while let Some(msg) = socket.recv().await {
        if let Err(err) = msg {
            println!("Error receiving message: {:?}", err);
            return;
        }

        let msg = msg.unwrap();
        let current_state = &mut state.write().await.orgs;
        let mut current_orgs = current_state.orgs.lock().await;
        if let Some(org) = current_orgs.get_mut(&org_id) {
            // TODO dont block here
            for client in &mut org.clients {
                if let Err(err) = client.send(msg.clone()).await {
                    println!("Error sending message: {:?}", err);
                };
            }
        } else {
            println!("Cannot send message org not found: {:?}", org_id);
        }
    }
}

#[tokio::main]
async fn main() {
    let auth_token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN env var set");
    let state = Arc::new(RwLock::new(TheState {
        orgs: Orgs::new(),
        auth_token,
    }));
    let app = Router::new()
        .route("/sub/:org", get(client_handler))
        .route("/game/:org", get(game_handler))
        .with_state(state);
    let port = std::env::var("PORT").unwrap_or("3001".to_string());
    let host = format!("0.0.0.0:{:}", port);
    println!("Running server on {:}", host);

    // let state = RwLock::new(State {
    //     orgs: Orgs::new(),
    //     auth_token,
    // });

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
