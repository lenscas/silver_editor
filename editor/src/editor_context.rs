use mergui::{Context, LayerId};
use quicksilver::{
    blinds::MouseButton,
    geom::{Rectangle, Shape, Vector},
    graphics::Color,
    Graphics,
};

use crate::{
    events::{EventStream},
    EditorConfig,
};
use silver_editor_event_types::{Event,AddRectangle,SendEvents};
pub struct EditorContext {
    _layer: LayerId,
    color: Color,
    event_stream: EventStream,
    rectangles: Vec<(Rectangle, Color, String)>,
    mouse_at: Vector,
}
impl EditorContext {
    pub fn new(context : &mut Context,config: EditorConfig) -> Self {
       let layer = context.add_layer();

        Self {
            _layer: layer,
            color: Color::WHITE,
            event_stream: EventStream::new(config),
            rectangles: Vec::new(),
            mouse_at: Vector::new(0., 0.),
        }
    }
    pub fn update(&mut self) {
        for event in self.event_stream.get_events() {
            match event {
                Event::Color(color) => self.color = Color::from_hex(&color),
                Event::AddRectangle(rec) => self.rectangles.push((
                    rec.rectangle,
                    Color::from_hex(&rec.color),
                    rec.id,
                )),
                Event::EditRectangle(rec) => {
                    if let Some(x) = self.rectangles.iter_mut().find(|(_, _, id)| *id == rec.id) {
                        x.0 = rec.rectangle.clone();
                        x.1 = Color::from_hex(&rec.color)
                    }
                }
            }
        }
    }
    pub fn draw(&self, gfx: &mut Graphics) {
        gfx.clear(self.color);
        for (rec, color, _) in &self.rectangles {
            gfx.fill_rect(rec, *color);
        }
    }
    pub fn event(&mut self, event: &quicksilver::input::Event) {
        match event {
            quicksilver::input::Event::PointerMoved(x) => {
                self.mouse_at = x.location();
            }
            quicksilver::input::Event::PointerInput(x) => {
                if x.is_down() && x.button() == MouseButton::Left {
                    let rec = self
                        .rectangles
                        .iter()
                        .find(|(rec, _, _)| rec.contains(self.mouse_at));
                    if let Some((rec, color, id)) = rec {
                        self.event_stream
                            .send_event(SendEvents::EditRectangle(AddRectangle {
                                color: format!(
                                    "#{:X}{:X}{:X}",
                                    (color.r * 255.).floor() as u32,
                                    (color.g * 255.).floor() as u32,
                                    (color.b * 255.).floor() as u32
                                ),
                                rectangle : rec.clone(),
                                id: (id.clone()),
                            }));
                    }
                }
            }
            _ => {}
        }
    }
}
