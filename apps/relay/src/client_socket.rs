use std::sync::atomic::AtomicUsize;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::Response,
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tracing::{error, info};

use crate::{
    org_client::{Client, Org, SharedState},
    util::ErrorFormatter,
};
use futures_util::{future::select_all, sink::SinkExt, stream::StreamExt};

static CLIENT_COUNT: AtomicUsize = AtomicUsize::new(0);

pub async fn client_handler(
    ws: WebSocketUpgrade,
    Path(org_id): Path<String>,
    State(state): State<SharedState>,
) -> Response {
    ws.on_upgrade(|socket| handle_client_socket(socket, org_id, state))
}

async fn handle_client_socket(ws: WebSocket, org_id: String, state: SharedState) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let (tx, mut incoming_messages_rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
        mpsc::unbounded_channel();
    let client_id = CLIENT_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    // TODO log ip here
    info!(org_id, client_id, "New client connected");

    let ctx_gaurd = state.as_ref().write().await;
    let mut current_orgs = ctx_gaurd.orgs.lock().await;
    current_orgs
        .entry(org_id.clone())
        .or_insert_with(|| Org::new(vec![]))
        .clients
        .push(Client { tx, client_id });

    drop(current_orgs);
    drop(ctx_gaurd);

    let org_id_for_message_task = org_id.clone();
    let message_task = tokio::spawn(async move {
        while let Some(msg) = incoming_messages_rx.recv().await {
            match msg {
                msg @ Message::Text(_) => {
                    if let Err(err) = ws_tx.send(msg).await {
                        error!(
                            org_id = org_id_for_message_task,
                            client_id,
                            error = ErrorFormatter::format_axum_error(err),
                            "Error sending message"
                        );
                    }
                }
                _ => continue,
            };
        }
    });

    let org_id_for_disconnect_task = org_id.clone();
    let disconnect_task = tokio::spawn(async move {
        while let Some(msg) = ws_rx.next().await {
            match msg {
                Ok(Message::Close(_)) => {
                    info!(
                        org_id = org_id_for_disconnect_task,
                        client_id, "Client disconnected",
                    );
                    return;
                }
                Ok(Message::Text(incoming_message)) => {
                    info!(
                        org_id = org_id_for_disconnect_task,
                        client_id, incoming_message, "Message from client",
                    );
                }
                Ok(_) => continue,
                Err(err) => {
                    error!(
                        org_id = org_id_for_disconnect_task,
                        client_id,
                        error = ErrorFormatter::format_axum_error(err),
                        "Error receiving message"
                    );
                }
            };
        }
    });

    let remaining_tasks = match select_all(vec![message_task, disconnect_task]).await {
        (Ok(_), _, remaining) => remaining,
        (Err(err), index, remaining) => {
            error!(
                org_id,
                client_id,
                index,
                error = ErrorFormatter::format_join_error(err),
                "Error in ws handling task, error"
            );
            remaining
        }
    };

    for task in remaining_tasks {
        task.abort();
    }

    let ctx_gaurd = state.write().await;
    let mut current_orgs = ctx_gaurd.orgs.lock().await;
    if let Some(org) = current_orgs.get_mut(&org_id) {
        org.clients.retain(|client| client.client_id != client_id);
    }
}