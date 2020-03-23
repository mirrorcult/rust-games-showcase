use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use termion::color;

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
    pub fn as_terminal_color(&self) -> &'static str {
        match self {
            TetrominoColor::Red => color::Red.fg_str(),
            TetrominoColor::Magenta => color::Magenta.fg_str(),
            TetrominoColor::Blue => color::LightBlue.fg_str(),
            TetrominoColor::Yellow => color::Yellow.fg_str(),
            TetrominoColor::Green => color::Green.fg_str(),
            TetrominoColor::Cyan => color::Cyan.fg_str(),
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
