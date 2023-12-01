pub mod game;
pub mod judge;

pub use judge::JudgeResponse;

use crate::utils;
use rand::seq::SliceRandom;
use std::path::Path;

pub use self::game::GameState;
pub struct WordleGenerator {
    answers: Vec<String>,
}

pub struct Wordle {
    generator: WordleGenerator,
    jury: judge::Jury,
}

impl WordleGenerator {
    pub fn new(answers: Vec<String>) -> WordleGenerator {
        return WordleGenerator { answers };
    }

    pub fn new_game(&self) -> GameState {
        let answer = self.answers.choose(&mut rand::thread_rng()).unwrap();
        return GameState::new(answer, 6);
    }
}

impl Wordle {
    pub fn new(answer_list_file: &Path, valid_guess_file: &Path) -> Wordle {
        let words = utils::get_word_list(answer_list_file);
        let valid_guesses = utils::get_word_list(valid_guess_file);
        return Wordle {
            generator: WordleGenerator::new(words),
            jury: judge::Jury::new(valid_guesses.iter().map(|s| s.clone()).collect()),
        };
    }

    pub fn generator(&self) -> &WordleGenerator {
        return &self.generator;
    }
    pub fn jury(&self) -> &judge::Jury {
        return &self.jury;
    }
}
