use crate::wordle_core::judge::Judgement;

pub mod ai;
pub mod interactive;

pub trait Player {
    fn guess_word(&mut self) -> String;
    fn add_score(&mut self, new_score: &Judgement);
}
