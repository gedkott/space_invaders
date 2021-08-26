use crate::Direction;

pub const BULLET_STEP_DISTANCE: i32 = 10;

#[derive(PartialEq)]
pub struct Bullet {
    pub x_pos: i32,
    pub y_pos: i32,
    pub width: u32,
    pub height: u32,
    pub direction: crate::Direction,
}

impl Bullet {
    pub fn step(&mut self) {
        if self.direction == Direction::Up {
            self.y_pos -= BULLET_STEP_DISTANCE;
        }
    }
}
