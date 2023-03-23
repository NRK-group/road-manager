use crate::vehicle::*;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::{rect::Rect, video::Window};
pub struct Render {
    pub canvas: Canvas<Window>,
    pub side: u32,
    pub v_width: u32,
    pub v_length: u32,
}

impl Render {
    pub fn new() -> (Self, EventPump) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("rust-sdl2 demo: Video", 600, 600)
            .position_centered()
            .build()
            .unwrap();
        (
            Self {
                canvas: window.into_canvas().build().unwrap(),
                side: 650,
                v_width: 20,
                v_length: 40,
            },
            sdl_context.event_pump().unwrap(),
        )
    }
    pub fn draw_vehicle(
        &mut self,
        vehicle: &Vehicle,
        vehicle_type: VehicleType,
    ) -> Result<(), String> {
        let Point(x, y) = vehicle.point;
        self.canvas.set_draw_color(Color::RED);
        let origin = &vehicle.origin;
        let texture_creator = self.canvas.texture_creator();
        match vehicle_type {
            VehicleType::Horizontal => {
                let file = if origin == &Origin::East {
                    "./src/assets/car_right.png"
                } else {
                    "./src/assets/car_left.png"
                };
                let texture = texture_creator.load_texture(file)?;
                let dst_rect = Rect::new(x, y, self.v_length, self.v_width);
                self.canvas.copy(&texture, None, dst_rect)?;
                // self.canvas
                //     .fill_rect(Rect::new(x, y, self.v_length, self.v_width))?;
            }
            VehicleType::Verticle => {
                let file = if origin == &Origin::North {
                    "./src/assets/car_down.png"
                } else {
                    "./src/assets/car_up.png"
                };
                let texture = texture_creator.load_texture(file)?;
                let dst_rect = Rect::new(x, y, self.v_width, self.v_length);
                self.canvas.copy(&texture, None, dst_rect)?;
                // self.canvas
                //     .fill_rect(Rect::new(x, y, self.v_width, self.v_length))?;
            }
        };

        Ok(())
    }

    pub fn draw_grid(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::WHITE);
        let lines = [
            (0, 180, 650, 1),
            (0, 220, 650, 1),
            (0, 260, 650, 1),
            (0, 300, 650, 1),
            (0, 340, 650, 1),
            (0, 380, 650, 1),
            (0, 420, 650, 1),
            (180, 0, 1, 650),
            (220, 0, 1, 650),
            (260, 0, 1, 650),
            (300, 0, 1, 650),
            (340, 0, 1, 650),
            (380, 0, 1, 650),
            (420, 0, 1, 650),
        ];
        for (x, y, width, height) in lines {
            self.canvas.fill_rect(Rect::new(x, y, width, height))?;
        }
        Ok(())
    }
}
