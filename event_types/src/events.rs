pub(crate)mod params;
use serde::{Deserialize,Serialize};
use params::{ImageParams, AddRectangle};
use schemars::JsonSchema;
#[derive(Deserialize, Serialize, JsonSchema)]
#[schemars(deny_unknown_fields)]
///Every event that the editor sends to the game
pub enum Event {
    Color(String),
    AddRectangle(AddRectangle),
    EditRectangle(AddRectangle),
    AddImage(ImageParams),
    EditImage(ImageParams)
}
#[derive(Deserialize, Serialize, JsonSchema)]
///Every event that the game sends to the editor
#[serde(deny_unknown_fields)]
pub enum SendEvents {
    EditRectangle(AddRectangle),
    EditImage(ImageParams)
}
