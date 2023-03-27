mod context;
use context::*;
mod external;
use external::*;

pub fn main() -> Result<(), String> {
    let (renderer, mut event_pump) = Render::new();
    let mut context = Context::new(renderer);
    let mut vehicle_id = 1;
    let texture_creator = context.render.canvas.texture_creator();
    let texture = texture_creator.load_texture("./src/assets/road.png")?;

    'running: loop {
        context.render.canvas.set_draw_color(Color::BLACK);
        context.render.canvas.clear();
        context.render.canvas.copy(&texture, None, None)?;
        // context.render.canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
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
                Event::KeyDown { keycode: ESC, .. } => {
                    show_simple_message_box(
                        MessageBoxFlag::empty(),
                        "Stats",
                        &context.stats.format_stats(),
                        context.render.canvas.window(),
                    )
                    .map_err(|e| e.to_string())?;
                    break 'running;
                }

                _ => {}
            }
        }
        context.move_vehicles()?;
        // context.shift_vehicle_from_bq_to_cq();
        context.remove_vehicles();
        context.speed_up_fastest();
        // The rest of the game loop goes here...
        // context.render.draw_grid()?;
        context.render.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }
    Ok(())
}
