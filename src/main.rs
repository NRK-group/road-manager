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
                    context.add_vehicle_to_queue(Origin::North, vehicle_id);
                    vehicle_id += 1;
                }
                Event::KeyDown {
                    keycode: ARROW_R, ..
                } => {
                    context.add_vehicle_to_queue(Origin::West, vehicle_id);
                    vehicle_id += 1;
                }
                Event::KeyDown {
                    keycode: ARROW_L, ..
                } => {
                    context.add_vehicle_to_queue(Origin::East, vehicle_id);
                    vehicle_id += 1;
                }
                Event::KeyDown {
                    keycode: ARROW_U, ..
                } => {
                    context.add_vehicle_to_queue(Origin::South, vehicle_id);
                    vehicle_id += 1;
                }
                Event::KeyDown { keycode: RAND, .. } => {
                    context.add_vehicle_to_queue(Origin::random(), vehicle_id);
                    vehicle_id += 1;
                }
                Event::KeyDown { keycode: ESC, .. } => break 'running,
                _ => {}
            }
        }
        context.move_vehicles()?;
        context.shift_vehicle_from_bq_to_cq();
        context.remove_vehicles();
        // The rest of the game loop goes here...
        context.render.draw_grid()?;
        context.render.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    Ok(())
}
