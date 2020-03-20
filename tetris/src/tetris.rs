use crate::tetromino::Tetromino;
use crate::colors::TetrominoColor;

/// Struct holding information and methods relating to the current tetris game.
pub struct TetrisGame {
    pub width: usize,
    pub height: usize,
    pub tetromino: Option<Tetromino>,
    pub tetromino_color: Option<TetrominoColor>,
    pub tetromino_x: usize,
    pub tetromino_y: usize,
    pub tetromino_rotation: usize,
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
            tetromino_x: 4,
            tetromino_y: 0,
            tetromino_rotation: 0,
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

    pub fn rotate_tetromino(&mut self) {
        self.tetromino_rotation = (self.tetromino_rotation + 1) % 4;
    }

    pub fn move_tetromino_down(&mut self) {
        self.tetromino_y += 1;
    }

    pub fn move_tetromino_left(&mut self) {
        if self.tetromino_x >= 1 {
            self.tetromino_x -= 1;
        }
    }

    pub fn move_tetromino_right(&mut self) {
        // TODO real colission detection
        if self.tetromino_x <= self.width - 4 {
            self.tetromino_x += 1;
        }
    }
}
