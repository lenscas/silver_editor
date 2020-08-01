use quicksilver::{geom::Vector, Graphics, graphics::Color};

mod rectangle;
pub(crate) use rectangle::Rectangle;

///This trait is used to turn the events from the editor into something that can actually be drawn
///It is made public in case the editor get plugin support.
///However, it doesn't have this as of yet, so you probably don't want to implement it
///***WARNING*** CHANGES MADE TO THIS TRAIT ARE NOT SEEN AS BREAKING UNTIL PLUGIN SUPPORT EXISTS
pub trait IntoSimpleDrawable {
    ///turns the value into something that can be drawn AND its id.
    ///the id should be unique as this is used to keep the editor and the game in sync when it comes to the added shapes
    fn into_simple_drawable(self) -> (Box<dyn SimpleDrawable>, String);
}
///This trait is implemented by everything the editor can draw.
///It is made public in case the editor get plugin support
///However, it doesn't have this yet so you probably don't want to implement it
///***WARNING*** CHANGES MADE TO THIS TRAIT ARE NOT SEEN AS BREAKING UNTIL PLUGIN SUPPORT EXISTS
pub trait SimpleDrawable {
    ///draws the shape
    fn draw(&mut self, gfx : &mut Graphics)->quicksilver::Result<()>;
    ///should return true if pos is a location inside the shape
    ///This is used by the editor to know what shape the user clicked on (if any)
    fn contains(&mut self, pos : Vector)->bool;
    ///creates a json value that the game can send to the editor
    ///used to edit already existing shapes
    fn as_event(&self, id : String) -> serde_json::Result<serde_json::Value>;
}

pub(crate) fn color_to_hex(color : Color) -> String {
    format!(
        "#{:X}{:X}{:X}",
        (color.r * 255.).floor() as u32,
        (color.g * 255.).floor() as u32,
        (color.b * 255.).floor() as u32
    )
}