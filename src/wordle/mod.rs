pub mod score;

use crate::utils;
use rand::seq::SliceRandom;
use std::path::Path;

use self::score::Score;

pub struct Wordle {
    answer: String,
    guesses: Vec<Score>,
}

impl Wordle {
    pub fn new(answer_list_file: &Path) -> Wordle {
        let words = utils::get_word_list(answer_list_file);
        return Wordle {
            answer: words
                .iter()
                .collect::<Vec<&String>>()
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            guesses: Vec::new(),
        };
    }

    pub fn guess(&mut self, guess_word: &String) -> Score {
        let guess = Score::new(guess_word, &self.answer);
        self.guesses.push(guess.clone());
        return guess;
    }

    pub fn get_score(&self, n: usize) -> Score {
        return self.guesses[n].clone();
    }

    pub fn get_answer(&self)->&String{
        return &self.answer;
    }

}
