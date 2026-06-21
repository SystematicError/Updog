use crate::evaluate::evaluate;
use cozy_chess::{Board, Move};

pub fn search(board: &Board) -> Option<Move> {
    let mut best_move = None;

    _search(board, i32::MIN + 1, i32::MAX, true, &mut best_move, 5);

    best_move
}

fn _search(
    board: &Board,
    mut alpha: i32,
    beta: i32,
    root: bool,
    best_move: &mut Option<Move>,
    depth: u8,
) -> i32 {
    if depth == 0 {
        return evaluate(board);
    }

    let mut best_score = i32::MIN + 1;

    board.generate_moves(|moves| {
        for mv in moves {
            let mut new_board = board.clone();
            new_board.play_unchecked(mv);

            let score = -_search(&new_board, -beta, -alpha, false, best_move, depth - 1);

            if score > best_score {
                best_score = score;

                if score > alpha {
                    alpha = score;
                }

                if root {
                    *best_move = Some(mv);
                }
            }

            if !root && score >= beta {
                best_score = beta;
                return true;
            }
        }

        false
    });

    best_score
}
