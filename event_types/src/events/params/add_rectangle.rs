use serde::{Deserialize,Serialize};
use schemars::JsonSchema;
use quicksilver::{graphics::Color, geom::Rectangle, Graphics};
use super::basics::RectangleDef;
use crate::simple_drawable::{self, IntoSimpleDrawable};

use async_trait::async_trait;
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AddRectangle {
    pub color: String,
    #[serde(with = "RectangleDef")]
    pub rectangle: Rectangle,
    pub id: String,
}


#[async_trait(?Send)]
impl IntoSimpleDrawable for AddRectangle {
    async fn into_simple_drawable(self, _ : &mut Graphics) -> quicksilver::Result<(Box<dyn simple_drawable::SimpleDrawable>, String)> {
        Ok((
            Box::new(crate::simple_drawable::Rectangle {
                rec: self.rectangle,
                color: Color::from_hex(&self.color),
            }),
            self.id,
        ))
    }
}