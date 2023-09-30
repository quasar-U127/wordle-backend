// use std::{collections::HashMap, path::Path};

// use crate::utils::get_word_list;
// use crate::wordle::score::Score;

// pub struct Player {
//     guesses: Vec<Score>,
//     active_set: HashMap<String, f32>,
// }

// use std::io::stdin;
// use std::vec::Vec;

// #[derive(Default)]
// struct Scanner {
//     buffer: Vec<String>,
// }

// impl Scanner {
//     fn scan<T: std::str::FromStr>(&mut self) -> T {
//         loop {
//             if let Some(token) = self.buffer.pop() {
//                 return token.parse().ok().expect("Failed parse");
//             }
//             let mut input = String::new();
//             stdin().read_line(&mut input).expect("Failed read");
//             self.buffer = input.split_whitespace().rev().map(String::from).collect();
//         }
//     }
// }


// impl Player {
//     pub fn new(guess_list_file: &Path) -> Player {
//         return Player {
//             guesses: Vec::new(),
//             active_set: get_word_list(guess_list_file)
//                 .iter()
//                 .map(|s| (s.clone(), 1f32))
//                 .collect(),
//         };
//     }

//     pub fn guess_word(&self)->String {
//         todo!()
//     }

//     pub fn add_score(new_score:&Score){

//     }

// }
