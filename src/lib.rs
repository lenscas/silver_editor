use mergui::{Context, LayerId};
use quicksilver::{geom::Rectangle, graphics::Color, Graphics};

mod events;
use events::{Event, EventStream};

pub struct EditorContext {
    _layer: LayerId,
    color: Color,
    event_stream: EventStream,
    rectangles: Vec<(Rectangle, Color)>,
}
impl EditorContext {
    pub fn new(context: &mut Context) -> Self {
        let layer = context.add_layer();

        Self {
            _layer: layer,
            color: Color::WHITE,
            event_stream: EventStream::new(),
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
