use crate::Direction;

pub const ALIEN_STEP_DISTANCE: i32 = 4;

#[derive(PartialEq)]
pub struct Alien {
    pub x_pos: i32,
    pub y_pos: i32,
    pub width: u32,
    pub height: u32,
    pub direction: crate::Direction,
}

impl Alien {
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
