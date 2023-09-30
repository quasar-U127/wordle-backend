use std::path::Path;

use player::Player;

mod utils;
mod wordle;
mod player;

// use std::vec::Vec;
// use std::collections::{HashSet, HashMap};

// struct GameState{
//     guesses:Vec<Guess>,
//     activeSet:HashMap<String,f32>
// }
// struct Wordle{
//     dictionary:HashSet<String>
//     answer:
// }


fn main() {
    let mut game = wordle::Wordle::new(Path::new("data/wordle-nyt-answers-alphabetical.txt"));
    let mut pulsar = player::interactive::InteractivePlayer::new();
    // println!("{}",wordle_a.get_answer());

    for _ in 0..6{
        let guess_word = pulsar.guess_word();
        let cur_score = game.guess(&guess_word);
        pulsar.add_score(&cur_score);
    }
    
}
