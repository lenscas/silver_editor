use super::{shared_event_logic::json_value_iter_to_event_iter, Event};
use stdweb::js;

fn get_events() -> impl Iterator<Item = Event> {
    let x = js! {
        return silver_editor.get_events().map(v => JSON.stringify(v));
    };
    let x = Vec::<_>::from(x.as_array().expect("Did not get an array back"));
    json_value_iter_to_event_iter(
        x.into_iter()
            .map(|v| v.into_string().expect("Event array contained a bad value"))
            .map(|v| serde_json::from_str(&v).expect("Couldn't parse event json")),
    )
}

pub(crate) fn inject_button_to_editor() {
    let content = include_str!("../../static/editor_index.html");
    js! {
        window.silver_editor.setup_extra_window_button(@{content});
    }
}
pub(crate) struct EventStream {}
impl EventStream {
    pub(crate) fn new() -> Self {
        inject_button_to_editor();
        EventStream {}
    }
    pub(crate) fn get_events(&mut self) -> impl Iterator<Item = Event> {
        get_events()
    }
}
