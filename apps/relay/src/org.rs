use axum::extract::ws::Message;
use tokio::sync::mpsc::UnboundedSender;

use crate::scene::Scene;

#[derive(Debug)]
pub struct Org {
    pub id: String,
    pub clients: Vec<Client>,
    pub server_connected: bool,
    pub scene: Scene,
}

impl Org {
    pub fn new(clients: Vec<Client>, server_connected: bool, id: String, scene: Scene) -> Self {
        Self {
            clients,
            server_connected,
            id,
            scene,
        }
    }
}

#[derive(Debug)]
pub struct Client {
    pub client_id: usize,
    pub tx: UnboundedSender<Message>,
}
