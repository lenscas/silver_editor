use crate::{Event, EventTypes, IntoEvent};
use serde_json::Value;
use std::convert::TryInto;

impl IntoEvent<Value> for EventTypes {
    fn into_event(self, params: Value) -> Event {
        match self {
            EventTypes::Color => Event::Color(match params {
                Value::String(x) => x,
                x => panic!(
                    "color event did not have the correct parameters, got : {:?}",
                    x
                ),
            }),
        }
    }
}
pub(crate) fn json_value_iter_to_event_iter<T: Iterator<Item = Value>>(
    iterator: T,
) -> impl Iterator<Item = Event> {
    iterator
        .map(|v| match v {
            Value::Object(x) => x,
            x => panic!("Event is not an object, got : {:?}", x),
        })
        .map(|mut x| {
            let event = x
                .remove("event_type")
                .expect("Could not get event type. Error");
            let event = match event {
                Value::String(x) => x,
                x => panic!("Event type is not a string. got : {:?}", x),
            };
            let event: EventTypes = event.try_into().expect("could not parse event type");

            event.into_event(x.remove("params").expect("Could not get params from event"))
        })
}
