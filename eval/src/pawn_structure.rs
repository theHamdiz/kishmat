use types::{clear_bit, get_lsb, Board, Color, Square};

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
            let pawn_square_index = get_lsb(pawn_bitboard);
            let pawn_square = Square::from_index(pawn_square_index);
            if is_isolated(board, color, pawn_square) {
                score += ISOLATED_PAWN_PENALTY * color_multiplier;
            }
            clear_bit(&mut pawn_bitboard, pawn_square_index);
        }

        // Evaluate doubled pawns
        score += evaluate_doubled_pawns(board, color) * color_multiplier;

        // Evaluate passed pawns
        score += evaluate_passed_pawns(board, color) * color_multiplier;
    }

    score
}


#[inline(always)]
fn is_isolated(board: &Board, color: Color, square: Square) -> bool {
    let file = square.to_index() % 8;
    let pawns = board.pawns(color);

    let left_file = if file > 0 { file - 1 } else { 8 }; // File to the left
    let right_file = if file < 7 { file + 1 } else { 8 }; // File to the right

    let left_file_mask = if left_file < 8 { 0x0101_0101_0101_0101u64 << left_file } else { 0 };
    let right_file_mask = if right_file < 8 { 0x0101_0101_0101_0101u64 << right_file } else { 0 };

    let adjacent_pawns = pawns & (left_file_mask | right_file_mask);
    adjacent_pawns == 0
}


#[inline(always)]
fn evaluate_doubled_pawns(board: &Board, color: Color) -> i32 {
    let mut penalty = 0;
    let mut files = 0u64;

    let mut pawns = board.pawns(color);
    while pawns != 0 {
        let square_index = get_lsb(pawns);
        let file = square_index % 8;
        let file_mask = 0x0101_0101_0101_0101u64 << file;

        // Check if there are multiple pawns on the same file
        if files & file_mask != 0 {
            penalty += DOUBLED_PAWN_PENALTY;
        }
        files |= file_mask;

        clear_bit(&mut pawns, square_index);
    }

    penalty
}


#[inline(always)]
fn evaluate_passed_pawns(board: &Board, color: Color) -> i32 {
    let mut bonus = 0;

    let pawns = board.pawns(color);
    let opponent_pawns = board.pawns(color.opponent());

    let direction_mask = if color == Color::White {
        0xFF_FF_FF_FF_FF_FF_FF_FFu64
    } else {
        0xFF_FF_FF_FF_FF_FF_FF_FFu64 << 56
    };

    let mut pawn_bitboard = pawns;
    while pawn_bitboard != 0 {
        let square_index = get_lsb(pawn_bitboard);
        let file = square_index % 8;
        let rank = square_index / 8;

        let left_file_mask = if file > 0 { 0x0101_0101_0101_0101u64 << (file - 1) } else { 0 };
        let right_file_mask = if file < 7 { 0x0101_0101_0101_0101u64 << (file + 1) } else { 0 };
        let forward_mask = direction_mask >> (rank * 8);

        let opposing_pawns_mask = opponent_pawns & (left_file_mask | right_file_mask | (1u64 << square_index));
        if opposing_pawns_mask & forward_mask == 0 {
            bonus += PASSED_PAWN_BONUS;
        }

        clear_bit(&mut pawn_bitboard, square_index);
    }

    bonus
}

