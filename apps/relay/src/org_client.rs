use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    iter::Map,
    sync::Arc,
};

use axum::extract::ws::{Message, WebSocket};

pub struct Orgs {
    pub orgs: HashMap<String, Org>,
}

impl Orgs {
    pub fn new() -> Self {
        Self {
            orgs: HashMap::new(),
        }
    }
}

pub struct Org {
    pub clients: Vec<WebSocket>,
}

impl Org {
    pub fn new() -> Self {
        Self { clients: vec![] }
    }
}

pub fn add_client_to_org(orgs: &mut Orgs, org_id: String, client: WebSocket) {
    if let Some(org) = orgs.orgs.get_mut(&org_id) {
        org.clients.push(client);
    } else {
        let mut org = Org::new();
        org.clients.push(client);
        orgs.orgs.insert(org_id, org);
    }
}

async fn send_event_to_clients(orgs: &mut Orgs, org_name: String, event: String) {
    if let Some(mut org) = orgs.orgs.get_mut(&org_name) {
        // TODO dont block here
        for client in &mut org.clients {
            if let Err(err) = client.send(Message::Text(event.clone())).await {
                println!("Error sending message: {:?}", err);
            };
        }
    } else {
        println!("Cannot send message org not found: {:?}", org_name);
    }
}
