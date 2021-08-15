use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub trait Renderable {
    fn render(&self, canvas: &mut WindowCanvas);
}

impl Renderable for crate::game_characters::shooter::Shooter {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(255, 210, 0));
        canvas
            .fill_rect(Rect::new(self.x_pos, self.y_pos, self.width, self.height))
            .unwrap();
    }
}

impl Renderable for crate::game_characters::alien::Alien {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        canvas
            .fill_rect(Rect::new(self.x_pos, self.y_pos, self.width, self.height))
            .unwrap();
    }
}

impl Renderable for crate::game_characters::bullet::Bullet {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        canvas
            .fill_rect(Rect::new(self.x_pos, self.y_pos, self.width, self.height))
            .unwrap();
    }
}
