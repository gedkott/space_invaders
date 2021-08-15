use crate::Direction;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub const SHOOTER_STEP_DISTANCE: i32 = 10;

pub struct Shooter {
    pub x_pos: i32,
    pub y_pos: i32,
    pub width: u32,
    pub height: u32,
    pub direction: crate::Direction,
}

impl Shooter {
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        // change the color of our drawing with a gold-color ...
        canvas.set_draw_color(Color::RGB(255, 210, 0));
        // A draw a rectangle which almost fills our window with it !
        canvas
            .fill_rect(Rect::new(self.x_pos, self.y_pos, self.width, self.height))
            .unwrap();
    }

    pub fn step(&mut self) {
        match self.direction {
            Direction::Right => {
                self.x_pos += SHOOTER_STEP_DISTANCE;
            }
            Direction::Left => {
                self.x_pos -= SHOOTER_STEP_DISTANCE;
            }
            _ => (),
        }
    }
}
