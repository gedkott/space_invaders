const SHELTER_HEALTH: i32 = 25;

pub struct Shelter {
    pub x_pos: i32,
    pub y_pos: i32,
    pub width: u32,
    pub height: u32,
    pub health: i32,
}

pub mod shelter_group {
    use super::{Shelter, SHELTER_HEALTH};

    pub fn new() -> Vec<Shelter> {
        let mut shelters = Vec::new();
        for i in 0..7 {
            let shelter_width = 100;
            let shelter_height = 75;
            let shelter = Shelter {
                x_pos: (i * (10 + shelter_width as i32)) + 10,
                y_pos: (300 + shelter_height as i32) + 10,
                width: shelter_width,
                height: shelter_height,
                health: SHELTER_HEALTH,
            };
            shelters.push(shelter);
        }
        shelters
    }
}
