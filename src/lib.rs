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

pub mod score_board {
    use sdl2::{pixels, rect::Rect};

    use crate::game_characters::renderers::Renderable;

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
}
