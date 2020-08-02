use super::basics::RectangleDef;
use crate::simple_drawable::{self, IntoSimpleDrawable};
use quicksilver::{geom::Rectangle};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use simple_drawable::Image;

use async_trait::async_trait;

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ImageParams {
    pub image_path: String,
    #[serde(with = "RectangleDef")]
    pub location: Rectangle,
    pub id: String,
}
#[async_trait(?Send)]
impl IntoSimpleDrawable for ImageParams {
    async fn into_simple_drawable(
        self,
        gfx: &mut quicksilver::Graphics,
    ) -> quicksilver::Result<(Box<dyn simple_drawable::SimpleDrawable>, String)> {
        let img = quicksilver::graphics::Image::load(gfx, self.image_path.clone()).await?;
        Ok((
            Box::new(Image {
                image: img,
                location: self.location,
                path: self.image_path
            }) as Box<dyn simple_drawable::SimpleDrawable>,
            self.id,
        ))
    }
}
