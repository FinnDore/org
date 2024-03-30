mod client_socket;
mod game_socket;
mod org_client;
mod util;

use std::sync::Arc;
use tracing::{info, level_filters::LevelFilter, subscriber};
use tracing_subscriber;

use crate::{client_socket::client_handler, game_socket::game_handler, org_client::TheState};
use axum::{routing::get, Router};

use tokio::sync::RwLock;

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
    let state = Arc::new(RwLock::new(TheState::new(auth_token)));

    let app = Router::new()
        .route("/sub/:org", get(client_handler))
        .route("/game/:org", get(game_handler))
        .with_state(state);
    let port = std::env::var("PORT").unwrap_or("3001".to_string());
    let host = format!("0.0.0.0:{}", port);
    info!("Running server on {}", host);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
