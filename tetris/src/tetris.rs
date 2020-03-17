/// Struct holding information and methods relating to the current tetris game.
pub struct TetrisGame {
    pub width: usize,
    pub height: usize,
    pub tetromino_position: (usize, usize), // topleft coord of tetromino
    grid: Vec<bool>, // true -> contains placed tetromino block, false -> doesnt
}

impl TetrisGame {
    pub fn new(width: usize, height: usize) -> TetrisGame {
        TetrisGame {
            width,
            height,
            tetromino_position: (4, 0),
            grid: Vec::with_capacity(width * height)
        }
    }
}
