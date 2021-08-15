use crate::Direction;

pub const SHOOTER_STEP_DISTANCE: i32 = 10;

pub struct Shooter {
    pub x_pos: i32,
    pub y_pos: i32,
    pub width: u32,
    pub height: u32,
    pub direction: crate::Direction,
}

impl Shooter {
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
