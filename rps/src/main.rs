use self::Move::*;
use std::fmt;

#[derive(Debug)]
struct Player {
    name: String,
    play: Move,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, who played {:?}", self.name, self.play)
    }
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn get_winner(p1: Player, p2: Player) -> Option<Player> {
    match (p1.play, p2.play) {
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Some(p1),
        (Rock, Paper) | (Scissors, Rock) | (Paper, Scissors) => Some(p2),
        _ => None,
    }
}

fn main() {
    let player1 = Player {
        name: String::from("Jeff"),
        play: Rock,
    };
    let player2 = Player {
        name: String::from("Maria"),
        play: Scissors,
    };
    let player3 = Player {
        name: String::from("Guy"),
        play: Paper,
    };

    let who_won = get_winner(player1, player2).unwrap();
    let who_won_champs = get_winner(who_won, player3).unwrap();
    println!("{}", who_won_champs);
}
