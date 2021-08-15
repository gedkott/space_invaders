use crate::Direction;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

const ALIEN_STEP_DISTANCE: f64 = 10f64 * 0.1;

#[derive(PartialEq)]
pub struct Alien {
    pub x_pos: i32,
    pub y_pos: i32,
    pub width: u32,
    pub height: u32,
    pub direction: crate::Direction,
}

impl Alien {
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        // change the color of our drawing with a white-color ...
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        canvas
            .fill_rect(Rect::new(self.x_pos, self.y_pos, self.width, self.height))
            .unwrap();
    }

    pub fn step(&mut self) {
        match self.direction {
            Direction::DownLeft => {
                self.x_pos -= ALIEN_STEP_DISTANCE as i32;
            }
            Direction::DownRight => {
                self.x_pos += ALIEN_STEP_DISTANCE as i32;
            }
            _ => (),
        }
    }
}
