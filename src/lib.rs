pub mod game_characters;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Left,
    Right,
    None,
    DownLeft,
    DownRight,
}
