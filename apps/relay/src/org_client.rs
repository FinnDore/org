use std::{borrow::BorrowMut, collections::HashMap};

use axum::extract::ws::{Message, WebSocket};
use tokio::sync::{Mutex, RwLock};

pub struct Orgs {
    pub orgs: Mutex<HashMap<String, Org>>,
}

impl Orgs {
    pub fn new() -> Self {
        Self {
            orgs: Mutex::new(HashMap::new()),
        }
    }
}

pub struct Org {
    pub clients: Vec<WebSocket>,
}

impl Org {
    pub fn new(clients: Vec<WebSocket>) -> Self {
        Self { clients: clients }
    }
}

pub async fn add_client_to_org(mut orgs: &Orgs, org_id: String, client: WebSocket) {}

async fn send_event_to_clients(orgs: &mut Orgs, org_name: String, event: String) {}
