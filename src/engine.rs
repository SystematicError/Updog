use crate::position::Position;
use crate::search::search;
use cozy_chess::{Board, Move};

pub struct Engine {
    position: Position,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            position: Position::default(),
        }
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn best_move(&mut self) -> Option<(&Board, Move)> {
        search(&self.position).map(|mv| (self.position.board(), mv))
    }
}
