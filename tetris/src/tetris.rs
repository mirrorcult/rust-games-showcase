use crate::tetromino::Tetromino;
use crate::colors::TetrominoColor;

/// Struct holding information and methods relating to the current tetris game.
pub struct TetrisGame {
    pub width: usize,
    pub height: usize,
    pub tetromino: Option<Tetromino>,
    pub tetromino_color: Option<TetrominoColor>,
    pub frame_counter: u16,
    grid: Vec<bool>, // true -> contains placed tetromino block, false -> doesnt
}

impl TetrisGame {
    pub fn new(width: usize, height: usize) -> TetrisGame {
        TetrisGame {
            width,
            height,
            tetromino: None,
            tetromino_color: None,
            frame_counter: 0,
            grid: vec![false; width * height],
        }
    }

    pub fn mark_square_as_placed(&mut self, idx: usize) {
        if idx < self.width * self.height {
            self.grid[idx] = true;
        }
    }

    pub fn has_active_tetromino(&self) -> bool {
        self.tetromino.is_some()
    }
}
