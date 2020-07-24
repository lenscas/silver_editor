use silver_editor_event_types::Event;
use serde_json::Value;



pub(crate) fn json_value_iter_to_event_iter<T: Iterator<Item = Value>>(
    iterator: T,
) -> impl Iterator<Item = Event> {
    iterator.map(|v| serde_json::from_value(v).expect("Could not deserialize event"))
}
