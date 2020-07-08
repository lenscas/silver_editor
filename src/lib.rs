use mergui::{Context, LayerId};
use stdweb::{js, unstable::TryInto};
pub fn did_click_button() -> bool {
    let x = js! {
        if(window.did_press) {
            window.did_press = false;
            return true;
        }
        return false;
    };
    x.try_into().unwrap()
}
pub fn log(x: String) {
    js! {console.log(@{x})}
}

pub struct EditorContext {
    layer: LayerId,
}
impl EditorContext {
    pub fn new(context: &mut Context) -> Self {
        let layer = context.add_layer();
        let page = include_str!("../static/editor_index.html");
        js! {
            window.silver_editor.setup_extra_window_button(@{page});
        }
        Self { layer }
    }
    pub fn update() {}
    pub fn draw() {}
    pub fn event(&mut self, event: &quicksilver::input::Event) {}
}
