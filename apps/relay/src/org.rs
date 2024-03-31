use axum::extract::ws::Message;
use tokio::sync::mpsc::UnboundedSender;

pub struct Org {
    pub id: String,
    pub clients: Vec<Client>,
    pub server_connected: bool,
}

impl Org {
    pub fn new(clients: Vec<Client>, server_connected: bool, id: String) -> Self {
        Self {
            clients,
            server_connected,
            id,
        }
    }
}

pub struct Client {
    pub client_id: usize,
    pub tx: UnboundedSender<Message>,
}
