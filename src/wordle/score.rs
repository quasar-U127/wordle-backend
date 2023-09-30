#[derive(Clone, Copy, PartialEq)]
pub enum Response {
    Wrong,
    Misplaced,
    Correct,
}

#[derive(Clone)]
pub struct Score {
    guess: String,
    responses: Vec<Response>,
}

impl Score {
    pub fn new(word: &String, reference: &String) -> Score {
        return Score {
            guess: word.clone(),
            responses: Score::calculate_response(word, reference),
        };
    }

    pub fn guess(&self) -> &String {
        return &self.guess;
    }

    pub fn responses(&self) -> &Vec<Response> {
        return &self.responses;
    }

    pub fn correct(&self)->bool{
        return self.responses.iter().all(|r| *r==Response::Correct);
    }

    pub fn calculate_response(guess_word: &String, answer: &String) -> Vec<Response> {
        // use crate::wordle::Response;
        let mut responses = vec![Response::Wrong; answer.len()];
        let guess_chars: Vec<char> = guess_word.to_lowercase().chars().collect();
        let answer_chars: Vec<char> = answer.to_lowercase().chars().collect();
        let mut letters = [0; 26];
        for c in answer_chars.iter() {
            letters[*c as usize - 'a' as usize] += 1;
        }
        for i in 0..guess_chars.len() {
            if guess_chars[i] == answer_chars[i] {
                letters[guess_chars[i] as usize - 'a' as usize] -= 1;
                responses[i] = Response::Correct;
            }
        }
        for i in 0..guess_chars.len() {
            if responses[i] == Response::Wrong
                && letters[guess_chars[i] as usize - 'a' as usize] > 0
            {
                letters[guess_chars[i] as usize - 'a' as usize] -= 1;
                responses[i] = Response::Misplaced;
            }
        }
        return responses;
    }
}
