extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

use rand::Rng;
use std::io;

use piston::window::WindowSettings;
use piston::TextEvent;
use piston_window::*;

const WIDTH: usize = 4;
const HEIGHT: usize = 4;

fn stack_dir(x: u8, map: &mut Vec<Vec<u16>>) {
    //stacks tiles on direction x
}

fn main() {
    let mut map = vec![vec![0; WIDTH]; HEIGHT];
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(
        "Rust 2048 by BoloniniD (mah first true exp, it's 100% awful)",
        [400, 400],
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();
    let mut state: u8 = 0;
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();
    window.set_lazy(true);
    while let Some(e) = window.next() {
        match state {
            0 => {
                //initializer
                state = 1;
                window.draw_2d(&e, |c, g, device| {
                    let transform = c.transform.trans(35.0, 170.0);
                    clear([1.0, 0.7, 0.0, 1.0], g);
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
                    glyphs.factory.encoder.flush(device);
                });
                if let Some(Button::Keyboard(Key::Space)) = e.press_args() {
                    println!("Init was successful!");
                    println!("Ready");
                } else {
                    state = 0;
                }
            }
            1 => {
                // main state
                println!("Drawing map");
                let mut next_2: Vec<(usize, usize)> = Vec::new();
                for i in 0..HEIGHT - 1 {
                    for j in 0..WIDTH - 1 {
                        if map[i][j] == 0 {
                            next_2.push((i, j));
                        }
                    }
                }
                if (next_2.len() > 0) {
                    let chosen: usize = rand::random::<usize>() % next_2.len();
                    map[next_2[chosen].0][next_2[chosen].1] = 2;
                    window.draw_2d(&e, |c, g, device| {
                        clear([1.0, 0.7, 0.0, 1.0], g);
                        for i in 0..HEIGHT - 1 {
                            for j in 0..WIDTH - 1 {
                                if map[i][j] > 0 {
                                    let h = map[i][j] as f32;
                                    let ix = i as f64;
                                    let jy = j as f64;
                                    rectangle(
                                        [h * 0.1 + 0.1, 0.0, 0.0, 1.0], // red
                                        [
                                            100.0 * ix,
                                            100.0 * jy,
                                            100.0 * (ix + 1.0),
                                            100.0 * (jy + 1.0),
                                        ],
                                        c.transform,
                                        g,
                                    );
                                } else {
                                    let ix = i as f64;
                                    let jy = j as f64;
                                    rectangle(
                                        [1.0, 0.7, 0.0, 1.0], // red
                                        [
                                            100.0 * ix,
                                            100.0 * jy,
                                            100.0 * (ix + 1.0),
                                            100.0 * (jy + 1.0),
                                        ],
                                        c.transform,
                                        g,
                                    );
                                }
                            }
                        }
                    });
                    println!("Awaiting direction");
                    let mut d = 1;
                    println!("Stacking tiles");
                    stack_dir(d, &mut map);
                    println!("Checking state");
                } else {
                    state = 3;
                }
            }
            2 => {
                //win
                println!("Congrats! You've got the 2048 tile!");
                state = 4;
            }
            3 => {
                //lose
                println!(":(");
                state = 4;
            }
            4 => { //retry?
            }
            _ => {
                println!("aieou, how did u manage to break this program?");
            }
        }
    }
}
