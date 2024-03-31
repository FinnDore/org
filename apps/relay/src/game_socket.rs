use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    vec,
};

use crate::{
    org::{self, Org},
    scene::{self, create_test_scene, SceneUpdate, SceneUpdateType},
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
use tokio::{select, sync::Mutex, time::sleep};
use tracing::{error, info};

const MESSAGE_THROTTLE_MS: u64 = 105;
const SIM_THROTTLE_MS: u64 = 105;

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
    // TODO reject new connections
    let state_gaurd = state.read().await;
    state_gaurd
        .orgs
        .lock()
        .await
        .entry(org_id.clone())
        .or_insert_with(|| org::Org::new(vec![], true, org_id.clone()));

    let is_simulation = state_gaurd.simulation;
    drop(state_gaurd);

    let message_backlog: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec::Vec::new()));
    let message_backlog_for_send_task = message_backlog.clone();

    let state_for_send_updates_task = state.clone();
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

            if messages.is_empty() {
                continue;
            }

            let message_to_send = Message::Text(format!("[{}]", messages));
            let state_gaurd = state_for_send_updates_task.read().await;
            let current_orgs = &mut state_gaurd.orgs.lock().await;
            let org = current_orgs.get_mut(&org_id_for_recv_task);
            if org.is_none() {
                info!(
                    org_id_for_recv_task,
                    "Org not found cannot send message exiting"
                );
                return;
            }
            send_message_to_client(org.unwrap(), message_to_send).await
        }
    });

    let org_id_for_message_task = org_id.clone();
    let recv_messages_task = tokio::spawn(async move {
        let mut sim = Simualtion {
            scene: create_test_scene(),
        };
        loop {
            let msg = match is_simulation {
                true => {
                    select! {
                        r = recv_simulation_frame(&mut sim) => r,
                        r= socket.recv() => r
                    }
                }
                false => socket.recv().await,
            };

            match msg {
                Some(Ok(Message::Text(text))) => {
                    let _ = &mut message_backlog_for_send_task.lock().await.push(text);
                }
                Some(Ok(close_msg @ Message::Close(_))) => {
                    info!(org_id_for_message_task, "Game server disconnected");
                    let state_gaurd = state.read().await;
                    let current_orgs = &mut state_gaurd.orgs.lock().await;
                    let org = current_orgs.get_mut(&org_id_for_message_task);
                    if org.is_none() {
                        info!(org_id_for_message_task, "Org not found cannot send message");
                        return;
                    }

                    let org = org.unwrap();
                    if org.clients.is_empty() {
                        println!("Removing org");
                        current_orgs.remove(&org_id_for_message_task);
                    } else {
                        println!("not org");
                        org.server_connected = false;
                        send_message_to_client(org, close_msg).await;
                    }

                    break;
                }
                Some(Err(err)) => {
                    error!(
                        org_id = org_id_for_message_task,
                        error = ErrorFormatter::format_axum_error(err),
                        "Error receiving message from simulation or gameserver"
                    );
                }
                Some(Ok(_)) => {}
                None => {}
            }
            // TODO We can probbaly coalce updates into a single containing the final state
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
}

async fn send_message_to_client(org: &mut Org, message: Message) -> () {
    for client in org.clients.iter() {
        if let Err(err) = client.tx.send(message.clone()) {
            error!(
                org_id = org.id,
                client_id = client.client_id,
                error = ErrorFormatter::format_ws_send_error(err),
                "Error producing message to client"
            );
        }
    }
}

struct Simualtion {
    scene: scene::Scene,
}

static DID_COLOR: AtomicBool = AtomicBool::new(false);

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
    let update_color = DID_COLOR
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |val| Some(!val))
        .unwrap();
    match update_color {
        true => {
            item.color.increment();
            Some(Ok(Message::Text(
                serde_json::to_string(&SceneUpdate {
                    object_id: item_index_to_update.to_string(),
                    path: "color".into(),
                    value: SceneUpdateType::Color(item.color),
                })
                .unwrap(),
            )))
        }
        false => {
            item.rotation.1 += 0.2;
            item.rotation.2 += 0.2;
            Some(Ok(Message::Text(
                serde_json::to_string(&SceneUpdate {
                    object_id: item_index_to_update.to_string(),
                    path: "rotation".into(),
                    value: SceneUpdateType::Rotation(vec![
                        item.rotation.0,
                        item.rotation.1,
                        item.rotation.2,
                    ]),
                })
                .unwrap(),
            )))
        }
        _ => None,
    }
}
