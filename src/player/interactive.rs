use crate::wordle::score::{Score, Response};

use std::io::stdin;
use std::vec::Vec;

use super::Player;

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn scan<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}
pub struct InteractivePlayer {
    guesses: Vec<Score>,
    scanner: Scanner,
}

impl InteractivePlayer {
    pub fn new() -> InteractivePlayer {
        return InteractivePlayer {
            guesses: Vec::new(),
            scanner: Scanner::default(),
        };
    }
}
impl Player for InteractivePlayer {
    fn guess_word(&mut self) -> String {
        return self.scanner.scan();
    }

    fn add_score(&mut self, new_score: &Score) {
        self.guesses.push(new_score.clone());
        println!("Got Reponse:");
        println!("\t{}",new_score.guess());
        print!("\t");
        for r in new_score.responses().iter() {
            match r {
                Response::Wrong => print!("_"),
                Response::Misplaced => print!("+"),
                Response::Correct => print!("*"),
            }
        }
        println!();
        println!();
    }
}
