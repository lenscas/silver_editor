use serde::{Deserialize,Serialize};
use schemars::JsonSchema;
use quicksilver::{graphics::Color, geom::Rectangle};
use super::basics::RectangleDef;
use crate::simple_drawable::{self, IntoSimpleDrawable};
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