use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::{Message};
use tokio::sync::{mpsc::UnboundedSender, Mutex, RwLock};

pub struct Org {
    pub clients: Vec<Client>,
}

impl Org {
    pub fn new(clients: Vec<Client>) -> Self {
        Self { clients }
    }
}

pub struct Client {
    pub client_id: usize,
    pub tx: UnboundedSender<Message>,
}

pub struct TheState {
    pub orgs: Mutex<HashMap<String, Org>>,
    pub auth_token: String,
}

impl TheState {
    pub fn new(auth_token: String) -> Self {
        Self {
            orgs: Mutex::new(HashMap::new()),
            auth_token,
        }
    }
}

pub type SharedState = Arc<RwLock<TheState>>;
