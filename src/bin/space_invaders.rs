use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use space_invaders::game_characters::alien::Alien;
use space_invaders::game_characters::bullet::Bullet;
use space_invaders::game_characters::shooter::Shooter;
use space_invaders::Direction;
use std::thread::sleep;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("space_invaders", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let shooter_width = 50;
    let shooter_height = 25;
    let mut shooter = Shooter {
        x_pos: (canvas.viewport().width() / 2) as i32 - (shooter_width / 2) as i32,
        y_pos: canvas.viewport().height() as i32 - (shooter_height as i32) - 10,
        width: shooter_width,
        height: shooter_height,
        direction: Direction::None,
    };

    let mut active_bullets = Vec::new();

    let alien_width: u32 = 25;
    let alien_height = 25;
    let mut aliens = Vec::new();
    for i in 0..10 {
        let alien = Alien {
            x_pos: (i * (10 + alien_width as i32)) + 10,
            y_pos: 10 + alien_width as i32 + 10,
            width: alien_width,
            height: alien_height,
            direction: Direction::DownRight,
        };
        aliens.push(alien);
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    shooter.direction = Direction::Left;

                    // step the shooter
                    if shooter.x_pos - STEP_DISTANCE >= 0 {
                        shooter.step();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    shooter.direction = Direction::Right;

                    // step the shooter
                    if shooter.x_pos + STEP_DISTANCE + shooter.width as i32
                        <= canvas.viewport().width() as i32
                    {
                        shooter.step();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    let bullet = Bullet {
                        x_pos: shooter.x_pos + (shooter.width as i32 / 2),
                        y_pos: shooter.y_pos,
                        width: 2,
                        height: 10,
                        direction: Direction::Up,
                    };
                    active_bullets.push(bullet);
                }

                _ => {}
            }
        }
        // The rest of the game loop goes here...

        // remove bullets and aliens that will/have collided
        // Did anything collide?
        // all locations on the bottom of any alien will have a y coordinate located at alien.y_pos + alien.height
        // if a bullet reaches that y and also on any x coordinate in alien.x_pos up until alien.x_pos + alien.width
        // is this O(n^2)?
        active_bullets.retain(|bullet| {
            let mut is_destroyed = false;
            aliens.retain(|alien| {
                if bullet.y_pos <= alien.y_pos + alien.height as i32
                    && bullet.y_pos >= alien.y_pos as i32
                {
                    if bullet.x_pos >= alien.x_pos
                        && bullet.x_pos <= alien.x_pos + alien.width as i32
                    {
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
        active_bullets = active_bullets
            .into_iter()
            .filter(|bullet| bullet.y_pos - STEP_DISTANCE >= 0)
            .collect();

        // shift all aliens down and switch directions if any of them touched a side
        if aliens.iter().any(|alien| {
            alien.direction == Direction::DownLeft && alien.x_pos - STEP_DISTANCE <= 0
                || alien.direction == Direction::DownRight
                    && alien.x_pos + alien.width as i32 + STEP_DISTANCE
                        >= canvas.viewport().width() as i32
        }) {
            aliens = aliens
                .into_iter()
                .map(|mut alien| {
                    // switch direction
                    alien.direction = if alien.direction == Direction::DownLeft {
                        Direction::DownRight
                    } else {
                        Direction::DownLeft
                    };

                    // shift down
                    alien.y_pos += ALIEN_STEP_DISTANCE as i32;
                    alien
                })
                .collect();
        }

        // step remaining bullets
        for bullet in active_bullets.iter_mut() {
            bullet.step();
        }

        // step aliens
        for alien in aliens.iter_mut() {
            alien.step();
        }

        // draw
        draw_screen(&mut canvas, i, &shooter, &mut active_bullets, &mut aliens);
        canvas.present();

        // sleep?
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_screen(
    canvas: &mut WindowCanvas,
    i: u8,
    shooter: &Shooter,
    active_bullets: &mut Vec<Bullet>,
    aliens: &mut Vec<Alien>,
) {
    canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    canvas.clear();
    shooter.draw(canvas);
    for bullet in active_bullets.iter_mut() {
        bullet.draw(canvas);
    }

    for alien in aliens.iter_mut() {
        alien.draw(canvas);
    }
}

const STEP_DISTANCE: i32 = 10;
const ALIEN_STEP_DISTANCE: f64 = STEP_DISTANCE as f64 * 0.1;
