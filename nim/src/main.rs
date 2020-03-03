use rand::Rng;
use std::{
    io::{stdin, stdout, Write},
    process::exit,
    thread::sleep,
    time::Duration,
};

struct NimBoard {
    sticks: i32,
}

impl NimBoard {
    fn new() -> NimBoard {
        NimBoard { sticks: 12 }
    }

    /// Take `sticks_to_take` sticks from the board
    fn take(&mut self, sticks_to_take: i32) {
        if sticks_to_take > 0 && sticks_to_take < 4 {
            self.sticks -= sticks_to_take;
        } else {
            let sticks_random = rand::thread_rng().gen_range(1, 4);
            println!(
                "You can't take {} sticks! Randomly chose {} sticks instead.\n",
                sticks_to_take, sticks_random
            );
            sleep(Duration::from_secs(2));
            self.sticks -= sticks_random;
        }
    }

    /// Returns string that represents state of board
    fn as_string(&self) -> String {
        if self.sticks <= 0 {
            return String::from(" ");
        }
        format!(
            "{} ({})",
            (0..self.sticks).map(|_| " | ").collect::<String>(),
            self.sticks
        )
    }

    fn print(&self) {
        print!("\x1B[2J");
        println!("{}", self.as_string());
        stdout().flush().expect("Failed to flush");
    }
}

fn input(message: &str) -> String {
    print!("{}", message);
    stdout().flush().expect("Failed to flush");
    let mut ret = String::new();
    stdin()
        .read_line(&mut ret)
        .expect("Failed to read from stdin");
    ret.split_whitespace().collect::<String>()
}

fn main() {
    println!("You are now playing NIM.");
    let mut player_first = false;
    let confirm = input("Start as first player? [y/n] ");
    if confirm.as_str() == "y" {
        player_first = true;
    }

    let mut board = NimBoard::new();
    if player_first {
        board.print();
        let player_sticks = input("How many sticks to take? ").parse().unwrap_or(0);
        board.take(player_sticks);
    }

    loop {
        board.print();
        let mut ai_sticks = board.sticks % 4; // always take 4 - (player_sticks)
        if ai_sticks == 0 {
            // if we started at a multiple of four, we cant take 0
            ai_sticks = rand::thread_rng().gen_range(1, 4);
        }
        println!("AI is taking: {} sticks", ai_sticks);
        board.take(ai_sticks);

        if board.sticks <= 0 {
            println!("AI wins! Too bad.");
            exit(0);
        }

        sleep(Duration::from_secs(2));
        board.print();
        let player_sticks = input("How many sticks to take? ").parse().unwrap_or(0);
        board.take(player_sticks);

        if board.sticks <= 0 {
            println!("Player won! Good job.");
            exit(0);
        }
    }
}
