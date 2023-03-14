mod context;
use context::*;
mod external;
use external::*;

pub fn main() -> Result<(), String> {
    let (renderer, mut event_pump) = Render::new();
    let mut context = Context::new(renderer);
    let mut vehicle_id = 0;
    'running: loop {
        context.render.canvas.set_draw_color(Color::BLACK);
        context.render.canvas.clear();
        // context.render.canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: ARROW_D, ..
                } => {
                    context.c_queue.create_vehicle(Origin::North, vehicle_id);
                    vehicle_id += 1;
                }
                Event::KeyDown {
                    keycode: ARROW_R, ..
                } => {
                    context.c_queue.create_vehicle(Origin::West, vehicle_id);
                    vehicle_id += 1;
                }
                Event::KeyDown {
                    keycode: ARROW_L, ..
                } => {
                    context.c_queue.create_vehicle(Origin::East, vehicle_id);
                    vehicle_id += 1;
                }
                Event::KeyDown {
                    keycode: ARROW_U, ..
                } => {
                    context.c_queue.create_vehicle(Origin::South, vehicle_id);
                    vehicle_id += 1;
                }
                Event::KeyDown { keycode: RAND, .. } => {
                    context.c_queue.create_vehicle(Origin::random(), vehicle_id);
                    vehicle_id += 1;
                }
                Event::KeyDown { keycode: ESC, .. } => break 'running,
                _ => {}
            }
        }
        context.move_vehicles()?;

        // The rest of the game loop goes here...
        context.render.draw_grid()?;
        context.render.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    Ok(())
}
