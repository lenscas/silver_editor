use mergui::{Context, LayerId};
use quicksilver::{
    blinds::MouseButton,
    geom::{Vector},
    graphics::Color,
    Graphics,
};

use crate::{events::EventStream, EditorConfig};
use silver_editor_event_types::{
    simple_drawable::{IntoSimpleDrawable, SimpleDrawable},
    Event,
};
pub struct EditorContext {
    _layer: LayerId,
    color: Color,
    event_stream: EventStream,
    simple_shapes: Vec<(Box<dyn SimpleDrawable + 'static>, String)>,
    mouse_at: Vector,
}
impl EditorContext {
    pub fn new(context: &mut Context, config: EditorConfig) -> Self {
        let layer = context.add_layer();

        Self {
            _layer: layer,
            color: Color::WHITE,
            event_stream: EventStream::new(config),
            simple_shapes: Vec::new(),
            mouse_at: Vector::new(0., 0.),
        }
    }
    pub fn update(&mut self) {
        for event in self.event_stream.get_events() {
            match event {
                Event::Color(color) => self.color = Color::from_hex(&color),
                Event::AddRectangle(rec) => self.simple_shapes.push(rec.into_simple_drawable()),
                Event::EditRectangle(rec) => {
                    if let Some(x) = self.simple_shapes.iter_mut().find(|(_, id)| *id == rec.id) {
                        *x = rec.into_simple_drawable()
                    }
                }
            }
        }
    }
    pub fn draw(&mut self, gfx: &mut Graphics) -> quicksilver::Result<()> {
        gfx.clear(self.color);
        for (shape, _) in &mut self.simple_shapes {
            shape.draw(gfx)?;
        }
        Ok(())
    }
    pub fn event(&mut self, event: &quicksilver::input::Event) {
        match event {
            quicksilver::input::Event::PointerMoved(x) => {
                self.mouse_at = x.location();
            }
            quicksilver::input::Event::PointerInput(x) => {
                if x.is_down() && x.button() == MouseButton::Left {
                    let mouse_at = self.mouse_at;
                    let rec = self
                        .simple_shapes
                        .iter_mut()
                        .map(|(shape, id)| (shape.contains(mouse_at), shape, id))
                        .find(|(is_inside, _, _)| *is_inside)
                        .map(|(_, shape, id)| (shape, id));
                    if let Some((shape, id)) = rec {
                        self.event_stream.send_event(
                            shape
                                .as_event(id.clone())
                                .expect("could not turn shape into event"),
                        )
                    }
                }
            }
            _ => {}
        }
    }
}
