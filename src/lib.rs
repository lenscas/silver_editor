use mergui::{Context, LayerId};
use quicksilver::{graphics::Color, Graphics};

mod events;
use events::{Event, EventStream};

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
