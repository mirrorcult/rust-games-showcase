// TODO T: Tetrominos
// TODO T: Get collision check working
// TODO T: Get placement working
// TODO T: Get rendering working (*)
// TODO G: Game logic

use std::io::{stdout, Read, Write};
use std::{thread, time::Duration};

use termion::{
    color,
    clear,
    cursor,
    raw::IntoRawMode,
    async_stdin
};

mod colors;
mod tetromino;
mod tetris;

use colors::TetrominoColor;
use tetromino::Tetromino;
use tetris::TetrisGame;

fn main() {
    let stdout = stdout();
    let mut stdin = async_stdin().bytes();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let mut game = TetrisGame::new(10, 20);
    
    // Main game loop
    // TODO G: Points system
    loop {
        // Create our random tetromino if we don't have one
        if !game.has_active_tetromino() {
            game.tetromino = Some(rand::random::<Tetromino>()); // starting point is always 4 blocks over
            game.tetromino_color = Some(rand::random::<TetrominoColor>());
        }

        // Handle movement downwards
        // TODO G: Add speed going up with points & time
        if game.frame_counter % 90 == 0 {
            game.move_tetromino_down();
        }

        // Handle input & movement left/right/down
        let input = stdin.next();
        match input {
            Some(Ok(b'q')) => break,
            Some(Ok(b'a')) => {
                game.move_tetromino_left();
            },
            Some(Ok(b'd')) => {
                game.move_tetromino_right();
            },
            Some(Ok(b's')) => {
                game.move_tetromino_down();
            },
            Some(Ok(b'z')) => {
                game.rotate_tetromino();
            }
            _ => (),
        }

        // ## Rendering ##

        write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

        // Draw tetris container/well/whatever you wanna call it
        for y in 0..game.height + 1 {
            for x in 0..game.width + 2 {
                if (x == 0 || x == game.width + 2 - 1) || (y == game.height + 1 - 1) {
                    write!(stdout, "{}{}█", cursor::Goto(x as u16 + 1, y as u16 + 1), color::Fg(color::Blue)).unwrap();
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
                let tetromino_idx = game.tetromino.unwrap().xy_to_idx(x, y, game.tetromino_rotation);
                if tetromino_chars.nth(tetromino_idx).unwrap() == '.' {
                    let x_to_render = game.tetromino_x + x + 1;
                    let y_to_render = game.tetromino_y + y + 1;
                    write!(stdout, "{}{}█", cursor::Goto(x_to_render as u16 + 1, y_to_render as u16 + 1), color::Fg(color::Red)).unwrap();
                }
            }
        }

        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(17)); // ~60fps, did this because of input handling bullshit
        game.frame_counter += 1;
    }
    println!("Frame count: {}\nRotation: {}\nXY: {} {}", game.frame_counter, game.tetromino_rotation, game.tetromino_x, game.tetromino_y);
}