use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use termion::color::{self, Color};

#[derive(Debug, Copy, Clone)]
pub enum TetrominoColor {
    Red, 
    Magenta, 
    Blue, 
    Yellow, 
    Green,
    Cyan, 
}

impl TetrominoColor {
    pub fn as_terminal_color(&self) -> Box<dyn Color> {
        match self {
            TetrominoColor::Red => Box::new(color::Red),
            TetrominoColor::Magenta => Box::new(color::Magenta),
            TetrominoColor::Blue => Box::new(color::Blue),
            TetrominoColor::Yellow => Box::new(color::Yellow),
            TetrominoColor::Green => Box::new(color::Green),
            TetrominoColor::Cyan => Box::new(color::Cyan),
        }
    }
}

impl Distribution<TetrominoColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrominoColor {
        match rng.gen_range(0, 6) {
            0 => TetrominoColor::Red,
            1 => TetrominoColor::Magenta,
            2 => TetrominoColor::Blue,
            3 => TetrominoColor::Yellow,
            4 => TetrominoColor::Green,
            _ => TetrominoColor::Cyan,
        }
    }
}