use crate::wordle_core::game;
use std::collections::HashSet;

use super::GameState;
#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Decision {
    Wrong,
    Misplaced,
    Correct,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Judgement {
    guess: String,
    decisions: Vec<Decision>,
}

pub struct Jury {
    valid_guesses: HashSet<String>,
}

impl Judgement {
    pub fn new(word: &String, reference: &String) -> Judgement {
        return Judgement {
            guess: word.clone(),
            decisions: Judgement::decide(word, reference),
        };
    }

    pub fn guess(&self) -> &String {
        return &self.guess;
    }

    pub fn responses(&self) -> &Vec<Decision> {
        return &self.decisions;
    }

    pub fn verdict(&self) -> bool {
        return self.decisions.iter().all(|r| *r == Decision::Correct);
    }

    pub fn decide(guess_word: &String, answer: &String) -> Vec<Decision> {
        // use crate::wordle::Response;
        let mut responses = vec![Decision::Wrong; answer.len()];
        let guess_chars: Vec<char> = guess_word.to_lowercase().chars().collect();
        let answer_chars: Vec<char> = answer.to_lowercase().chars().collect();
        let mut letters = [0; 26];
        for c in answer_chars.iter() {
            letters[*c as usize - 'a' as usize] += 1;
        }
        for i in 0..guess_chars.len() {
            if guess_chars[i] == answer_chars[i] {
                letters[guess_chars[i] as usize - 'a' as usize] -= 1;
                responses[i] = Decision::Correct;
            }
        }
        for i in 0..guess_chars.len() {
            if responses[i] == Decision::Wrong
                && letters[guess_chars[i] as usize - 'a' as usize] > 0
            {
                letters[guess_chars[i] as usize - 'a' as usize] -= 1;
                responses[i] = Decision::Misplaced;
            }
        }
        return responses;
    }
}

impl Jury {
    pub fn new(valid_guesses: HashSet<String>) -> Jury {
        return Jury { valid_guesses };
    }
    pub fn decide(&self, word: &String, state: &mut GameState) -> Option<Judgement> {
        if !self.valid_guesses.contains(word) || state.guesses_made() >= state.guesses_allowed() {
            return None;
        }
        let judgement = Judgement::new(word, &state.get_answer());
        state.add_judgement(judgement.clone());
        return Some(judgement);
    }
}
