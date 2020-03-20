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
    input::TermRead,
    raw::IntoRawMode,
    async_stdin
};

mod colors;
mod tetromino;
mod tetris;

use colors::TetrominoColor;
use tetromino::{TetrominoType, Tetromino};
use tetris::TetrisGame;

fn main() {
    let stdout = stdout();
    let mut stdin = async_stdin().bytes();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    let mut Game = TetrisGame::new(10, 20);
    
    // Draw tetris container/well/whatever you wanna call it
    for y in 0..Game.height {
        for x in 0..Game.width {
            if (x == 0 || x == Game.width - 1) || (y == Game.height - 1) {
                write!(stdout, "{}{}█", cursor::Goto(x as u16 + 2, y as u16 + 2), color::Fg(color::Blue)).unwrap(); // +2 -> offset from border
            }
        }
    }
    stdout.flush().unwrap();
    
    // Main game loop
    // TODO G: Points system
    loop {
        // Create our random tetromino if we don't have one
        if !Game.has_active_tetromino() {
            let tetromino_type = rand::random::<TetrominoType>();
            Game.tetromino = Some(Tetromino::new(4, 0, tetromino_type)); // starting point is always 4 blocks over
            Game.tetromino_color = Some(rand::random::<TetrominoColor>());
        }

        // Handle moving tetromino down
        // TODO G: Make tetrominos actually move down
        // TODO G: Add speed going up
        if Game.frame_counter % 30 == 0 {
            write!(stdout, "{}F", cursor::Goto(1, Game.frame_counter / 30)).unwrap();
            stdout.flush().unwrap();
        }

        // Handle input
        let input = stdin.next();
        match input {
            Some(Ok(b'q')) => return,
            Some(Ok(b'z')) => {
                // rotate
                write!(stdout, "{}X", cursor::Goto(3, 5)).unwrap();
            }
            _ => (),
        }

        Game.frame_counter += 1;
        thread::sleep(Duration::from_millis(17)); // ~60fps, did this because of input handling bullshit
        stdout.flush().unwrap();
    }
}