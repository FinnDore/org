use crate::{
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
use rand::Rng;
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

async fn handle_game_socket(mut socket: WebSocket, org_id: String, state: SharedState) {
    let state_gaurd = state.read().await;
    let is_simulation = state_gaurd.simulation.clone();
    drop(state_gaurd);

    let mut sim = Simualtion {
        scene: create_test_scene(),
    };
    println!("Simulation: {:?}", is_simulation);

    loop {
        let msg = match is_simulation {
            true => recv_simulation_frame(&mut sim).await,
            false => socket.recv().await,
        };

        if msg.is_none() {
            info!(
                org_id,
                "Cannot recive message from simulation or gameserver"
            );
            return;
        };

        let msg = msg.unwrap();
        if let Err(err) = msg {
            error!(
                org_id,
                error = ErrorFormatter::format_axum_error(err),
                "Error receiving message from simulation or gameserver"
            );
            return;
        }
        let update_msg = msg.unwrap();

        let state_gaurd = &mut state.write().await;
        let mut current_orgs = state_gaurd.orgs.lock().await;
        if let Some(org) = current_orgs.get_mut(&org_id) {
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
        }
    }
}

struct Simualtion {
    scene: scene::Scene,
}

async fn recv_simulation_frame(simulation: &mut Simualtion) -> Option<Result<Message, Error>> {
    let item_index_to_update = {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..simulation.scene.items.len())
    };
    let item_to_update = simulation.scene.items.get_mut(item_index_to_update);

    if item_to_update.is_none() {
        sleep(std::time::Duration::from_millis(5)).await;
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

    sleep(std::time::Duration::from_millis(50)).await;
    Some(Ok(update_msg))
}
