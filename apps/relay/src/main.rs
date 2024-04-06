mod client_socket;
mod data;
mod game_socket;
mod org;
mod scene;
mod util;

use org::Org;
use scene::Scene;
use std::str::FromStr;
use std::{collections::HashMap, sync::Arc};
use tracing::{info, level_filters::LevelFilter};
use tracing::{instrument, span};

use crate::{client_socket::client_handler, game_socket::game_handler, scene::get_scene};
use axum::{routing::get, Router};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::{
    fmt::{self, layer, writer::WithMaxLevel},
    layer::{self, Filter},
    prelude::*,
    registry, Registry,
};

use tokio::sync::{Mutex, RwLock};

#[derive(Debug)]
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

pub type SharedState = Arc<TheState>;

#[tokio::main]
#[instrument]
async fn main() {
    let env = std::env::var("ENV").unwrap_or("production".into());
    if env == "development" {
        tracing_subscriber::fmt().without_time().init();
    } else {
        let env_filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::DEBUG.into())
            .from_env()
            .expect("Failed to create env filter invalid RUST_LOG env var");

        let registry = Registry::default().with(env_filter).with(fmt::layer());

        if let Ok(_) = std::env::var("AXIOM_TOKEN") {
            let axiom_layer = tracing_axiom::builder()
                .with_service_name("org")
                .with_tags(&[(
                    &"deployment_id",
                    &std::env::var("RAILWAY_DEPLOYMENT_ID")
                        .map(|s| {
                            s + "-"
                                + std::env::var("RAILWAY_DEPLOYMENT_ID")
                                    .unwrap_or("unknown_replica".into())
                                    .as_str()
                        })
                        .unwrap_or("unknown_deployment".into()),
                )])
                .with_tags(&[(&"service.name", "org".into())])
                .layer()
                .expect("Axiom layer failed to initialize");

            registry
                .with(axiom_layer)
                .try_init()
                .expect("Failed to initialize tracing with axiom");
            info!("Initialized tracing with axiom");
        } else {
            registry.try_init().expect("Failed to initialize tracing");
        }
    };

    let auth_token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN env var set");
    let simulation = std::env::var("SIMULATE").unwrap_or_default() == "true";
    let state = Arc::new(TheState::new(auth_token, simulation));

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
