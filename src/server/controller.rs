use std::path::Path;

use actix_web::web;

use super::session;
use crate::server::model;
use crate::wordle_core;
pub struct Controller {
    store: model::Model,
    driver: wordle_core::Wordle,
}

impl Controller {
    pub fn new() -> Controller {
        return Controller {
            store: model::Model::new(Path::new("data/games")),
            driver: wordle_core::Wordle::new(
                Path::new("data/wordle-nyt-answers-alphabetical.txt"),
                Path::new("data/valid-wordle-words.txt"),
            ),
        };
    }
    pub fn store(&self) -> &model::Model {
        return &self.store;
    }
    pub fn driver(&self) -> &wordle_core::Wordle {
        return &self.driver;
    }
    pub fn new_game(&self) -> session::SessionId {
        let id = self.store.new_id();
        let sess = session::Session::new(id, self.driver());
        let _ = self.store().store_session(sess);
        return id;
    }
    pub fn guess(&self, id: session::SessionId, word: &String) -> Option<wordle_core::Judgement> {
        let mut sess = self.store.get_session(id);
        let jury = self.driver.jury();
        let judgement = jury.decide(word, sess.state_mut());
        let _ = self.store().store_session(sess);
        return judgement;
    }
}

// pub async fn guess(self: &WordleData, id: session::SessionId) {
//     let mut sess = data.store.get_session(id);
//     let judgement = sess.guess(valid_guess_word)
// }
