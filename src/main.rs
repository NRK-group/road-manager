mod context;
use context::*;
mod external;
use external::*;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 600, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
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

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
