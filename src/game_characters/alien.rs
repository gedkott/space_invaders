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
pub struct AlienGroup {
    pub aliens: Vec<Alien>,
}

impl AlienGroup {
    pub fn new() -> Self {
        let alien_width: u32 = 25;
        let alien_height = 25;
        let mut aliens = Vec::new();
        for i in 0..5 {
            for j in 0..10 {
                let alien = Alien {
                    x_pos: (j * (10 + alien_width as i32)) + 10,
                    y_pos: (i * (10 + alien_width as i32)) + 10,
                    width: alien_width,
                    height: alien_height,
                    direction: Direction::DownRight,
                };
                aliens.push(alien);
            }
        }
        AlienGroup { aliens }
    }
}
