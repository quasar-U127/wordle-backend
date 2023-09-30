use crate::wordle::score::Score;

pub mod ai;
pub mod interactive;

pub trait Player {
    fn guess_word(&mut self) -> String;
    fn add_score(&mut self, new_score: &Score);
}
