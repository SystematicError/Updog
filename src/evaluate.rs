use cozy_chess::{Board, Color, Piece};

#[inline(always)]
pub fn piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Queen => 900,
        Piece::Rook => 500,
        Piece::Knight => 300,
        Piece::Bishop => 300,
        Piece::Pawn => 100,
        Piece::King => 0,
    }
}

pub fn evaluate(board: &Board) -> i32 {
    let mut material_score = 0;

    for piece in [
        Piece::Queen,
        Piece::Rook,
        Piece::Knight,
        Piece::Bishop,
        Piece::Pawn,
    ] {
        let value = piece_value(piece);

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
