mod image;
mod rectangle;

pub(crate) use image::Image;
pub(crate) use rectangle::Rectangle;

use quicksilver::graphics::Color;

pub(crate) fn color_to_hex(color : Color) -> String {
    format!(
        "#{:X}{:X}{:X}",
        (color.r * 255.).floor() as u32,
        (color.g * 255.).floor() as u32,
        (color.b * 255.).floor() as u32
    )
}