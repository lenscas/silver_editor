//This code has been taken from https://github.com/lenscas/silver_animation/blob/master/examples/01_simple_linear.rs
//it only servers as a quick way to test the editor features.

use mergui::Context;
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Color, Graphics, Image},
    run, Input, Result, Settings, Timer, Window,
};
use silver_animation::LinearConfig;
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
    let image = Image::load(&gfx, "img1.png").await?;
    let timing = Timer::time_per_second(30.);
    let step_size: f32 = 5.;
    let amount_of_steps = (360. / step_size).ceil() as usize;
    let mut animation = LinearConfig {
        begin_state: image,
        timing,
        draw: |state, tick, gfx, location| {
            let origin = location.center();
            let rotation = step_size * tick as f32;
            gfx.set_transform(
                Transform::translate(origin)
                    * Transform::rotate(rotation)
                    * Transform::translate(-origin),
            );
            gfx.draw_image(state, location);
            gfx.set_transform(Transform::IDENTITY);
            Ok(())
        },
        max_frames: |_| amount_of_steps,
    }
    .into_animation();

    let location = Rectangle::new(Vector::new(200., 200.), Vector::new(200., 200.));
    gfx.clear(Color::WHITE);
    gfx.present(&window)?;
    let mut context = Context::new();
    let mut edit_context = EditorContext::new(&mut context, Default::default());
    loop {
        while inputs.next_event().await.is_some() {}
        edit_context.update();
        edit_context.draw(&mut gfx);
        animation.draw(&mut gfx, location)?;
        gfx.present(&window)?;
    }
}
