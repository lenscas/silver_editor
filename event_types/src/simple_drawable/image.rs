use super::SimpleDrawable;
use crate::events::{params::ImageParams, SendEvents};
use quicksilver::geom::{Rectangle, Shape};

pub(crate) struct Image {
    pub(crate) image: quicksilver::graphics::Image,
    pub(crate) path : String,
    pub(crate) location: Rectangle,
}
impl SimpleDrawable for Image {
    fn draw(&mut self, gfx: &mut quicksilver::Graphics) -> quicksilver::Result<()> {
        gfx.draw_image(&self.image, self.location);
        Ok(())
    }
    fn contains(&mut self, pos: quicksilver::geom::Vector) -> bool {
        self.location.contains(pos)
    }
    fn as_event(&self, id: String) -> serde_json::Result<serde_json::Value> {
        Ok(serde_json::to_value(SendEvents::EditImage(ImageParams {
            image_path: self.path.clone(),
            location: self.location,
            id,
        }))?)
    }
}
