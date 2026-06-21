use cozy_chess::{Board, Color, Piece};

pub fn evaluate(board: &Board) -> i32 {
    let mut material_score = 0;

    for (piece, value) in [
        (Piece::Queen, 900),
        (Piece::Rook, 500),
        (Piece::Knight, 300),
        (Piece::Bishop, 300),
        (Piece::Pawn, 100),
    ] {
        let white_count = board.colored_pieces(Color::White, piece).len() as i32;
        let black_count = board.colored_pieces(Color::Black, piece).len() as i32;

        material_score += value * (white_count - black_count)
    }

    let perspective = match board.side_to_move() {
        Color::White => 1,
        Color::Black => -1,
    };

    material_score * perspective
}
