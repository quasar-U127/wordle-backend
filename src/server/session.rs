use std::fmt;

use crate::wordle_core::{GameState, Wordle};

#[derive(PartialEq, Eq, Hash, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct SessionId(u64);
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Session {
    id: SessionId,
    state: GameState,
}

impl Session {
    pub fn new(id: SessionId, wordle: &Wordle) -> Session {
        return Session {
            id,
            state: wordle.generator().new_game(),
        };
    }
    pub fn id(&self) -> SessionId {
        return self.id;
    }
    pub fn state_mut(&mut self) -> &mut GameState {
        return &mut self.state;
    }
    pub fn state(&self) -> &GameState {
        return &self.state;
    }
}

impl SessionId {
    pub fn new(id: u64) -> SessionId {
        return SessionId(id);
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
