use cozy_chess::{Board, Move};

pub struct Engine {
    pub board: Board,
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
        let mut found_move = None;

        self.board.generate_moves(|moves| {
            for mv in moves {
                found_move = Some(mv);
                return true;
            }
            true
        });

        Some((&self.board, found_move?))
    }
}
