use cozy_chess::{Board, Color, Move, Piece, Rank, Square};

use crate::evaluate::piece_value;

fn mvv_lva_score(board: &Board, mv: Move) -> i32 {
    let attacker = board.piece_on(mv.from).unwrap();

    let victim = if let Some(file) = board.en_passant() {
        let rank = match board.side_to_move() {
            Color::White => Rank::Sixth,
            Color::Black => Rank::Third,
        };

        if mv.to == Square::new(file, rank) && attacker == Piece::Pawn {
            Piece::Pawn
        } else {
            match board.piece_on(mv.to) {
                Some(piece) => piece,
                None => return 0,
            }
        }
    } else {
        match board.piece_on(mv.to) {
            Some(piece) => piece,
            None => return 0,
        }
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
