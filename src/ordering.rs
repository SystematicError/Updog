use cozy_chess::{Board, Move};

use crate::captures::capture_pair;
use crate::evaluate::piece_value;

fn mvv_lva_score(board: &Board, mv: Move) -> i32 {
    let (attacker, victim) = match capture_pair(board, mv) {
        Some(pair) => pair,
        None => return 0,
    };

    10 * piece_value(victim) - piece_value(attacker)
}

pub fn generate_ordered_moves(board: &Board) -> impl Iterator<Item = Move> {
    let mut ordered_moves = Vec::new();

    board.generate_moves(|moves| {
        ordered_moves.extend(moves);
        false
    });

    ordered_moves.sort_unstable_by_key(|&mv| std::cmp::Reverse(mvv_lva_score(board, mv)));

    ordered_moves.into_iter()
}
