use async_trait::async_trait;
use quicksilver::{geom::Vector, Graphics};
use serde::{Deserialize,Serialize};



///This trait is used to turn the events from the editor into something that can actually be drawn.
#[async_trait(?Send)]

pub trait IntoSimpleDrawable {
    ///turns the value into something that can be drawn AND its id.
    ///the id should be unique as this is used to keep the editor and the game in sync when it comes to the added shapes
    ///if you decide to use integers as id's please use the `crate_name:{numeric_id}` format to avoid conflicts.
    async fn into_simple_drawable(&mut self, gfx : &mut Graphics) -> quicksilver::Result<(Box<dyn SimpleDrawable>, String)>;
}
///This trait is implemented by everything the editor can draw.
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

#[derive(Serialize,Deserialize,Debug)]
pub struct Event {
    pub event_type : EventType,
    pub parser : String,
    pub data: serde_json::Value
}
#[derive(Serialize,Deserialize,Debug)]
pub enum EventType {
    Insert, Update,Remove
}