use super::judge::Judgement;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GameState {
    answer: String,
    judgements: Vec<Judgement>,
    guesses_allowed: i64,
}

impl GameState {
    pub fn new(answer: &String, guesses_allowed: i64) -> GameState {
        return GameState {
            answer: answer.clone(),
            judgements: Vec::new(),
            guesses_allowed,
        };
    }
    pub fn add_judgement(&mut self, judgement: Judgement) {
        self.judgements.push(judgement);
    }

    pub fn guesses_allowed(&self) -> i64 {
        return self.guesses_allowed;
    }
    pub fn guesses_made(&self) -> i64 {
        return self.judgements.len() as i64;
    }

    pub fn get_judgement(&self, n: usize) -> Judgement {
        return self.judgements[n].clone();
    }

    pub fn get_answer(&self) -> String {
        return self.answer.clone();
    }
}
