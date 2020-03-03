use std::{fmt, io::stdin};

use self::{Color::*, Piece::*};

#[derive(Debug, Clone, Copy)]
enum Color {
    White,
    Black,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            White => write!(f, "W"),
            Black => write!(f, "B"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Piece {
    Pawn(Color),
    Bishop(Color),
    Knight(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
    Empty,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pawn(col) => write!(f, "{}P", col.to_string()),
            Rook(col) => write!(f, "{}R", col.to_string()),
            Knight(col) => write!(f, "{}N", col.to_string()),
            Bishop(col) => write!(f, "{}B", col.to_string()),
            Queen(col) => write!(f, "{}Q", col.to_string()),
            King(col) => write!(f, "{}K", col.to_string()),
            Empty => write!(f, "  "),
        }
    }
}

struct Chessboard {
    pieces: [Piece; 64],
    #[allow(dead_code)]
    white_graveyard: Vec<Piece>,
    #[allow(dead_code)]
    black_graveyard: Vec<Piece>,
}

impl Chessboard {
    fn new() -> Chessboard {
        let mut pieces: [Piece; 64] = [Empty; 64];
        pieces[0] = Rook(Black);
        pieces[1] = Knight(Black);
        pieces[2] = Bishop(Black);
        pieces[3] = Queen(Black);
        pieces[4] = King(Black);
        pieces[5] = Bishop(Black);
        pieces[6] = Knight(Black);
        pieces[7] = Rook(Black);

        for x in 8..16 {
            pieces[x] = Pawn(Black);
        }
        for x in 48..56 {
            pieces[x] = Pawn(White);
        }
        pieces[56] = Rook(White);
        pieces[57] = Knight(White);
        pieces[58] = Bishop(White);
        pieces[59] = Queen(White);
        pieces[60] = King(White);
        pieces[61] = Bishop(White);
        pieces[62] = Knight(White);
        pieces[63] = Rook(White);

        Chessboard {
            pieces,
            white_graveyard: Vec::new(),
            black_graveyard: Vec::new(),
        }
    }

    fn as_string(&self) -> String {
        let mut result = String::new();
        result = format!("\n  0    1    2    3    4    5    6    7\n{}", result);
        for x in 0..8 {
            result = format!("{}{}\n", result, (0..41).map(|_| "-").collect::<String>());
            for y in (8 * x)..((8 * x) + 8) {
                if y % 8 == 0 {
                    result = format!("{}|", result);
                }
                result = format!("{} {} |", result, self.pieces[y].to_string());
            }
            result = format!("{} {}\n", result, x);
        }
        result = format!("{}{}\n", result, (0..41).map(|_| "-").collect::<String>());
        format!("{}\n{}\n{}", result, "WG: ", "BG: ")
    }

    fn move_piece(&mut self, start: (usize, usize), end: (usize, usize)) {
        let start_idx = self.rowcol_to_idx(start);
        let end_idx = self.rowcol_to_idx(end);

        if self.is_legal_move(start, end) {
            self.pieces[end_idx] = self.pieces[start_idx];
            self.pieces[start_idx] = Empty;
        } else {
            println!("Illegal move from {} to {}", start_idx, end_idx);
        }
    }

    fn rowcol_to_idx(&self, rowcol: (usize, usize)) -> usize {
        let (x, y) = rowcol;
        (8 * x) + y
    }

    fn is_legal_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let (sr, sc) = start;
        let (er, ec) = end;
        match self.pieces[self.rowcol_to_idx((sr, sc))] {
            Pawn(_) => {
                (sr == er - 1) // todo add check for en passant + taking + start of game +2
            }
            Rook(_) => (sr == er && sc != sc) || (sc == ec && sr != er),
            Bishop(_) => (ec as isize - sc as isize).abs() == (er as isize - sr as isize).abs(),
            _ => true,
        }
    }
}

fn main() {
    let mut testboard = Chessboard::new();
    loop {
        print!("\x1B[2J");
        println!("{}", testboard.as_string());

        let mut start = String::new();
        let mut end = String::new();
        stdin().read_line(&mut start).expect("readline failed");
        stdin().read_line(&mut end).expect("readline failed");
        let mut start_iter = start.split_whitespace();
        let mut end_iter = end.split_whitespace();
        let start_rowcol = (
            start_iter.next().unwrap().parse().unwrap(),
            start_iter.next().unwrap().parse().unwrap(),
        );
        let end_rowcol = (
            end_iter.next().unwrap().parse().unwrap(),
            end_iter.next().unwrap().parse().unwrap(),
        );
        testboard.move_piece(start_rowcol, end_rowcol);
    }
}
