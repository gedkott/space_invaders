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

    pub fn new(canvas_width: u32) -> Vec<Shelter> {
        let mut shelters = Vec::new();

        let shelter_width = 75;
        let shelter_height = 50;
        let min_shelter_buff = 50;

        let mut no_shelters = 0;
        let mut room = canvas_width;
        while room >= shelter_width + min_shelter_buff {
            room -= shelter_width + min_shelter_buff;
            no_shelters += 1;
        }

        // if there is extra room, we need to shift the shelters down by splitting the remaining room on left and right while accounting for buffer on left and extra space on right
        let shift_amount = ((room + min_shelter_buff) / 2) as i32 - min_shelter_buff as i32;

        for i in 0..no_shelters {
            let x_pos = (i * (shelter_width + min_shelter_buff)) as i32
                + min_shelter_buff as i32
                + shift_amount;
            let shelter = Shelter {
                x_pos,
                y_pos: (400 + shelter_height as i32) + 10,
                width: shelter_width as u32,
                height: shelter_height,
                health: SHELTER_HEALTH,
            };
            shelters.push(shelter);
        }
        shelters
    }
}
