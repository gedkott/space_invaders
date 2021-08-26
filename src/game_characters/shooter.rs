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
    pub fn new(canvas_width: i32, canvas_height: i32) -> Self {
        let shooter_width = 50;
        let shooter_height = 25;
        Shooter {
            x_pos: (canvas_width / 2) as i32 - (shooter_width / 2) as i32,
            y_pos: canvas_height as i32 - (shooter_height as i32) - 10,
            width: shooter_width,
            height: shooter_height,
            direction: Direction::None,
        }
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
