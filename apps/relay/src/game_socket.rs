use std::{sync::Arc, vec};

use crate::{
    org,
    scene::{self, create_test_scene, SceneUpdate},
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
    Error,
};
use futures_util::future::select_all;
use rand::Rng;
use tokio::{sync::Mutex, time::sleep};
use tracing::{error, info};

const MESSAGE_THROTTLE_MS: u64 = 50;
const SIM_THROTTLE_MS: u64 = 25;

pub async fn game_handler(
    ws: WebSocketUpgrade,
    Path(org_id): Path<String>,
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let auth_header = headers
        .get("authorization")
        .and_then(|header| header.to_str().ok());

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

async fn handle_game_socket(mut socket: WebSocket, org_id: String, state: SharedState) {
    let state_gaurd = state.read().await;
    state_gaurd
        .orgs
        .lock()
        .await
        .entry(org_id.clone())
        .or_insert_with(|| org::Org::new(vec![]));

    let is_simulation = state_gaurd.simulation;
    drop(state_gaurd);

    let message_backlog: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec::Vec::new()));
    let message_backlog_for_send_task = message_backlog.clone();

    let org_id_for_recv_task = org_id.clone();
    let send_updates_task = tokio::spawn(async move {
        loop {
            sleep(std::time::Duration::from_millis(MESSAGE_THROTTLE_MS)).await;
            let messages = &message_backlog
                .lock()
                .await
                .drain(..)
                .collect::<Vec<String>>()
                .join(",");

            let message_to_send = Message::Text(format!("[{}]", messages));
            let state_gaurd = state.read().await;
            let current_orgs = state_gaurd.orgs.lock().await;
            let org = current_orgs.get(&org_id_for_recv_task);
            if org.is_none() {
                info!(org_id_for_recv_task, "Org not found");
                continue;
            }

            for client in org.unwrap().clients.iter() {
                if let Err(err) = client.tx.send(message_to_send.clone()) {
                    error!(
                        org_id = org_id_for_recv_task,
                        client_id = client.client_id,
                        error = ErrorFormatter::format_ws_send_error(err),
                        "Error producing message to client"
                    );
                }
            }
        }
    });

    let org_id_for_message_task = org_id.clone();
    let recv_messages_task = tokio::spawn(async move {
        let mut sim = Simualtion {
            scene: create_test_scene(),
        };
        loop {
            // TODO we still need to handle server disconnects
            let msg = match is_simulation {
                true => recv_simulation_frame(&mut sim).await,
                false => socket.recv().await,
            };

            if msg.is_none() {
                info!(
                    org_id = org_id_for_message_task,
                    "Cannot recive message from simulation or gameserver"
                );
                continue;
            };

            let msg = msg.unwrap();
            if let Err(err) = msg {
                error!(
                    org_id = org_id_for_message_task,
                    error = ErrorFormatter::format_axum_error(err),
                    "Error receiving message from simulation or gameserver"
                );
                continue;
            }
            if let Message::Text(text) = msg.unwrap() {
                let _ = &mut message_backlog_for_send_task.lock().await.push(text);
            } else {
                info!(org_id_for_message_task, "Received non-text message");
            }
        }
    });
    let remaining_tasks = match select_all(vec![send_updates_task, recv_messages_task]).await {
        (Ok(_), _, remaining) => remaining,
        (Err(err), index, remaining) => {
            error!(
                org_id,
                index,
                error = ErrorFormatter::format_join_error(err),
                "Error in gamerserver handling task"
            );
            remaining
        }
    };

    for task in remaining_tasks {
        task.abort();
    }

    // TODO handle cleanup of orphan tasks
}

struct Simualtion {
    scene: scene::Scene,
}

async fn recv_simulation_frame(simulation: &mut Simualtion) -> Option<Result<Message, Error>> {
    sleep(std::time::Duration::from_millis(SIM_THROTTLE_MS)).await;
    let item_index_to_update = {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..simulation.scene.items.len())
    };
    let item_to_update = simulation.scene.items.get_mut(item_index_to_update);

    if item_to_update.is_none() {
        return Some(Err(Error::new("No items in scene")));
    }

    let item = item_to_update.unwrap();
    item.rotation.1 += 0.2;
    item.rotation.2 += 0.2;
    let update_msg = Message::Text(
        serde_json::to_string(&SceneUpdate {
            object_id: item_index_to_update.to_string(),
            path: "rotation".into(),
            value: item.rotation,
        })
        .unwrap(),
    );

    Some(Ok(update_msg))
}
