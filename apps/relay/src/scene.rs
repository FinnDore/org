use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use tracing::info;

use crate::{
    color::{self, Color},
    SharedState,
};

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
    pub color: Color,
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
                position: (3.0, 0.0, 0.0),
                rotation: (1.0, 0.0, 1.0),
                color: color::Color::from_hex("#af00F0").unwrap(),
            },
            SceneItem {
                id: "1".into(),
                mesh_type: MeshType::Cube,
                position: (0.0, 0.0, 0.0),
                rotation: (-1.0, 0.0, -1.0),
                color: color::Color::from_hex("#0FFF00").unwrap(),
            },
            SceneItem {
                id: "2".into(),
                mesh_type: MeshType::Cube,
                position: (-3.0, 0.0, 0.0),
                rotation: (1.0, 1.0, 1.0),
                color: color::Color::from_hex("#00f0f0").unwrap(),
            },
        ],
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SceneUpdateType {
    Color(Color),
    Rotation(Vec<f32>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SceneUpdate {
    pub id: String,
    pub rotation: Option<(f32, f32, f32)>,
    pub position: Option<(f32, f32, f32)>,
    pub color: Option<Color>,
}

pub async fn get_scene(State(state): State<SharedState>) -> Response {
    info!("Getting scene");
    let scene = state.read().await.scene.read().await.clone();
    Json(scene).into_response()
}
