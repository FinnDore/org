use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{ws::Message, State},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::{mpsc::UnboundedSender, Mutex, RwLock};
use tracing::{info, instrument};

use crate::{SharedState, TheState};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scene {
    pub name: String,
    pub items: Vec<SceneItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SceneItem {
    pub mesh_type: MeshType,
    pub id: String,
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MeshType {
    Cube,
    Sphere,
    Cylinder,
    Plane,
}

pub fn create_test_scene() -> Scene {
    Scene {
        name: "test scene".to_string(),
        items: vec![
            SceneItem {
                id: "0".into(),
                mesh_type: MeshType::Cube,
                position: (0.0, 0.0, 0.0),
                rotation: (0.0, 0.0, 0.0),
            },
            // SceneItem {
            //     id: "1".into(),
            //     mesh_type: MeshType::Cube,
            //     position: (3.0, 1.0, 1.0),
            //     rotation: (1.0, 1.0, 1.0),
            // },
            // SceneItem {
            //     id: "2".into(),
            //     mesh_type: MeshType::Cylinder,
            //     position: (-2.0, -2.0, 1.0),
            //     rotation: (1.0, 1.0, 1.0),
            // },
        ],
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SceneUpdate {
    pub object_id: String,
    pub path: String,
    pub value: (f32, f32, f32),
}

pub async fn get_scene(State(state): State<SharedState>) -> Response {
    info!("Getting scene");
    let scene = state.read().await.scene.read().await.clone();
    Json(scene).into_response()
}
