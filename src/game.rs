use std::{collections::HashSet, thread::sleep, time::Duration};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, EventPump};

use crate::{
    game_characters::{
        alien::AlienGroup, alien::ALIEN_STEP_DISTANCE, bullet::Bullet,
        bullet::BULLET_STEP_DISTANCE, renderers::Renderable, shooter::Shooter,
        shooter::SHOOTER_STEP_DISTANCE,
    },
    Direction, DrawingBoard, ScoreBoard,
};

const SCORE_INCREMENT: usize = 10;

pub struct Game {
    drawing_board: DrawingBoard,
    shooter: Shooter,
    alien_group: AlienGroup,
    bullets: Vec<Bullet>,
    score_board: ScoreBoard,
    event_pump: EventPump,
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let drawing_board = DrawingBoard::new();
        let canvas = &drawing_board.canvas;

        let shooter = Shooter::new(
            canvas.viewport().width() as i32,
            canvas.viewport().height() as i32,
        );

        let active_bullets: Vec<Bullet> = Vec::new();

        let alien_group = AlienGroup::new();

        let score_board = ScoreBoard { score: 0 };

        let event_pump = drawing_board.sdl_context.event_pump().unwrap();

        Game {
            drawing_board,
            shooter,
            alien_group,
            bullets: active_bullets,
            score_board,
            event_pump,
        }
    }

    fn detect_collisions(&mut self) {
        // remove bullets and aliens that will/have collided
        // Did anything collide?
        // all locations on the bottom of any alien will have a y coordinate located at alien.y_pos + alien.height
        // if a bullet reaches that y and also on any x coordinate in alien.x_pos up until alien.x_pos + alien.width
        // is this O(n^2)?
        let aliens = &mut self.alien_group.aliens;

        let score_board = &mut self.score_board;

        self.bullets.retain(|bullet| {
            let mut is_destroyed = false;
            aliens.retain(|alien| {
                if bullet.y_pos <= alien.y_pos + alien.height as i32
                    && bullet.y_pos >= alien.y_pos as i32
                {
                    if bullet.x_pos >= alien.x_pos
                        && bullet.x_pos <= alien.x_pos + alien.width as i32
                    {
                        score_board.score += SCORE_INCREMENT;

                        is_destroyed = true;
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            });
            !is_destroyed
        });

        // remove bullets that have/will reached the top
        self.bullets
            .retain(|bullet| bullet.y_pos - BULLET_STEP_DISTANCE >= 0);
    }

    fn process_key_presses(&mut self) {
        let shooter = &mut self.shooter;
        let canvas = &self.drawing_board.canvas;
        let pressed_keys: HashSet<Keycode> = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        if pressed_keys.contains(&Keycode::Right) {
            shooter.direction = Direction::Right;

            // step the shooter
            if shooter.x_pos + SHOOTER_STEP_DISTANCE + shooter.width as i32
                <= canvas.viewport().width() as i32
            {
                shooter.step();
            }
        }

        if pressed_keys.contains(&Keycode::Left) {
            shooter.direction = Direction::Left;

            // step the shooter
            if shooter.x_pos - SHOOTER_STEP_DISTANCE >= 0 {
                shooter.step();
            }
        }

        if pressed_keys.contains(&Keycode::Space) {
            let bullet = Bullet {
                x_pos: shooter.x_pos + (shooter.width as i32 / 2),
                y_pos: shooter.y_pos,
                width: 2,
                height: 10,
                direction: Direction::Up,
            };
            self.bullets.push(bullet);
        }
    }

    fn draw_screen(&mut self, i: u8) {
        let mut canvas = &mut self.drawing_board.canvas;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        self.shooter.render(&mut canvas);

        for bullet in self.bullets.iter_mut() {
            bullet.render(&mut canvas);
        }

        for alien in self.alien_group.aliens.iter_mut() {
            alien.render(&mut canvas);
        }

        self.score_board.render(&mut canvas)
    }

    fn manage_canvas_boundaries(&mut self) {
        // shift all aliens down and switch directions if any of them touched a side
        if self.alien_group.aliens.iter().any(|alien| {
            alien.direction == Direction::DownLeft && alien.x_pos - ALIEN_STEP_DISTANCE <= 0
                || alien.direction == Direction::DownRight
                    && alien.x_pos + alien.width as i32 + ALIEN_STEP_DISTANCE
                        >= self.drawing_board.canvas.viewport().width() as i32
        }) {
            for alien in &mut self.alien_group.aliens {
                // switch direction
                alien.direction = if alien.direction == Direction::DownLeft {
                    Direction::DownRight
                } else {
                    Direction::DownLeft
                };

                // shift down
                const ALIEN_VERTICAL_STEP_DISTANCE: i32 = 9;
                alien.y_pos += ALIEN_VERTICAL_STEP_DISTANCE as i32;
            }
        }
    }

    pub fn run(mut self) {
        let mut i = 0;

        'running: loop {
            i = (i + 1) % 255;

            // watch out for events; specifically if the quit button (ESC) was pressed
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            self.process_key_presses();

            self.detect_collisions();

            self.manage_canvas_boundaries();

            // step remaining bullets
            for bullet in self.bullets.iter_mut() {
                bullet.step();
            }

            // step aliens
            for alien in self.alien_group.aliens.iter_mut() {
                alien.step();
            }

            // draw
            self.draw_screen(i);
            self.drawing_board.canvas.present();

            // sleep?
            sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}