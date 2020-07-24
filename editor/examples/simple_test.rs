
use quicksilver::{
    geom::{ Vector},
    graphics::{Color, Graphics},
    run, Input, Result, Settings,  Window,
};
use silver_editor::EditorContext;
fn main() {
    run(
        Settings {
            size: Vector::new(800.0, 600.0),
            title: "Linear Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut inputs: Input) -> Result<()> {
    gfx.clear(Color::WHITE);
    gfx.present(&window)?;
    let mut edit_context = EditorContext::new( Default::default());
    loop {
        while let Some(e) = inputs.next_event().await {
            edit_context.event(&e);
        }
        edit_context.update();
        edit_context.draw(&mut gfx);
        gfx.present(&window)?;
    }
}
