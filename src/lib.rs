use sdl2::{pixels, rect::Rect, video::Window, Sdl};

use crate::game_characters::renderers::Renderable;

mod game_characters;

pub mod game;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
    DownLeft,
    DownRight,
}

pub struct ScoreBoard {
    pub score: usize,
}

impl Renderable for ScoreBoard {
    fn render(&self, canvas: &mut sdl2::render::WindowCanvas) {
        canvas.set_draw_color(pixels::Color::RGB(204, 204, 0));
        canvas.fill_rect(Rect::new(10, 10, 100, 100)).unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context
            .load_font("./fonts/OpenSans-Regular.ttf", 20)
            .unwrap();
        let surface = font
            .render(&format!("Score: {}", self.score))
            .solid(pixels::Color::RGB(255, 255, 255))
            .unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        canvas
            .copy(&texture, None, Some(Rect::new(10, 10, 100, 100)))
            .unwrap();
    }
}
pub struct DrawingBoard {
    pub sdl_context: Sdl,
    pub canvas: sdl2::render::Canvas<Window>,
}

impl DrawingBoard {
    pub fn new() -> DrawingBoard {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("space_invaders", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        DrawingBoard {
            canvas,
            sdl_context,
        }
    }
}

impl Default for DrawingBoard {
    fn default() -> Self {
        Self::new()
    }
}
