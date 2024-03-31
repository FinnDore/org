mod client_socket;
mod game_socket;
mod org;
mod scene;
mod util;

use org::Org;
use scene::Scene;
use std::{collections::HashMap, sync::Arc};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber;

use crate::{client_socket::client_handler, game_socket::game_handler, scene::get_scene};
use axum::{routing::get, Router};

use tokio::sync::{Mutex, RwLock};

pub struct TheState {
    pub auth_token: String,
    pub simulation: bool,
    pub orgs: Mutex<HashMap<String, Org>>,
    pub scene: RwLock<Scene>,
}

impl TheState {
    pub fn new(auth_token: String, simulation: bool) -> Self {
        Self {
            orgs: Mutex::new(HashMap::new()),
            scene: RwLock::new(scene::create_test_scene()),
            auth_token,
            simulation,
        }
    }
}

pub type SharedState = Arc<RwLock<TheState>>;

#[tokio::main]
async fn main() {
    let env = std::env::var("ENV").unwrap_or("production".into());
    if env == "development" {
        tracing_subscriber::fmt().without_time().init();
    } else {
        let log_level = std::env::var("LOG_LEVEL")
            .unwrap_or("info".to_string())
            .parse::<LevelFilter>()
            .expect("Invalid log level");
        tracing_subscriber::fmt().with_max_level(log_level).init();
    };

    let auth_token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN env var set");
    let simulation = std::env::var("SIMULATE").unwrap_or_default() == "true";
    let state = Arc::new(RwLock::new(TheState::new(auth_token, simulation)));

    let app = Router::new()
        .route("/sub/:org", get(client_handler))
        .route("/game/:org", get(game_handler))
        .route("/scene/:org", get(get_scene))
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or("3002".to_string());
    let host = format!("0.0.0.0:{}", port);
    info!("Running server on {}", host);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
