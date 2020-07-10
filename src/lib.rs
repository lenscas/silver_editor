use mergui::{Context, LayerId};
use quicksilver::{graphics::Color, Graphics};
use std::{collections::HashMap, convert::TryFrom};
use stdweb::{console, js, unstable::TryInto, Array, Object, Value};

enum EventTypes {
    Color,
}
impl TryFrom<String> for EventTypes {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "color" => Ok(EventTypes::Color),
            x => {
                console!(error, "dit not get a useable event. Got : ", x);
                Err(x.to_string())
            }
        }
    }
}

impl EventTypes {
    fn into_event(self, params: Value) -> Event {
        match self {
            EventTypes::Color => Event::Color(
                params
                    .try_into()
                    .expect("color event did not have the correct parameters"),
            ),
        }
    }
}

enum Event {
    Color(String),
}

fn get_events() -> impl Iterator<Item = Event> {
    let x = js! {
        return silver_editor.get_events()
    };
    let x = match x {
        stdweb::Value::Reference(x) => x,
        _ => panic!("Did not get a reference back from silver_editor.get_events!!"),
    };
    let x: Vec<Object> = x
        .downcast::<Array>()
        .expect("couldn't downcast to array")
        .try_into()
        .expect("Dit not get an array back from silver_editor.get_events!!");
    x.into_iter().map(|v| {
        let mut x: HashMap<String, Value> = v
            .try_into()
            .unwrap_or_else(|v| panic!("failed to turn object into hashmap : {:?}", v));
        let event_name = match x.remove("event_type").expect("event has no event_type") {
            Value::String(x) => x,
            x => panic!("event_type is not a string. Got : {:?}", x),
        };
        let event = EventTypes::try_from(event_name)
            .unwrap_or_else(|v| panic!("Failed to convert event type to event {:0}", v));
        event.into_event(x.remove("params").expect("Event has no params"))
    })
}

pub struct EditorContext {
    layer: LayerId,
    color: Color,
}
impl EditorContext {
    pub fn new(context: &mut Context) -> Self {
        let layer = context.add_layer();
        let page = include_str!("../static/editor_index.html");
        js! {
            window.silver_editor.setup_extra_window_button(@{page});
        }
        Self {
            layer,
            color: Color::WHITE,
        }
    }
    pub fn update(&mut self) {
        for event in get_events() {
            match event {
                Event::Color(color) => self.color = Color::from_hex(&color),
            }
        }
    }
    pub fn draw(&self, gfx: &mut Graphics) {
        gfx.clear(self.color)
    }
    pub fn event(&mut self, event: &quicksilver::input::Event) {}
}
