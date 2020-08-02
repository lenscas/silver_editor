use mergui::{Context, LayerId};
use quicksilver::{blinds::MouseButton, geom::Vector, graphics::Color, Graphics};

use crate::{events::EventStream, EditorConfig};
use plugin_api::Event;
use plugin_api::{IntoSimpleDrawable, SimpleDrawable};
use std::{collections::HashMap};

///The type of parser functions.
///These are used to turn events received from the editor into values that the context can work with.
pub type Parser = Box<
    dyn Fn(
        &Event,
        &mut Graphics,
    ) -> Option<Box<dyn IntoSimpleDrawable>>
>;

///This renders the changes to the game made by the editor.
///It also decides what events need to be send to the editr
pub struct EditorContext {
    _layer: LayerId,
    color: Color,
    event_stream: EventStream,
    simple_shapes: Vec<(Box<dyn SimpleDrawable + 'static>, String)>,
    mouse_at: Vector,
    parsers: HashMap<String, Vec<Parser>>,
}
impl EditorContext {
    ///context refers to the Mergui context. Plan is to change it to become a bit more generic
    ///So other libraries that need their own context can also make plugins for the editor
    pub fn new(context: &mut Context, config: EditorConfig) -> Self {
        let layer = context.add_layer();

        Self {
            _layer: layer,
            color: Color::WHITE,
            event_stream: EventStream::new(config),
            simple_shapes: Vec::new(),
            mouse_at: Vector::new(0., 0.),
            parsers: Default::default(),
        }
    }
    ///Adds multiple parsers at once See add_parser for more information.
    pub fn add_parsers(&mut self, parsers: Vec<(String, Parser)>) {
        for (name, parser) in parsers {
            self.add_parser(name, parser)
        }
    }
    ///Adds an event parser to the context.
    ///
    ///Event parsers are used to change events from the editor into something the context can understand.
    ///
    ///The name is what this parser listens to.
    ///
    ///Though the context tries to deal with name conflicts it is MUCH better to be as unique as possible
    ///`{crate_name}:{event_name}` is a decent naming scheme
    pub fn add_parser(&mut self, name: String, parser: Parser) {
        let x = self.parsers.entry(name).or_insert(Vec::new());
        x.push(parser);
    }

    ///updates the context
    ///
    ///It will read all events that it got since last time and update the context accordingly
    pub async fn update(&mut self, gfx: &mut Graphics) -> quicksilver::Result<()> {
        for event in self.event_stream.get_events() {
            let parsed = match self.parsers.get(&event.parser) {
                Some(x) => {
                    let mut result = None;
                    for potential_parser in x {
                        result = match potential_parser(&event, gfx) {
                            Some(mut parsed) => Some(parsed.into_simple_drawable(gfx).await?),
                            None => continue
                        };
                        break

                    }
                    result
                }
                None => None,
            };
            if let Some(parsed) = parsed {
                match event.event_type {
                    plugin_api::EventType::Insert => self.simple_shapes.push(parsed),
                    plugin_api::EventType::Update => {
                        if let Some(old) = self
                            .simple_shapes
                            .iter_mut()
                            .find(|(_, id)| *id == parsed.1)
                        {
                            old.0 = parsed.0;
                        }
                    }
                    plugin_api::EventType::Remove => {
                        self.simple_shapes.retain(|(_, id)| *id == parsed.1)
                    }
                }
            }
        }
        Ok(())
    }
    ///Draws the aditions the editor made to the game
    pub fn draw(&mut self, gfx: &mut Graphics) -> quicksilver::Result<()> {
        gfx.clear(self.color);
        for (shape, _) in &mut self.simple_shapes {
            shape.draw(gfx)?;
        }
        Ok(())
    }
    ///handles game events. This allows you to click on shapes drawn by the context to open them in the editor
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
