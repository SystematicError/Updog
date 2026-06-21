use crate::search::search;
use cozy_chess::{Board, Move};

pub struct Engine {
    board: Board,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
        }
    }

    pub fn set_board(&mut self, board: Board) {
        self.board = board
    }

    pub fn best_move(&mut self) -> Option<(&Board, Move)> {
        search(&self.board).map(|m| (&self.board, m))
    }
}
