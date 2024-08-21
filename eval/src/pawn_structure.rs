use types::{Bitboard, Square, Color, Board, clear_bit, get_lsb};

const ISOLATED_PAWN_PENALTY: i32 = -10;
const DOUBLED_PAWN_PENALTY: i32 = -20;
const PASSED_PAWN_BONUS: i32 = 50;

#[inline(always)]
pub fn evaluate_pawn_structure(board: &Board) -> i32 {
    let mut score = 0;

    for &color in &[Color::White, Color::Black] {
        let pawns = board.pawns(color);
        let color_multiplier = if color == Color::White { 1 } else { -1 };

        // Evaluate isolated pawns
        let mut pawn_bitboard = pawns;
        while pawn_bitboard != 0 {
            let pawn_square_index = get_lsb(pawn_bitboard);  // Get the index of the least significant bit
            let pawn_square = Square::from_index(pawn_square_index);  // Convert to Square
            if is_isolated(board, color, pawn_square) {
                score += ISOLATED_PAWN_PENALTY * color_multiplier;
            }
            clear_bit(&mut pawn_bitboard, pawn_square_index);  // Clear the processed bit
        }

        // Evaluate doubled pawns
        score += evaluate_doubled_pawns(board, color) * color_multiplier;

        // Evaluate passed pawns
        score += evaluate_passed_pawns(board, color) * color_multiplier;
    }

    score
}

fn is_isolated(board: &Board, color: Color, square: Square) -> bool {
    // Check if the pawn has no friendly pawns on adjacent files
    unimplemented!()
}

fn evaluate_doubled_pawns(board: &Board, color: Color) -> i32 {
    // Check for doubled pawns and penalize accordingly
    unimplemented!()
}

fn evaluate_passed_pawns(board: &Board, color: Color) -> i32 {
    // Check for passed pawns and give bonuses accordingly
    unimplemented!()
}
