use cozy_chess::{Board, GameStatus, IllegalMoveError, Move};

#[derive(Debug, Clone)]
pub struct Position {
    board: Board,
    history: Vec<u64>,
}

impl Position {
    pub fn new(board: Board) -> Self {
        Self {
            history: vec![board.hash()],
            board: board,
        }
    }

    pub fn default() -> Self {
        Self::new(Board::default())
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn try_play(&mut self, mv: Move) -> Result<(), IllegalMoveError> {
        self.board.try_play(mv)?;
        self.history.push(self.board.hash());
        Ok(())
    }

    pub fn play_unchecked(&mut self, mv: Move) {
        self.board.play_unchecked(mv);
        self.history.push(self.board.hash());
    }

    pub fn status(&self) -> GameStatus {
        let status = self.board.status();

        if status != GameStatus::Ongoing {
            return status;
        }

        let current_hash = self.board().hash();

        let repetitions = self
            .history
            .iter()
            .rev()
            .take(self.board.halfmove_clock() as usize + 1)
            .step_by(2)
            .filter(|&&hash| hash == current_hash)
            .count();

        if repetitions >= 3 {
            return GameStatus::Drawn;
        }

        GameStatus::Ongoing
    }
}
