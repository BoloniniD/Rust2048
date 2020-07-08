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

fn stack_dir(d: Event, map: &mut Vec<Vec<u16>>, mut score: u32) -> (u8, u32) {
    // function that stacks tiles in some direction
    let mut rez: u8 = 0;
    match d.press_args() {
        //matches events with directions
        Some(Button::Keyboard(Key::W)) => {
            for i in 0..WIDTH {
                //up
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
                        score += map[i][j] as u32;
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
        Some(Button::Keyboard(Key::A)) => {
            //left
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
                        score += map[i][j] as u32;
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
        Some(Button::Keyboard(Key::S)) => {
            //down
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
                        score += map[i][HEIGHT - j] as u32;
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
        Some(Button::Keyboard(Key::D)) => {
            // right
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
                        score += map[WIDTH - i][j] as u32;
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
    (rez, score)
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
    let mut score: u32 = 0; // score
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, d| {
            clear([1.0, 0.7, 0.0, 1.0], g); // background
            match state {
                0 => {
                    // initialization
                    let xc: f64 = 50.0;
                    let sq = rectangle::square(xc, 30.0, 70.0);
                    rectangle([0.6, 0.2, 0.1, 1.0], sq, c.transform, g);
                    let transform = c.transform.trans(75.5, 76.5);
                    text::Text::new_color([0.1, 1.0, 0.1, 1.0], 44)
                        .draw("2", &mut glyphs, &c.draw_state, transform, g)
                        .unwrap();
                    let sq = rectangle::square(xc + 80.0, 30.0, 70.0);
                    rectangle([0.6, 0.2, 0.1, 1.0], sq, c.transform, g);
                    let transform = c.transform.trans(155.5, 76.5);
                    text::Text::new_color([0.1, 1.0, 0.1, 1.0], 44)
                        .draw("0", &mut glyphs, &c.draw_state, transform, g)
                        .unwrap();
                    let sq = rectangle::square(xc + 160.0, 30.0, 70.0);
                    rectangle([0.6, 0.2, 0.1, 1.0], sq, c.transform, g);
                    let transform = c.transform.trans(235.5, 76.5);
                    text::Text::new_color([0.1, 1.0, 0.1, 1.0], 44)
                        .draw("4", &mut glyphs, &c.draw_state, transform, g)
                        .unwrap();
                    let sq = rectangle::square(xc + 240.0, 30.0, 70.0);
                    rectangle([0.6, 0.2, 0.1, 1.0], sq, c.transform, g);
                    let transform = c.transform.trans(315.5, 76.5);
                    text::Text::new_color([0.1, 1.0, 0.1, 1.0], 44)
                        .draw("8", &mut glyphs, &c.draw_state, transform, g)
                        .unwrap();
                    let transform = c.transform.trans(22.0, 170.0);
                    text::Text::new_color([1.0, 0.2, 0.1, 1.0], 31)
                        .draw(
                            "* PRESS [SPACE] TO START *",
                            &mut glyphs,
                            &c.draw_state,
                            transform,
                            g,
                        )
                        .unwrap();
                    let transform = c.transform.trans(40.0, 190.0);
                    text::Text::new_color([1.0, 0.2, 0.1, 1.0], 16)
                        .draw(
                            "*use WASD to stack tiles in different directions*",
                            &mut glyphs,
                            &c.draw_state,
                            transform,
                            g,
                        )
                        .unwrap();
                    let transform = c.transform.trans(52.0, 230.0);
                    text::Text::new_color([1.0, 0.2, 0.1, 1.0], 31)
                        .draw(
                            "* PRESS [ESC] TO EXIT *",
                            &mut glyphs,
                            &c.draw_state,
                            transform,
                            g,
                        )
                        .unwrap();
                    glyphs.factory.encoder.flush(d);
                    let transform = c.transform.trans(140.0, 330.0);
                    text::Text::new_color([1.0, 0.2, 0.1, 1.0], 20)
                        .draw("{by BoloniniD}", &mut glyphs, &c.draw_state, transform, g)
                        .unwrap();
                    glyphs.factory.encoder.flush(d);
                    let transform = c.transform.trans(80.0, 357.0);
                    text::Text::new_color([1.0, 0.2, 0.1, 1.0], 18)
                        .draw(
                            "{the original is somewhere else...}",
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
                    let tr = c.transform.trans(110.0, 35.0);
                    text::Text::new_color([1.0, 0.2, 0.1, 1.0], 28)
                        .draw("SCORE: ", &mut glyphs, &c.draw_state, tr, g)
                        .unwrap();
                        glyphs.factory.encoder.flush(d);
                    let tr = c.transform.trans(240.0, 35.0);
                    let sc = score.to_string();
                    let sc: &str = &sc[..];
                    let sc: &str = &sc[..];
                    text::Text::new_color([1.0, 0.2, 0.1, 1.0], 28)
                        .draw(sc, &mut glyphs, &c.draw_state, tr, g)
                        .unwrap();
                    glyphs.factory.encoder.flush(d);
                    for i in 0..HEIGHT {
                        for j in 0..WIDTH {
                            if map[i][j] > 0 {
                                let h = map[i][j] as f32;
                                let ix = i as f64;
                                let jy = j as f64;
                                let square =
                                    rectangle::square(100.0 * ix, 100.0 * jy + 50.0, 100.0);
                                rectangle(
                                    [h.log(2.0) * 0.05 + 0.1, 0.0, 0.0, 1.0], // different red
                                    square,
                                    c.transform,
                                    g,
                                );
                                let mut transform = c.transform.trans(0.0, 0.0);
                                match map[i][j] {
                                    0..=9 => {
                                        transform = c
                                            .transform
                                            .trans(100.0 * ix + 43.0, 100.0 * jy + 106.5);
                                    }
                                    10..=99 => {
                                        transform = c
                                            .transform
                                            .trans(100.0 * ix + 36.0, 100.0 * jy + 106.5);
                                    }
                                    100..=999 => {
                                        transform = c
                                            .transform
                                            .trans(100.0 * ix + 29.0, 100.0 * jy + 106.5);
                                    }
                                    1000..=9999 => {
                                        transform = c
                                            .transform
                                            .trans(100.0 * ix + 20.5, 100.0 * jy + 106.5);
                                    }
                                    _ => {}
                                }
                                let st = map[i][j].to_string();
                                let st: &str = &st[..];
                                text::Text::new_color([0.1, 1.0, 0.1, 1.0], 32)
                                    .draw(st, &mut glyphs, &c.draw_state, transform, g)
                                    .unwrap();
                                glyphs.factory.encoder.flush(d);
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
                // state 1 event matcher
                let d = window.wait_event_timeout(Duration::new(3, 0));
                if d != None {
                    let stac = stack_dir(d.unwrap(), &mut map, score);
                    state = stac.0;
                    score = stac.1;
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
