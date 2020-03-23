use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Copy, Clone)]
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
pub enum Tetromino {
    I, // straight
    O, // square
    T,
    J, // reverse L
    L,
    S, // reverse Z
    Z,
}

impl Distribution<Tetromino> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tetromino {
        match rng.gen_range(0, 6) {
            0 => Tetromino::I,
            1 => Tetromino::O,
            2 => Tetromino::T,
            3 => Tetromino::J,
            4 => Tetromino::L,
            5 => Tetromino::S,
            _ => Tetromino::Z,
        }
    }
}

impl Tetromino {
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
    pub fn xy_to_idx(&self, x: usize, y: usize, r: usize) -> usize {
        match r {
            0 => {
                // 0
                y * 4 + x
            }
            1 => {
                // 90
                12 + y - (x * 4)
            }
            2 => {
                // 180
                15 - (y * 4) - x
            }
            3 => 3 - y + (x * 4),
            _ => 0,
        }
    }

    /// Returns a String of 16 length representing the tetromino.
    /// Rotation is not accounted for here, only shape, as the renderer itself
    /// will rotate pieces, and rotated shapes indexes using `xy_to_idx`.
    pub fn as_string(&self) -> String {
        let mut result: String = String::new();

        match self {
            Tetromino::I => {
                result.push_str("XX.X");
                result.push_str("XX.X");
                result.push_str("XX.X");
                result.push_str("XX.X");
            }
            Tetromino::O => {
                result.push_str("XXXX");
                result.push_str("X..X");
                result.push_str("X..X");
                result.push_str("XXXX");
            }
            Tetromino::T => {
                result.push_str("XXXX");
                result.push_str("X.XX");
                result.push_str("...X");
                result.push_str("XXXX");
            }
            Tetromino::J => {
                result.push_str("XX.X");
                result.push_str("XX.X");
                result.push_str("X..X");
                result.push_str("XXXX");
            }
            Tetromino::L => {
                result.push_str("X.XX");
                result.push_str("X.XX");
                result.push_str("X..X");
                result.push_str("XXXX");
            }
            Tetromino::S => {
                result.push_str("XXXX");
                result.push_str("X..X");
                result.push_str("..XX");
                result.push_str("XXXX");
            }
            Tetromino::Z => {
                result.push_str("XXXX");
                result.push_str("X..X");
                result.push_str("XX..");
                result.push_str("XXXX");
            }
        }

        result
    }
}
