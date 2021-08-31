use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub trait Renderable {
    fn render(&self, canvas: &mut WindowCanvas);
    fn box_form(&self) -> ((i32, i32), (i32, i32));
    fn x_pos(&self) -> i32;
    fn y_pos(&self) -> i32;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
}

impl Renderable for crate::game_characters::shooter::Shooter {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(255, 210, 0));
        canvas
            .fill_rect(Rect::new(self.x_pos, self.y_pos, self.width, self.height))
            .unwrap();
    }

    fn box_form(&self) -> ((i32, i32), (i32, i32)) {
        (
            (self.x_pos as i32, self.y_pos as i32),
            (
                self.x_pos as i32 + self.width as i32,
                self.y_pos as i32 + self.height as i32,
            ),
        )
    }
}

impl Renderable for crate::game_characters::alien::Alien {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        canvas
            .fill_rect(Rect::new(self.x_pos, self.y_pos, self.width, self.height))
            .unwrap();
    }

    fn box_form(&self) -> ((i32, i32), (i32, i32)) {
        (
            (self.x_pos as i32, self.y_pos as i32),
            (
                self.x_pos as i32 + self.width as i32,
                self.y_pos as i32 + self.height as i32,
            ),
        )
    }
}

impl Renderable for crate::game_characters::bullet::Bullet {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        canvas
            .fill_rect(Rect::new(self.x_pos, self.y_pos, self.width, self.height))
            .unwrap();
    }

    fn box_form(&self) -> ((i32, i32), (i32, i32)) {
        (
            (self.x_pos as i32, self.y_pos as i32),
            (
                self.x_pos as i32 + self.width as i32,
                self.y_pos as i32 + self.height as i32,
            ),
        )
    }
}

impl Renderable for crate::game_characters::shelter::Shelter {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 102, 0));
        canvas
            .fill_rect(Rect::new(self.x_pos, self.y_pos, self.width, self.height))
            .unwrap();
    }

    fn box_form(&self) -> ((i32, i32), (i32, i32)) {
        (
            (self.x_pos as i32, self.y_pos as i32),
            (
                self.x_pos as i32 + self.width as i32,
                self.y_pos as i32 + self.height as i32,
            ),
        )
    }
}
