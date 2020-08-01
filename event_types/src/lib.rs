use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use simple_drawable::IntoSimpleDrawable;

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

impl IntoSimpleDrawable for AddRectangle {
    fn into_simple_drawable(self) -> (Box<dyn simple_drawable::SimpleDrawable>, String) {
        (
            Box::new(crate::simple_drawable::Rectangle {
                rec: self.rectangle,
                color: Color::from_hex(&self.color),
            }),
            self.id,
        )
    }
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

pub mod simple_drawable;
