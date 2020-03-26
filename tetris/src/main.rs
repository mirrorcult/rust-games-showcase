use std::io::{stdout, Read, Write};
use std::{thread, time::Duration};

use termion::{async_stdin, clear, color, cursor, screen, raw::IntoRawMode};

mod colors;
mod tetris;
mod tetromino;

use tetris::TetrisGame;

const GAME_WIDTH: usize = 10;
const GAME_HEIGHT: usize = 20;
const SCORE_TEXT_OFFSET: u16 = 20;
const SCREEN_X_OFFSET: usize = 4;
const LINE_WAIT_DURATION: usize = 60;
const LINE_FLASH_TIME: usize = 30;

fn main() {
    let stdout = stdout();
    let mut stdin = async_stdin().bytes();
    let mut stdout = screen::AlternateScreen::from(stdout.lock().into_raw_mode().unwrap());
    write!(stdout, "{}", cursor::Hide).unwrap();

    // Create new Tetris game: this handles tetromino creation, ending turn, etc..
    let mut game = TetrisGame::new(GAME_WIDTH, GAME_HEIGHT);

    // Main game loop
    loop {
        // Handle movement downwards
        if game.frame_counter % 90 == 0 {
            let curr_y = game.tetromino_y;
            game.move_tetromino_down();
            if game.tetromino_collides() {
                // remove piece
                game.tetromino_y = curr_y;
                game.turn_tetromino_to_debris();
                // since we added debris, check for lines
                game.check_for_lines();
            }
        }

        // Handle input & movement left/right/down as well as collision detection
        let input = stdin.next();
        match input {
            Some(Ok(b'q')) => break,
            Some(Ok(b'a')) => {
                game.move_tetromino_left();
                if game.tetromino_collides() {
                    game.move_tetromino_right();
                }
            }
            Some(Ok(b'd')) => {
                game.move_tetromino_right();
                if game.tetromino_collides() {
                    game.move_tetromino_left();
                }
            }
            Some(Ok(b's')) => {
                let curr_y = game.tetromino_y;
                game.move_tetromino_down();
                if game.tetromino_collides() {
                    game.tetromino_y = curr_y;
                }
            }
            Some(Ok(b'z')) => {
                let curr_rot = game.tetromino_rotation;
                game.rotate_tetromino();
                if game.tetromino_collides() {
                    game.tetromino_rotation = curr_rot;
                }
            }
            _ => (),
        }

        // ## Rendering ##

        // Clear everything except the well (WIP)

        write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

        // Render our score
        write!(stdout, "{}Score: {}", cursor::Goto(SCORE_TEXT_OFFSET, 1), game.score).unwrap();

        // Render line deletion, if need be
        if let Some(lines_vec) = &game.lines_to_remove {
            match game.lines_frame_counter {
                None => game.lines_frame_counter = Some(LINE_WAIT_DURATION),
                Some(n) => {
                    match n {
                        0 => {
                            // line finished!
                            game.remove_lines();
                            game.lines_frame_counter = None;
                        },
                        LINE_FLASH_TIME => {
                            // render lines as = signs
                            
                        },
                        _ => {
                            game.lines_frame_counter = Some(n - 1);
                        }
                    }
                }
            }
        }

        // Draw our tetromino
        // Trick here is to get the index of the string through xy_to_idx (which accounts for rotation)
        // Then we draw it at the top left point, offset by one because of the well
        let tetromino_string = game.tetromino.unwrap().as_string();
        for y in 0..4 {
            for x in 0..4 {
                let mut tetromino_chars = tetromino_string.chars();
                let tetromino_idx =
                    game.tetromino
                        .unwrap()
                        .xy_to_idx(x, y, game.tetromino_rotation);
                if tetromino_chars.nth(tetromino_idx).unwrap() == '.' {
                    let x_to_render = game.tetromino_x + x + 1;
                    let y_to_render = game.tetromino_y + y + 1;
                    write!(
                        stdout,
                        "{}{}█",
                        cursor::Goto(x_to_render as u16 + 1, y_to_render as u16 + 1),
                        game.tetromino_color.unwrap().as_terminal_color()
                    )
                    .unwrap();
                }
            }
        }

        // Draw tetris container/well/whatever you wanna call it
        for y in 0..game.height + 1 {
            for x in SCREEN_X_OFFSET..game.width + SCREEN_X_OFFSET + 2 {
                if (x == SCREEN_X_OFFSET || x == game.width + SCREEN_X_OFFSET + 1)
                    || (y == game.height + 1 - 1)
                {
                    write!(
                        stdout,
                        "{}{}█",
                        cursor::Goto(x as u16 + 1, y as u16 + 1),
                        color::Fg(color::Blue)
                    )
                    .unwrap();
                }
            }
        }

        // Draw 'debris' (fallen tetris pieces)
        for y in 0..game.height {
            for x in 0..game.width {
                if game.has_debris_at(x, y) {
                    let color = game.get_color_at(x, y).unwrap().as_terminal_color();
                    write!(
                        stdout,
                        "{}{}█",
                        cursor::Goto((x + SCREEN_X_OFFSET + 2) as u16, y as u16 + 2),
                        color
                    )
                    .unwrap();
                }
            }
        }

        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(17)); // ~60fps, did this because of input handling bullshit
        game.frame_counter += 1;
    }
}
