use mergui::{Context, LayerId};
use quicksilver::{
    blinds::MouseButton,
    geom::{Rectangle, Shape, Vector},
    graphics::Color,
    Graphics,
};

use crate::{
    events::{event_params::AddRectangle, Event, EventStream, SendEvents},
    EditorConfig,
};
pub struct EditorContext {
    _layer: LayerId,
    color: Color,
    event_stream: EventStream,
    rectangles: Vec<(Rectangle, Color, String)>,
    mouse_at: Vector,
}
impl EditorContext {
    pub fn new(context: &mut Context, config: EditorConfig) -> Self {
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
                    Rectangle::new(rec.location.into(), rec.size.into()),
                    Color::from_hex(&rec.color),
                    rec.id,
                )),
                Event::EditRectangle(rec) => {
                    if let Some(x) = self.rectangles.iter_mut().find(|(_, _, id)| *id == rec.id) {
                        x.0 = Rectangle::new(rec.location.into(), rec.size.into());
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
                        println!("got into the editor");
                        self.event_stream
                            .send_event(SendEvents::EditRectangle(AddRectangle {
                                color: format!(
                                    "#{}{}{}",
                                    std::char::from_digit(color.r.floor() as u32, 16)
                                        .expect("something werd happened"),
                                    std::char::from_digit(color.g.floor() as u32, 16)
                                        .expect("something werd happened"),
                                    std::char::from_digit(color.b.floor() as u32, 16)
                                        .expect("something werd happened")
                                ),
                                location: (rec.pos.x, rec.pos.y),
                                size: (rec.size.x, rec.size.y),
                                id: (id.clone()),
                            }));
                    }
                }
            }
            _ => {}
        }
    }
}
