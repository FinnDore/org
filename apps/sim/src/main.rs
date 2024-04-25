use std::fs::read;
use std::path;
use std::str::FromStr;

use clap::{arg, command, Parser};

use futures_util::SinkExt;
use futures_util::StreamExt;
use serde::Deserialize;
use serde::Serialize;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite;
use tokio_tungstenite::tungstenite::handshake::client::Request;
use tokio_tungstenite::tungstenite::http::uri;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::Message;
use tracing::info;
use tracing::instrument;

/// Simulation of a game server
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input demo file
    #[arg(short, long)]
    file: String,

    /// Authentication token
    #[arg(short, long)]
    auth_token: String,

    /// Url of the server
    #[arg(short, long)]
    server: String,

    /// Game id to connect with
    #[arg(short, long)]
    game_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SceneUpdate {
    pub id: String,
    pub rotation: Option<(f32, f32, f32)>,
    pub position: Option<(f32, f32, f32)>,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InitScene {
    pub name: String,
    pub items: Vec<SceneUpdate>,
}

#[tokio::main]
#[instrument]
async fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::INFO)
            .finish(),
    )
    .expect("Failed to set subscriber");
    let args = Args::parse();

    let frames = read(path::Path::new(&args.file)).expect("Failed to read file");

    let frames =
        serde_json::from_slice::<Vec<Vec<SceneUpdate>>>(&frames).expect("Failed to parse json");

    let body = InitScene {
        name: args.file,
        items: frames[0].clone(),
    };

    let scene_updates = frames
        .iter()
        .map(|frame| {
            frame
                .iter()
                .map(|update| {
                    tungstenite::Message::Text(
                        serde_json::to_string(update).expect("Failed to serialize"),
                    )
                })
                .collect()
        })
        .collect::<Vec<Vec<Message>>>();

    let server_uri = uri::Uri::from_str(&format!("{}/game/{}", &args.server, &args.game_id))
        .expect("Failed to parse uri");
    let request = Request::builder()
        .uri(&server_uri)
        .header("Host", server_uri.host().unwrap())
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header(
            "Sec-WebSocket-Key",
            tokio_tungstenite::tungstenite::handshake::client::generate_key(),
        )
        .header(
            "authorization",
            HeaderValue::from_str(&args.auth_token).expect("Failed to parse auth header"),
        )
        .body(body)
        .expect("Failed to build request");

    let (ws_stream, _) = connect_async(request).await.expect("Failed to connect");

    info!("WebSocket handshake has been successfully completed");

    let (mut write, _) = ws_stream.split();

    info!("Running sim ");
    loop {
        for update in scene_updates.iter() {
            tokio::time::sleep(std::time::Duration::from_millis(25)).await;
            for frame in update.iter() {
                write
                    .send(frame.clone())
                    .await
                    .expect("Failed to send message");
            }
        }
    }
}
