use quicksilver::geom::{Vector, Rectangle};
use serde::{Deserialize, Serialize};
use schemars::{JsonSchema};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(remote = "Vector")]
struct VectorDef {
    x : f32,
    y : f32
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(remote = "Rectangle")]
struct RectangleDef {
    #[serde(with = "VectorDef")]
    pos : Vector,
    #[serde(with = "VectorDef")]
    size : Vector
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AddRectangle {
    pub color: String,
    #[serde(with = "RectangleDef")]
    pub rectangle : Rectangle,
    pub id: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
///Every event that the editor sends to the game
pub enum Event {
    Color(String),
    AddRectangle(AddRectangle),
    EditRectangle(AddRectangle),
}
#[derive(Deserialize, Serialize, JsonSchema)]
///Every event that the game sends to the editor
pub enum SendEvents {
    EditRectangle(AddRectangle),
}