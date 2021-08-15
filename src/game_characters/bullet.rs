use crate::Direction;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

const STEP_DISTANCE: i32 = 10;

#[derive(PartialEq)]
pub struct Bullet {
    pub x_pos: i32,
    pub y_pos: i32,
    pub width: u32,
    pub height: u32,
    pub direction: crate::Direction,
}

impl Bullet {
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        // change the color of our drawing with a white-color ...
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        canvas
            .fill_rect(Rect::new(self.x_pos, self.y_pos, self.width, self.height))
            .unwrap();
    }

    pub fn step(&mut self) {
        if self.direction == Direction::Up {
            self.y_pos -= STEP_DISTANCE;
        }
    }
}
