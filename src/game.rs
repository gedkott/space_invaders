use std::{collections::HashSet, thread::sleep, time::Duration};

use rand::Rng;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, EventPump};

use crate::{
    game_characters::{
        alien::AlienGroup, alien::ALIEN_STEP_DISTANCE, bullet::Bullet,
        bullet::BULLET_STEP_DISTANCE, renderers::Renderable, shelter::ShelterGroup,
        shooter::Shooter, shooter::SHOOTER_STEP_DISTANCE,
    },
    DeadScreen, Direction, DrawingBoard, ScoreBoard,
};

const SCORE_INCREMENT: usize = 10;
pub struct Game {
    drawing_board: DrawingBoard,
    shooter: Shooter,
    alien_group: AlienGroup,
    shooter_bullets: Vec<Bullet>,
    alien_bullets: Vec<Bullet>,
    score_board: ScoreBoard,
    event_pump: EventPump,
    shelter_group: ShelterGroup,
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

        let score_board = ScoreBoard {
            score: 0,
            remaining_health: 0,
        };

        let event_pump = drawing_board.sdl_context.event_pump().unwrap();

        let shelters = ShelterGroup::new();

        Game {
            drawing_board,
            shooter,
            alien_group,
            shooter_bullets: active_bullets,
            alien_bullets: vec![],
            score_board,
            event_pump,
            shelter_group: shelters,
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
        let shooter = &self.shooter;

        self.shooter_bullets.retain(|bullet| {
            let mut is_destroyed = false;
            aliens.retain(|alien| {
                let alien_x = alien.x_pos;
                let alien_y = alien.y_pos;
                let alien_box = (
                    (alien_x, alien_y),
                    (
                        alien_x + shooter.width as i32,
                        alien_y + shooter.height as i32,
                    ),
                );
                let bullet_box = (
                    (bullet.x_pos, bullet.y_pos),
                    (
                        bullet.x_pos + bullet.width as i32,
                        bullet.y_pos + bullet.height as i32,
                    ),
                );

                if overlap(alien_box, bullet_box) {
                    score_board.score += SCORE_INCREMENT;
                    is_destroyed = true;
                    false
                } else {
                    true
                }
            });
            !is_destroyed
        });

        let shooter = &self.shooter;
        let mut shooter_hit_no = 0;
        let shooter_x = shooter.x_pos;
        let shooter_y = shooter.y_pos;
        let shooter_box = (
            (shooter_x, shooter_y),
            (
                shooter_x + shooter.width as i32,
                shooter_y + shooter.height as i32,
            ),
        );
        self.alien_bullets.retain(|b| {
            let bullet_box = (
                (b.x_pos, b.y_pos),
                (b.x_pos + b.width as i32, b.y_pos + b.height as i32),
            );

            if overlap(shooter_box, bullet_box) {
                shooter_hit_no += 1;
                false
            } else {
                true
            }
        });

        self.shooter.health -= shooter_hit_no;
        score_board.remaining_health = self.shooter.health;

        let alien_bs = &mut self.alien_bullets;
        let shooter_bs = &mut self.shooter_bullets;
        for shelter in &mut self.shelter_group.shelters {
            let mut shelter_hit_times = 0;

            let shelter_box = (
                (shelter.x_pos, shelter.y_pos),
                (
                    shelter.x_pos + shelter.width as i32,
                    shelter.y_pos + shelter.height as i32,
                ),
            );

            let f = |b: &Bullet| -> bool {
                let bullet_box = (
                    (b.x_pos, b.y_pos),
                    (b.x_pos + b.width as i32, b.y_pos + b.height as i32),
                );
                !overlap(shelter_box, bullet_box)
            };
            alien_bs.retain(|b| {
                let retain = f(b);
                if !retain {
                    shelter_hit_times += 1;
                } else {
                }
                retain
            });
            shooter_bs.retain(|b| {
                let retain = f(b);
                if !retain {
                    shelter_hit_times += 1;
                } else {
                }
                retain
            });

            shelter.health -= shelter_hit_times;
        }

        self.shelter_group.shelters.retain(|s| s.health > 0);
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
            self.shooter_bullets.push(bullet);
        }
    }

    fn draw_screen(&mut self, i: u8) {
        let mut canvas = &mut self.drawing_board.canvas;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        self.shooter.render(&mut canvas);

        for bullet in self.shooter_bullets.iter_mut() {
            bullet.render(&mut canvas);
        }

        for bullet in self.alien_bullets.iter_mut() {
            bullet.render(&mut canvas);
        }

        for alien in self.alien_group.aliens.iter_mut() {
            alien.render(&mut canvas);
        }

        for shelter in &self.shelter_group.shelters {
            shelter.render(&mut canvas);
        }

        self.score_board.render(&mut canvas);

        if self.shooter.health <= 0 {
            // draw the "you dead" screen over everything; not a fan
            let dead_screen = DeadScreen;
            dead_screen.render(&mut canvas);
        }
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
                const ALIEN_VERTICAL_STEP_DISTANCE: i32 = 18;
                alien.y_pos += ALIEN_VERTICAL_STEP_DISTANCE as i32;
            }
        }

        // remove bullets that have/will reached the top
        self.shooter_bullets
            .retain(|bullet| bullet.y_pos - BULLET_STEP_DISTANCE >= 0);

        // remove bullets that have/will reached the bottom
        let viewport_height = self.drawing_board.canvas.viewport().height();
        self.alien_bullets
            .retain(|bullet| bullet.y_pos + BULLET_STEP_DISTANCE <= viewport_height as i32);
    }

    fn process_alien_shots(&mut self) {
        let aliens = &self.alien_group.aliens;
        let no_aliens = aliens.len();
        if no_aliens != 0 {
            let will_shoot = rand::thread_rng().gen_range(0..4) > 2;
            if will_shoot {
                let rand = rand::thread_rng().gen_range(0..no_aliens);
                let shooting_alien = aliens.get(rand).unwrap();
                let bullet = Bullet {
                    x_pos: shooting_alien.x_pos + (shooting_alien.width as i32 / 2),
                    y_pos: shooting_alien.y_pos + shooting_alien.height as i32,
                    width: 2,
                    height: 10,
                    direction: Direction::Down,
                };
                self.alien_bullets.push(bullet);
            }
        }
    }

    pub fn run(&mut self) {
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

            self.process_alien_shots();

            // step remaining bullets
            for bullet in self.shooter_bullets.iter_mut() {
                bullet.step();
            }

            // step remaining bullets
            for bullet in self.alien_bullets.iter_mut() {
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

fn overlap(one_box: ((i32, i32), (i32, i32)), other_box: ((i32, i32), (i32, i32))) -> bool {
    one_box.0 .0 < other_box.1 .0
        && one_box.1 .0 > other_box.0 .0
        && one_box.0 .1 < other_box.1 .1
        && one_box.1 .1 > other_box.0 .1
}
