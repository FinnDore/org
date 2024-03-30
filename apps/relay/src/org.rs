use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::Message;
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
