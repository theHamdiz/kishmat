use types::{Board, Color, Piece};

const PAWN_PSQT: [i32; 64] = [
    0,  5,  5, -5, -5,  5,  5,  0,
    0, 10, -5,  0,  0, -5, 10,  0,
    0, 10, 10, 20, 20, 10, 10,  0,
    5, 10, 20, 25, 25, 20, 10,  5,
    10, 20, 20, 30, 30, 20, 20, 10,
    50, 50, 50, 50, 50, 50, 50, 50,
    90, 90, 90, 90, 90, 90, 90, 90,
    0,  0,  0,  0,  0,  0,  0,  0,
];

#[inline(always)]
pub fn evaluate_psqt(board: &Board) -> i32 {
    let mut score = 0;

    for color in [Color::White, Color::Black].iter() {
        for piece in [Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen].iter() {
            let piece_squares = board.piece_squares(*piece, *color );
            for &square in piece_squares.iter() {
                let index = if *color == Color::White { square.to_index() } else { 63 - square.to_index() };
                score += psqt_value(*piece, index) * if *color == Color::White { 1 } else { -1 };
            }
        }
    }

    score
}

fn psqt_value(piece: Piece, index: usize) -> i32 {
    match piece {
        Piece::Pawn => PAWN_PSQT[index],
        _ => 0,
    }
}
