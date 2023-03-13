mod context;
use context::*;
mod external;
use external::*;

pub fn main() -> Result<(), String> {
    let (renderer, mut event_pump) = Render::new();
    let mut context = Context::new(renderer);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: ARROW_D, ..
                } => {}
                Event::KeyDown {
                    keycode: ARROW_L, ..
                } => {}
                Event::KeyDown {
                    keycode: ARROW_R, ..
                } => {}
                Event::KeyDown {
                    keycode: ARROW_U, ..
                } => {}
                Event::KeyDown { keycode: RAND, .. } => {}
                Event::KeyDown { keycode: ESC, .. } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        context.render.draw_grid()?;
        context.render.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
