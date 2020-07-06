extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

use rand::Rng;
use std::time::Duration;

use piston::window::WindowSettings;
use piston_window::*;

const WIDTH: usize = 4;
const HEIGHT: usize = 4;

fn stack_dir(d: Event, map: &mut Vec<Vec<u16>>, state: u8) -> u8 {
    let mut rez: u8 = state;
    match d.press_args() {
        Some(Button::Keyboard(Key::W)) => {
            rez = 1;
        }
        Some(Button::Keyboard(Key::A)) => {
            rez = 1;
        }
        Some(Button::Keyboard(Key::S)) => {
            rez = 1;
        }
        Some(Button::Keyboard(Key::D)) => {
            rez = 1;
        }
        Some(Button::Keyboard(Key::Escape)) => {
            rez = 5;
        }
        _ => {
            rez = 50;
        }
    }
    rez
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Rust 2048 by BoloniniD", [400, 450]) // creates a new window
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap(); //loading font
    let mut chk = false;
    let mut map = vec![vec![0; WIDTH]; HEIGHT];
    let mut next_2: Vec<(usize, usize)> = Vec::new();
    let mut state: u8 = 0;
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, d| {
            clear([1.0, 0.7, 0.0, 1.0], g);
            match state {
                0 => {
                    let transform = c.transform.trans(35.0, 170.0);
                    text::Text::new_color([1.0, 0.3, 0.1, 1.0], 32)
                        .draw(
                            "PRESS [SPACE] TO START",
                            &mut glyphs,
                            &c.draw_state,
                            transform,
                            g,
                        )
                        .unwrap();
                    let transform = c.transform.trans(40.0, 190.0);
                    text::Text::new_color([1.0, 0.3, 0.1, 1.0], 16)
                        .draw(
                            "*use WASD to stack tiles in different directions*",
                            &mut glyphs,
                            &c.draw_state,
                            transform,
                            g,
                        )
                        .unwrap();
                    let transform = c.transform.trans(136.0, 300.0);
                    text::Text::new_color([1.0, 0.3, 0.1, 1.0], 16)
                        .draw(
                            "*press ESC to exit*",
                            &mut glyphs,
                            &c.draw_state,
                            transform,
                            g,
                        )
                        .unwrap();
                    glyphs.factory.encoder.flush(d);
                }
                1 => {
                    for i in 0..HEIGHT {
                        for j in 0..WIDTH {
                            if map[i][j] > 0 {
                                let h = map[i][j] as f32;
                                let ix = i as f64;
                                let jy = j as f64;
                                let square = rectangle::square(100.0 * ix, 100.0 * jy, 100.0);
                                rectangle(
                                    [h * 0.1 + 0.1, 0.0, 0.0, 1.0], // red
                                    square,
                                    c.transform,
                                    g,
                                );
                            } else {
                                let ix = i as f64;
                                let jy = j as f64;
                                let square = rectangle::square(100.0 * ix, 100.0 * jy, 100.0);
                                rectangle(
                                    [1.0, 0.7, 0.0, 1.0], // red
                                    square,
                                    c.transform,
                                    g,
                                );
                            }
                        }
                    }
                }
                2 => {
                    let transform = c.transform.trans(59.0, 170.0);
                    text::Text::new_color([1.0, 0.3, 0.3, 1.0], 32)
                        .draw("CONGRATULATIONS!", &mut glyphs, &c.draw_state, transform, g)
                        .unwrap();

                    let transform = c.transform.trans(35.0, 210.0);
                    text::Text::new_color([1.0, 0.3, 0.3, 1.0], 20)
                        .draw(
                            "PRESS Y TO RESTART, PRESS N TO EXIT",
                            &mut glyphs,
                            &c.draw_state,
                            transform,
                            g,
                        )
                        .unwrap();
                    glyphs.factory.encoder.flush(d);
                }
                3 => {
                    let transform = c.transform.trans(100.0, 170.0);
                    text::Text::new_color([1.0, 0.3, 0.3, 1.0], 34)
                        .draw("YOU'VE LOST!", &mut glyphs, &c.draw_state, transform, g)
                        .unwrap();

                    let transform = c.transform.trans(35.0, 210.0);
                    text::Text::new_color([1.0, 0.3, 0.3, 1.0], 20)
                        .draw(
                            "PRESS Y TO RESTART, PRESS N TO EXIT",
                            &mut glyphs,
                            &c.draw_state,
                            transform,
                            g,
                        )
                        .unwrap();
                    glyphs.factory.encoder.flush(d);
                }
                4 => {
                    chk = true;
                }
                _ => {
                    println!("Something went wrong");
                    chk = true;
                }
            }
        });
        match state {
            0 => {
                next_2 = Vec::new();
                let d = window.wait_event_timeout(Duration::new(3, 0));
                if d != None {
                    match d.unwrap().press_args() {
                        Some(Button::Keyboard(Key::Space)) => {
                            for i in 0..HEIGHT {
                                for j in 0..WIDTH {
                                    if map[i][j] == 0 {
                                        next_2.push((i, j));
                                    }
                                }
                            }
                            if next_2.len() == 0 {
                                state = 3;
                            } else {
                                let chosen: usize = rand::random::<usize>() % next_2.len();
                                map[next_2[chosen].0][next_2[chosen].1] = 2;
                            }
                            state = 1;
                        }
                        Some(Button::Keyboard(Key::Escape)) => {
                            state = 4;
                        }
                        _ => {}
                    }
                }
            }
            1 => {
                let d = window.wait_event_timeout(Duration::new(3, 0));
                if d != None {
                    state = stack_dir(d.unwrap(), &mut map, state);
                    if state == 1 {
                        next_2 = Vec::new();
                        for i in 0..HEIGHT {
                            for j in 0..WIDTH {
                                if map[i][j] == 0 {
                                    next_2.push((i, j));
                                }
                            }
                        }
                        if next_2.len() == 0 {
                            state = 3;
                        } else {
                            let chosen: usize = rand::random::<usize>() % next_2.len();
                            map[next_2[chosen].0][next_2[chosen].1] = 2;
                        }
                    } else if state == 50 {
                        state = 1;
                    }
                }
            }
            2..=3 => {
                let d = window.wait_event_timeout(Duration::new(3, 0));
                if d != None {
                    match d.unwrap().press_args() {
                        Some(Button::Keyboard(Key::N)) => {
                            state = 4;
                        }
                        Some(Button::Keyboard(Key::Y)) => {
                            for i in 0..HEIGHT {
                                for j in 0..WIDTH {
                                    map[i][j] = 0;
                                }
                            }
                            state = 1;
                            for i in 0..HEIGHT {
                                for j in 0..WIDTH {
                                    if map[i][j] == 0 {
                                        next_2.push((i, j));
                                    }
                                }
                            }
                            if next_2.len() == 0 {
                                state = 3;
                            } else {
                                let chosen: usize = rand::random::<usize>() % next_2.len();
                                map[next_2[chosen].0][next_2[chosen].1] = 2;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        if chk {
            break;
        }
    }
}
