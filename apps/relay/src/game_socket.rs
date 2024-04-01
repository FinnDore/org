use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    vec,
};

use crate::{
    org::{self, Org},
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
use tokio::{select, sync::Mutex, time::sleep};
use tracing::{error, info};

const MESSAGE_THROTTLE_MS: u64 = 105;
const SIM_THROTTLE_MS: u64 = 25;

pub async fn game_handler(
    ws: WebSocketUpgrade,
    Path(org_id): Path<String>,
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    info!(org_id, "Gameserver establising connection");
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

    ws.on_upgrade(|socket| handle_game_socket(socket, org_id, state))
}

async fn handle_game_socket(socket: WebSocket, org_id: String, state: SharedState) {
    // TODO reject new connections
    let state_gaurd = state.read().await;
    let mut orgs = state_gaurd.orgs.lock().await;
    let org = orgs
        .entry(org_id.clone())
        .or_insert_with(|| org::Org::new(vec![], true, org_id.clone()));

    info!(
        org_id,
        client_count = org.clients.len(),
        "New game server connected"
    );
    let is_simulation = state_gaurd.simulation;
    drop(orgs);
    drop(state_gaurd);

    let pending_messages: Arc<Mutex<Vec<SceneUpdate>>> = Arc::new(Mutex::new(vec::Vec::new()));

    let send_updates_task = tokio::spawn(send_message_task(
        org_id.clone(),
        state.clone(),
        pending_messages.clone(),
    ));

    let recv_messages_task = tokio::spawn(recv_messages_task(
        socket,
        org_id.clone(),
        state,
        pending_messages,
        is_simulation,
    ));

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

async fn send_message_task(
    org_id: String,
    state: SharedState,
    pending_messages: Arc<Mutex<Vec<SceneUpdate>>>,
) {
    loop {
        sleep(std::time::Duration::from_millis(MESSAGE_THROTTLE_MS)).await;
        let messages = &pending_messages
            .lock()
            .await
            .drain(..)
            .fold(Vec::new(), |mut acc: Vec<SceneUpdate>, incoming_update| {
                if let Some(_) = acc.iter().find(|x| x.id == incoming_update.id) {
                    acc.into_iter()
                        .map(|mut current_update| {
                            if current_update.id == incoming_update.id {
                                current_update.position =
                                    incoming_update.position.or(current_update.position);
                                current_update.rotation =
                                    incoming_update.rotation.or(current_update.rotation);
                                current_update.color =
                                    incoming_update.color.or(current_update.color);
                            }
                            current_update
                        })
                        .collect()
                } else {
                    acc.push(incoming_update);
                    acc
                }
            })
            .iter()
            .map(|x| serde_json::to_string(x).expect("Failed to serialize message"))
            .collect::<Vec<String>>()
            .join(",");

        if messages.is_empty() {
            continue;
        }

        let message_to_send = Message::Text(format!("[{}]", messages));
        let state_gaurd = state.read().await;
        let current_orgs = &mut state_gaurd.orgs.lock().await;
        let org = current_orgs.get_mut(&org_id);
        if org.is_none() {
            info!(org_id, "Org not found cannot send message exiting");
            return;
        }
        send_message_to_client(org.unwrap(), message_to_send).await
    }
}

async fn recv_messages_task(
    mut socket: WebSocket,
    org_id: String,
    state: SharedState,
    pending_messages: Arc<Mutex<Vec<SceneUpdate>>>,
    is_simulation: bool,
) -> () {
    let mut scene = create_test_scene();
    loop {
        let msg = match is_simulation {
            true => {
                select! {
                    r = recv_simulation_frame(&mut scene) => r,
                    r = socket.recv() => r
                }
            }
            false => socket.recv().await,
        };

        match msg {
            Some(Ok(Message::Text(text))) => match serde_json::from_str::<SceneUpdate>(&text) {
                Ok(parsed_update) => pending_messages.lock().await.push(parsed_update),
                Err(err) => {
                    error!(
                        org_id,
                        error = ErrorFormatter::format_serde_error(err),
                        "Error parsing message from gameserver"
                    );
                }
            },

            Some(Ok(close_msg @ Message::Close(_))) => {
                info!(org_id, "Game server disconnected");
                let state_gaurd = state.read().await;
                let current_orgs = &mut state_gaurd.orgs.lock().await;
                let org = current_orgs.get_mut(&org_id);
                if org.is_none() {
                    return;
                }

                let org = org.unwrap();
                if org.clients.is_empty() {
                    current_orgs.remove(&org_id);
                } else {
                    org.server_connected = false;
                    send_message_to_client(org, close_msg).await;
                }

                return;
            }
            Some(Err(err)) => {
                error!(
                    org_id = org_id,
                    error = ErrorFormatter::format_axum_error(err),
                    "Error receiving message from simulation or gameserver"
                );
            }
            Some(Ok(_)) => {}
            None => {}
        }
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

static DID_COLOR: AtomicBool = AtomicBool::new(false);

async fn recv_simulation_frame(scene: &mut scene::Scene) -> Option<Result<Message, Error>> {
    sleep(std::time::Duration::from_millis(SIM_THROTTLE_MS)).await;

    let item_index_to_update = {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..scene.items.len())
    };
    let item_to_update = scene.items.get_mut(item_index_to_update);

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
                    id: item_index_to_update.to_string(),
                    color: Some(item.color),
                    rotation: None,
                    position: None,
                })
                .unwrap(),
            )))
        }
        false => {
            item.rotation.1 += 0.2;
            item.rotation.2 += 0.2;
            Some(Ok(Message::Text(
                serde_json::to_string(&SceneUpdate {
                    id: item_index_to_update.to_string(),
                    rotation: Some((item.rotation.0, item.rotation.1, item.rotation.2)),
                    color: None,
                    position: None,
                })
                .unwrap(),
            )))
        }
    }
}
