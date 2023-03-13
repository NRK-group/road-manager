mod context;
use context::*;
mod external;
use external::*;

pub fn main() -> Result<(), String> {
    let (renderer, mut event_pump) = Render::new();
    let mut context = Context::new(renderer);
    'running: loop {
        context.render.canvas.set_draw_color(Color::BLACK);
        context.render.canvas.clear();
        // context.render.canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: ARROW_D, ..
                } => context.c_queue.push_vehicle(Origin::North),
                Event::KeyDown {
                    keycode: ARROW_L, ..
                } => context.c_queue.push_vehicle(Origin::West),
                Event::KeyDown {
                    keycode: ARROW_R, ..
                } => context.c_queue.push_vehicle(Origin::East),
                Event::KeyDown {
                    keycode: ARROW_U, ..
                } => context.c_queue.push_vehicle(Origin::South),
                Event::KeyDown { keycode: RAND, .. } => {
                    context.c_queue.push_vehicle(Origin::random())
                }
                Event::KeyDown { keycode: ESC, .. } => break 'running,
                _ => {}
            }
        }
        context.move_vehicles()?;
        // The rest of the game loop goes here...
        context.render.draw_grid()?;
        context.render.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
