use std::{collections::HashMap};

use axum::extract::ws::{WebSocket};
use tokio::sync::{Mutex};

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
        Self { clients }
    }
}
