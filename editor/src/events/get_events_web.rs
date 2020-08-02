use super::shared_event_logic::{JS_SCRIPT, json_value_iter_to_event_iter};
use crate::{AttachButtonAt, EditorConfig};
use build_in_event_types::events::{Event};
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

pub(crate) fn inject_button_to_editor(config: EditorConfig) {
    let content = include_str!("../../static/editor_wasm.html");
    let script = JS_SCRIPT.clone();
    let (element, id) = match config.web_config.button_loc {
        AttachButtonAt::Id(x) => (None, Some(x)),
        AttachButtonAt::Element(x) => (Some(x), None),
    };
    js! {
        let script_tag = document.createElement("script");
        script_tag.type="text/javascript";
        script_tag.text = @{script};
        document.body.appendChild(script_tag);
        let el;
        if(@{element.clone()}){
            el = document.getElementsByTagName(@{element})[0]
        } else {
            el = document.getElementById(@{id})
        }
        window.silver_editor.setup_extra_window_button(@{content},el);
    }
}
pub(crate) struct EventStream {}
impl EventStream {
    pub(crate) fn new(config: EditorConfig) -> Self {
        inject_button_to_editor(config);
        EventStream {}
    }
    pub(crate) fn get_events(&mut self) -> impl Iterator<Item = Event> {
        get_events()
    }
    pub(crate) fn send_event(&mut self, event: serde_json::Value) {
        let as_str = serde_json::to_string(&event).expect("Could not serialize event");
        js! {
            window.silver_editor.send_event(JSON.parse(@{as_str}));
        }
    }
}
