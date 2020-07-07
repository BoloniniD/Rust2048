extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

use std::time::Duration;

use piston::window::WindowSettings;
use piston_window::*;

const WIDTH: usize = 4;
const HEIGHT: usize = 4;

fn stack_dir(d: Event, map: &mut Vec<Vec<u16>>) -> u8 { // function that stacks tiles in some direction
    let mut rez: u8 = 0;
    match d.press_args() { //matches events with directions
        Some(Button::Keyboard(Key::W)) => {
            for i in 0..WIDTH { //up
                for _ in 0..4 {
                    for j in 0..HEIGHT - 1 {
                        if map[i][j] == 0 {
                            map[i][j] = map[i][j + 1];
                            map[i][j + 1] = 0;
                        }
                    }
                }
                for j in 0..HEIGHT - 1 {
                    if map[i][j] == map[i][j + 1] {
                        map[i][j] += map[i][j + 1];
                        map[i][j + 1] = 0;
                    }
                }
                for _ in 0..4 {
                    for j in 0..HEIGHT - 1 {
                        if map[i][j] == 0 {
                            map[i][j] = map[i][j + 1];
                            map[i][j + 1] = 0;
                        }
                    }
                }
            }
            rez = 1;
        }
        Some(Button::Keyboard(Key::A)) => { //left
            for j in 0..HEIGHT {
                for _ in 0..4 {
                    for i in 0..WIDTH - 1 {
                        if map[i][j] == 0 {
                            map[i][j] = map[i + 1][j];
                            map[i + 1][j] = 0;
                        }
                    }
                }
                for i in 0..WIDTH - 1 {
                    if map[i][j] == map[i + 1][j] {
                        map[i][j] += map[i + 1][j];
                        map[i + 1][j] = 0;
                    }
                }
                for _ in 0..4 {
                    for i in 0..WIDTH - 1 {
                        if map[i][j] == 0 {
                            map[i][j] = map[i + 1][j];
                            map[i + 1][j] = 0;
                        }
                    }
                }
            }
            rez = 1;
        }
        Some(Button::Keyboard(Key::S)) => { //down
            for i in 0..WIDTH {
                for _ in 0..4 {
                    for j in 1..HEIGHT {
                        if map[i][HEIGHT - j] == 0 {
                            map[i][HEIGHT - j] = map[i][HEIGHT - j - 1];
                            map[i][HEIGHT - j - 1] = 0;
                        }
                    }
                }
                for j in 1..HEIGHT {
                    if map[i][HEIGHT - j] == map[i][HEIGHT - j - 1] {
                        map[i][HEIGHT - j] += map[i][HEIGHT - j - 1];
                        map[i][HEIGHT - j - 1] = 0;
                    }
                }
                for _ in 0..4 {
                    for j in 1..HEIGHT {
                        if map[i][HEIGHT - j] == 0 {
                            map[i][HEIGHT - j] = map[i][HEIGHT - j - 1];
                            map[i][HEIGHT - j - 1] = 0;
                        }
                    }
                }
            }
            rez = 1;
        }
        Some(Button::Keyboard(Key::D)) => { // right
            for j in 0..HEIGHT {
                for _ in 0..4 {
                    for i in 1..WIDTH {
                        if map[WIDTH - i][j] == 0 {
                            map[WIDTH - i][j] = map[WIDTH - i - 1][j];
                            map[WIDTH - i - 1][j] = 0;
                        }
                    }
                }
                for i in 1..WIDTH {
                    if map[WIDTH - i][j] == map[WIDTH - i - 1][j] {
                        map[WIDTH - i][j] += map[WIDTH - i - 1][j];
                        map[WIDTH - i - 1][j] = 0;
                    }
                }
                for _ in 0..4 {
                    for i in 1..WIDTH {
                        if map[WIDTH - i][j] == 0 {
                            map[WIDTH - i][j] = map[WIDTH - i - 1][j];
                            map[WIDTH - i - 1][j] = 0;
                        }
                    }
                }
            }
            rez = 1;
        }
        Some(Button::Keyboard(Key::Escape)) => {
            rez = 4;
            // exit state
        }
        _ => {
            rez = 50;
            // actually, it does magic
            // if event is not one from the list above, this "state"
            // allows program to skip the step with random choice of a new 2 tile
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
    let assets = find_folder::Search::ParentsThenKids(3, 3) // searches for an assets folder
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window //loads font
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap(); //loading font
    let mut chk = false; // if chk == true, then stop the program
    let mut map = vec![vec![0; WIDTH]; HEIGHT]; // game map
    let mut next_2: Vec<(usize, usize)> = Vec::new(); // vector for 0-tiles enumeration
    let mut state: u8 = 0; // state
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, d| {
            clear([1.0, 0.7, 0.0, 1.0], g); // background
            match state {
                0 => {
                    // initialization
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
                    // main game state, draws tiles, etc.
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
                    // win state
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
                    // loose state
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
                    // exit state
                    chk = true;
                }
                _ => {
                    // err state
                    println!("Something went wrong");
                    chk = true;
                }
            }
        });
        match state {
            0 => {
                // state 0 event matcher
                next_2 = Vec::new();
                let d = window.wait_event_timeout(Duration::new(1, 0));
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
                                state = 1;
                            }
                        }
                        Some(Button::Keyboard(Key::Escape)) => {
                            state = 4;
                        }
                        _ => {}
                    }
                }
            }
            1 => {
                // state 1 evnt matcher
                let d = window.wait_event_timeout(Duration::new(10, 0));
                if d != None {
                    state = stack_dir(d.unwrap(), &mut map);
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
                //state 2 and 3 event matcher
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
            // exit the loop
            break;
        }
    }
}
