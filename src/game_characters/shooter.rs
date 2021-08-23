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
    pub fn new() -> Self {
        let shooter = Shooter {
            x_pos: 0,
            y_pos: 0,
            width: 0,
            height: 0,
            direction: Direction::None,
        };
        shooter
    }

    pub fn starting_x_at<'a>(&'a mut self, x: i32) -> &'a mut Self {
        self.x_pos = x;
        self
    }

    pub fn starting_y_at<'a>(&'a mut self, y: i32) -> &'a mut Self {
        self.y_pos = y;
        self
    }

    pub fn with_width<'a>(&'a mut self, width: u32) -> &'a mut Self {
        self.width = width;
        self
    }

    pub fn with_height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
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

