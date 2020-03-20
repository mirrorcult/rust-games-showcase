use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
pub enum TetrominoType {
    I, // straight
    O, // square
    T, 
    J, // reverse L
    L,
    S, // reverse Z
    Z,
}

impl Distribution<TetrominoType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrominoType {
        match rng.gen_range(0, 6) {
            0 => TetrominoType::I,
            1 => TetrominoType::O,
            2 => TetrominoType::T,
            3 => TetrominoType::J,
            4 => TetrominoType::L,
            5 => TetrominoType::S,
            _ => TetrominoType::Z
        }
    }
}

/// Basic struct representing a Tetromino, the pieces in Tetris.
/// Each tetromino is represented by a 16-long string, which can be
/// mapped to a 4x4 array in which the tetromino resides.
/// For instance, the string for an I tetromino might be
/// `"XX.XXX.XXX.XXX.X"`, which would look like:
///
/// ```
///XX.X
///XX.X
///XX.X
///XX.X
/// ```
pub struct Tetromino {
    rotation: usize, // 0 = 0*, 1 = 90*, 2 = 180*, 3 = 270*
    x: usize, // (x, y) position of the top corner of the 4x4 grid
    y: usize, // ''
    shape: TetrominoType,
}

impl Tetromino {
    pub fn new(x: usize, y: usize, shape: TetrominoType) -> Tetromino {
        Tetromino {
            rotation: 0,
            x,
            y,
            shape
        }
    }

    /// Given an X and Y coordinate, returns the string index of the tetromino's hitbox that corresponds.
    /// Accounts for rotation.
    ///
    /// Below is what the array looks like for a 0* rotation and 90* rotation: you can see where the formulas come from.
    /// ```
    /// 0 1 2 3     12 8 4 0
    /// 4 5 6 7     13 9 5 1
    /// 8 9 10 11   14 10 6 2
    /// 12 13 14 15 15 11 7 3
    /// ```
    pub fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        match self.rotation {
            0 => { // 0
                y * 4 + x
            },
            1 => { // 90
                12 + y - (x * 4)
            },
            2 => { // 180
                15 - (y * 4) - x
            },
            3 => {
                3 - y + (x * 4)
            },
            _ => 0
        }
    }

    /// Returns a String of 16 length representing the tetromino.
    /// Rotation is not accounted for here, only shape, as the renderer itself
    /// will rotate pieces, and rotated shapes indexes using `xy_to_idx`.
    pub fn as_string(&self) -> String {
        let mut result: String = String::new();

        match self.shape {
            TetrominoType::I => {
                result.push_str("XX.X");
                result.push_str("XX.X");
                result.push_str("XX.X");
                result.push_str("XX.X");
            },
            TetrominoType::O => {
                result.push_str("XXXX");
                result.push_str("X..X");
                result.push_str("X..X");
                result.push_str("XXXX");
            },
            TetrominoType::T => {
                result.push_str("XXXX");
                result.push_str("X.XX");
                result.push_str("...X");
                result.push_str("XXXX");
            },
            TetrominoType::J => {
                result.push_str("XX.X");
                result.push_str("XX.X");
                result.push_str("X..X");
                result.push_str("XXXX");
            },
            TetrominoType::L => {
                result.push_str("X.XX");
                result.push_str("X.XX");
                result.push_str("X..X");
                result.push_str("XXXX");
            },
            TetrominoType::S => {
                result.push_str("XXXX");
                result.push_str("X..X");
                result.push_str("..XX");
                result.push_str("XXXX");
            },
            TetrominoType::Z => {
                result.push_str("XXXX");
                result.push_str("X..X");
                result.push_str("XX..");
                result.push_str("XXXX");
            }
        }

        result
    }

    pub fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn move_down(&mut self) {
        self.y -= 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }
}
