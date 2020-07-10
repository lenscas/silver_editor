use crate::{Event, EventTypes, IntoEvent};
use std::{collections::HashMap, convert::TryFrom};
use stdweb::{console, js, unstable::TryInto, Array, Object, Value};

impl IntoEvent<Value> for EventTypes {
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

pub(crate) fn inject_button_to_editor() {
    let content = include_str!("../static/editor_index.html");
    js! {
        window.silver_editor.setup_extra_window_button(@{content});
    }
}
pub(crate) struct EventStream {}
impl EventStream {
    pub(crate) fn new() -> Self {
        EventStream {}
    }
    pub(crate) fn get_events(&mut self) -> impl Iterator<Item = Event> {
        get_events()
    }
}
