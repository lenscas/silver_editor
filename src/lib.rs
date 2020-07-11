use mergui::{Context, LayerId};
use quicksilver::{graphics::Color, Graphics};

mod events;
use events::EventStream;
use std::convert::TryFrom;

pub(crate) trait IntoEvent<T> {
    fn into_event(self, params: T) -> Event;
}
enum EventTypes {
    Color,
}
impl TryFrom<String> for EventTypes {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "color" => Ok(EventTypes::Color),
            x => {
                println!("did not get a useable event. Got : {:?}", x);
                Err(x.to_string())
            }
        }
    }
}

enum Event {
    Color(String),
}

pub struct EditorContext {
    _layer: LayerId,
    color: Color,
    event_stream: EventStream,
}
impl EditorContext {
    pub fn new(context: &mut Context) -> Self {
        let layer = context.add_layer();

        Self {
            _layer: layer,
            color: Color::WHITE,
            event_stream: EventStream::new(),
        }
    }
    pub fn update(&mut self) {
        for event in self.event_stream.get_events() {
            match event {
                Event::Color(color) => self.color = Color::from_hex(&color),
            }
        }
    }
    pub fn draw(&self, gfx: &mut Graphics) {
        gfx.clear(self.color)
    }
    pub fn event(&mut self, _event: &quicksilver::input::Event) {}
}
