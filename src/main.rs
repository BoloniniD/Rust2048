extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

use std::time::Duration;
use std::fs;

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
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap(); //loading font
    let state0_texts = fs::read_to_string(assets.join("state0_text.txt")).expect("");
    let state0_texts: Vec<&str> = state0_texts.split("==").collect();
    let mut chk = false; // if chk == true, then stop the program
    let mut map = vec![vec![0; WIDTH]; HEIGHT]; // game map
    let mut next_2: Vec<(usize, usize)> = Vec::new(); // vector for 0-tiles enumeration
    let mut state: u8 = 0; // state
    let mut score: u32 = 0; // score
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, d| {
            match state {
                0 => {
                    // initialization
                    clear([1.0, 0.7, 0.0, 1.0], g); // background
                    for i in 0..4 { // 4 square things for 2 0 4 8
                        let sq = rectangle::square(50.0 + 80.0 * (i as f64), 30.0, 70.0);
                        rectangle([0.6, 0.2, 0.1, 1.0], sq, c.transform, g);
                    }
                    let text_size = state0_texts.len(); //shows, how many lines are in the text
                    for i in 0..text_size {
                        let line: Vec<&str> = state0_texts[i].split("|").collect();
                        let transf_x: f64 = line[0].trim().parse().expect("");
                        let transf_y: f64 = line[1].trim().parse().expect("");
                        let transform = c.transform.trans(transf_x, transf_y);
                        let mut col: Vec<f32> = Vec::new();
                        for j in 2..6 {
                            let temp: f32 = line[j].trim().parse().expect("");
                            col.push(temp);
                        }
                        let col = [col[0], col[1], col[2], col[3]];
                        let font_size: u32 = line[6].trim().parse().expect("");
                        text::Text::new_color(col, font_size)
                            .draw(line[7], &mut glyphs, &c.draw_state, transform, g)
                            .unwrap();
                    }
                    glyphs.factory.encoder.flush(d);
                }
                1 => {
                    // main game state, draws tiles, etc.
                    clear([1.0, 0.7, 0.0, 1.0], g); // background
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
                    let square = rectangle::square(0.0, 0.0, 450.0);
                    rectangle(
                        [1.0, 0.7, 0.0, 0.05], // different red
                        square,
                        c.transform,
                        g,
                    );
                    let transform = c.transform.trans(65.0, 170.0);
                    text::Text::new_color([1.0, 0.3, 0.3, 1.0], 32)
                        .draw("CONGRATULATIONS!", &mut glyphs, &c.draw_state, transform, g)
                        .unwrap();

                    let transform = c.transform.trans(110.0, 260.0);
                    text::Text::new_color([1.0, 0.3, 0.3, 1.0], 20)
                        .draw(
                            "PRESS [Y] TO RESTART",
                            &mut glyphs,
                            &c.draw_state,
                            transform,
                            g,
                        )
                        .unwrap();
                    let transform = c.transform.trans(130.0, 290.0);
                    text::Text::new_color([1.0, 0.3, 0.3, 1.0], 20)
                        .draw(
                            "PRESS [N] TO EXIT",
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
                    clear([1.0, 0.7, 0.0, 1.0], g); // background
                    let transform = c.transform.trans(90.0, 100.0);
                    text::Text::new_color([1.0, 0.3, 0.3, 1.0], 40)
                        .draw("YOU'VE LOST!", &mut glyphs, &c.draw_state, transform, g)
                        .unwrap();
                    let transform = c.transform.trans(70.0, 150.0);
                    text::Text::new_color([1.0, 0.3, 0.3, 1.0], 34)
                        .draw("YOUR SCORE WAS:", &mut glyphs, &c.draw_state, transform, g)
                        .unwrap();
                    let mut _s = score.to_string();
                    while _s.trim().len() < 5 {
                        _s.insert(0, '0');
                    }
                    let _s: &str = &_s[..];
                    let transform = c.transform.trans(156.0, 190.0);
                    text::Text::new_color([1.0, 0.3, 0.3, 1.0], 34)
                        .draw(_s, &mut glyphs, &c.draw_state, transform, g)
                        .unwrap();
                    let transform = c.transform.trans(110.0, 260.0);
                    text::Text::new_color([1.0, 0.3, 0.3, 1.0], 20)
                        .draw(
                            "PRESS [Y] TO RESTART",
                            &mut glyphs,
                            &c.draw_state,
                            transform,
                            g,
                        )
                        .unwrap();
                    let transform = c.transform.trans(130.0, 290.0);
                    text::Text::new_color([1.0, 0.3, 0.3, 1.0], 20)
                        .draw(
                            "PRESS [N] TO EXIT",
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
                                } else if map[i][j] == 2048 {
                                    state = 2;
                                    continue;
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
