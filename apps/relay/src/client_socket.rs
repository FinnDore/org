use std::sync::atomic::AtomicUsize;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::Response,
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::org_client::{Client, Org, SharedState};
use futures_util::{sink::SinkExt, stream::StreamExt};

static CLIENT_COUNT: AtomicUsize = AtomicUsize::new(0);

pub async fn client_handler(
    ws: WebSocketUpgrade,
    Path(org_id): Path<String>,
    State(state): State<SharedState>,
) -> Response {
    println!("New client connected to org: {:?}", org_id);
    ws.on_upgrade(|socket| handle_client_socket(socket, org_id, state))
}

async fn handle_client_socket(ws: WebSocket, org_id: String, state: SharedState) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let (tx, mut incoming_messages_rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
        mpsc::unbounded_channel();
    let client_id = CLIENT_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    println!("Client id: {}", client_id);

    let ctx_gaurd = state.as_ref().write().await;
    let mut current_orgs = ctx_gaurd.orgs.lock().await;
    current_orgs
        .entry(org_id.clone())
        .or_insert_with(|| Org::new(vec![]))
        .clients
        .push(Client { tx, client_id });

    drop(current_orgs);
    drop(ctx_gaurd);
    tokio::spawn(async move {
        while let Some(msg) = incoming_messages_rx.recv().await {
            let _ = match msg {
                msg @ Message::Text(_) => ws_tx.send(msg).await,
                _ => continue,
            };
        }
    });

    tokio::spawn(async move {
        while let Some(msg) = ws_rx.next().await {
            match msg {
                Ok(Message::Close(_)) => {
                    println!("Client disconnected");
                    let ctx_gaurd = state.write().await;
                    let mut current_orgs = ctx_gaurd.orgs.lock().await;
                    if let Some(org) = current_orgs.get_mut(&org_id) {
                        org.clients.retain(|client| client.client_id != client_id);
                    }
                }
                Ok(_) => continue,
                Err(err) => {
                    println!("Error receiving message: {:?}", err);
                }
            };
        }
    });

    // TODO do task cleanup
}
