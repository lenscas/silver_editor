use quicksilver::geom::{Rectangle, Vector};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(remote = "Vector", deny_unknown_fields)]
struct VectorDef {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(remote = "Rectangle", deny_unknown_fields)]
struct RectangleDef {
    #[serde(with = "VectorDef")]
    pos: Vector,
    #[serde(with = "VectorDef")]
    size: Vector,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AddRectangle {
    pub color: String,
    #[serde(with = "RectangleDef")]
    pub rectangle: Rectangle,
    pub id: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
///Every event that the editor sends to the game
pub enum Event {
    Color(String),
    AddRectangle(AddRectangle),
    EditRectangle(AddRectangle),
}
#[derive(Deserialize, Serialize, JsonSchema)]
///Every event that the game sends to the editor
#[serde(deny_unknown_fields)]
pub enum SendEvents {
    EditRectangle(AddRectangle),
}
