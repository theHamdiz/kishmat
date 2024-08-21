use types::{Piece, Color, Board};

const PIECE_VALUES: [i32; 6] = [100, 320, 330, 500, 900, 20000]; // Pawn, Knight, Bishop, Rook, Queen, King

#[inline(always)]
pub fn evaluate_material(board: &Board) -> i32 {
    let mut score = 0;

    for color in [Color::White, Color::Black].iter() {
        for piece in [Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen].iter() {
            let piece_count = board.piece_count(*piece);
            score += piece_count as i32 * PIECE_VALUES[*piece as usize] * if *color == Color::White { 1 } else { -1 };
        }
    }

    score
}
