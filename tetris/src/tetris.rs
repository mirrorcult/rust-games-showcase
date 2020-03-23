use crate::colors::TetrominoColor;
use crate::tetromino::Tetromino;

const SCREEN_X_OFFSET: usize = 4;

/// Struct holding information and methods relating to the current tetris game.
pub struct TetrisGame {
    pub width: usize,
    pub height: usize,
    pub tetromino: Option<Tetromino>,
    pub tetromino_color: Option<TetrominoColor>,
    pub tetromino_x: usize,
    pub tetromino_y: usize,
    pub tetromino_rotation: usize,
    pub frame_counter: usize,
    grid: Vec<(bool, Option<TetrominoColor>)>, // true -> contains placed tetromino block, false -> doesnt
}

impl TetrisGame {
    pub fn new(width: usize, height: usize) -> TetrisGame {
        TetrisGame {
            width,
            height,
            tetromino: Some(rand::random::<Tetromino>()),
            tetromino_color: Some(rand::random::<TetrominoColor>()),
            tetromino_x: 4 + SCREEN_X_OFFSET,
            tetromino_y: 0,
            tetromino_rotation: 0,
            frame_counter: 0,
            grid: vec![(false, None); width * height + 1], // tuple of bool (debris or not) + what color the debris is
        }
    }

    /// Returns true if the tetromino collides with the wall or a debris block.
    pub fn tetromino_collides(&self) -> bool {
        let tetromino_string = self.tetromino.unwrap().as_string();
        for y in 0..4 {
            for x in 0..4 {
                let mut tetromino_chars = tetromino_string.chars();
                let tetromino_idx =
                    self.tetromino
                        .unwrap()
                        .xy_to_idx(x, y, self.tetromino_rotation);
                if tetromino_chars.nth(tetromino_idx).unwrap() == '.' {
                    // offset by current x,y
                    let offset_x = self.tetromino_x + x;
                    let offset_y = self.tetromino_y + y;
                    // check for collision with borders
                    if (offset_x == SCREEN_X_OFFSET - 1 || offset_x >= self.width + SCREEN_X_OFFSET)
                        || (offset_y >= self.height - 1)
                    {
                        return true;
                    }
                    // check for collision with debris
                    if self.grid[self.tetromino_xy_to_grid_idx(offset_x, offset_y)].0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn turn_tetromino_to_debris(&mut self) {
        let tetromino_string = self.tetromino.unwrap().as_string();
        for y in 0..4 {
            for x in 0..4 {
                let mut tetromino_chars = tetromino_string.chars();
                let tetromino_string_idx =
                    self.tetromino
                        .unwrap()
                        .xy_to_idx(x, y, self.tetromino_rotation);
                if tetromino_chars.nth(tetromino_string_idx).unwrap() == '.' {
                    let grid_idx =
                        self.tetromino_xy_to_grid_idx(self.tetromino_x + x, self.tetromino_y + y);
                    self.grid[grid_idx].0 = true;
                    self.grid[grid_idx].1 = self.tetromino_color;
                }
            }
        }

        // pick new tetromino
        self.tetromino = Some(rand::random::<Tetromino>());
        self.tetromino_color = Some(rand::random::<TetrominoColor>());
        self.tetromino_x = 4 + SCREEN_X_OFFSET;
        self.tetromino_y = 0;
        self.tetromino_rotation = 0;
    }

    /// Given an (x, y) position of the tetromino, returns the index of that point in the actual
    /// gamespace grid used for debris
    pub fn tetromino_xy_to_grid_idx(&self, x: usize, y: usize) -> usize {
        (y * self.width) + (x - SCREEN_X_OFFSET)
    }

    /// Returns true if a given (x, y) point of the gamespace has debris on it
    pub fn has_debris_at(&self, x: usize, y: usize) -> bool {
        let idx = y * self.width + x;
        if idx <= self.width * self.height {
            return self.grid[idx].0;
        } else {
            panic!(
                "({}, {}) passed to has_debris_at, outside of gamespace",
                x, y
            );
        }
    }

    /// Returns a TetrominoColor corresponding to the point on the grid
    pub fn get_color_at(&self, x: usize, y: usize) -> Option<TetrominoColor> {
        let idx = y * self.width + x;
        if idx <= self.width * self.height {
            return self.grid[idx].1;
        } else {
            panic!(
                "({}, {}) passed to get_color_at, outside of gamespace",
                x, y
            );
        }
    }

    pub fn rotate_tetromino(&mut self) {
        self.tetromino_rotation = (self.tetromino_rotation + 1) % 4;
    }

    pub fn move_tetromino_down(&mut self) {
        self.tetromino_y += 1;
    }

    pub fn move_tetromino_left(&mut self) {
        self.tetromino_x -= 1;
    }

    pub fn move_tetromino_right(&mut self) {
        self.tetromino_x += 1;
    }
}
