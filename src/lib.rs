use mergui::{Context, LayerId};
use quicksilver::{geom::Rectangle, graphics::Color, Graphics};

mod events;
use events::{Event, EventStream};

pub struct CompiledToNativeConfig {
    pub port: u16,
    pub address: [u8; 4],
}
impl Default for CompiledToNativeConfig {
    fn default() -> Self {
        Self {
            port: 3030,
            address: [0, 0, 0, 0],
        }
    }
}
pub enum AttachButtonAt {
    Id(String),
    Element(String),
}
impl Default for AttachButtonAt {
    fn default() -> Self {
        Self::Element("body".into())
    }
}
#[derive(Default)]
pub struct CompiledToWasmConfig {
    pub button_loc: AttachButtonAt,
}

#[derive(Default)]
pub struct EditorConfig {
    pub native_config: CompiledToNativeConfig,
    pub web_config: CompiledToWasmConfig,
}

pub struct EditorContext {
    _layer: LayerId,
    color: Color,
    event_stream: EventStream,
    rectangles: Vec<(Rectangle, Color)>,
}
impl EditorContext {
    pub fn new(context: &mut Context, config: EditorConfig) -> Self {
        let layer = context.add_layer();

        Self {
            _layer: layer,
            color: Color::WHITE,
            event_stream: EventStream::new(config),
            rectangles: Vec::new(),
        }
    }
    pub fn update(&mut self) {
        for event in self.event_stream.get_events() {
            match event {
                Event::Color(color) => self.color = Color::from_hex(&color),
                Event::AddRectangle(rec) => self.rectangles.push((
                    Rectangle::new(rec.location.into(), rec.size.into()),
                    Color::from_hex(&rec.color),
                )),
            }
        }
    }
    pub fn draw(&self, gfx: &mut Graphics) {
        gfx.clear(self.color);
        for (rec, color) in &self.rectangles {
            gfx.fill_rect(rec, *color);
        }
    }
    pub fn event(&mut self, _event: &quicksilver::input::Event) {}
}
