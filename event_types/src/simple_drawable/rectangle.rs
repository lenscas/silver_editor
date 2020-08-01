use super::{color_to_hex, SimpleDrawable};
use crate::{SendEvents::EditRectangle, AddRectangle};
use quicksilver::{geom::Shape, graphics::Color};
///A rectangle that can be drawn by the editor context
///It only exists for the editor and thus you probably don't want/need to use it.
pub struct Rectangle {
    pub(crate) rec: quicksilver::geom::Rectangle,
    pub(crate) color: Color,
}

impl SimpleDrawable for Rectangle {
    fn draw(&mut self, gfx: &mut quicksilver::Graphics) -> quicksilver::Result<()> {
        gfx.fill_rect(&self.rec, self.color);
        Ok(())
    }
    fn contains(&mut self, pos: quicksilver::geom::Vector) -> bool {
        self.rec.contains(pos)
    }
    fn as_event(&self, id: String) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(EditRectangle(AddRectangle {
            color: color_to_hex(self.color),
            rectangle: self.rec.clone(),
            id,
        }))
    }
}
