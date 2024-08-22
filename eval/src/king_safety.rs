use types::{Bitboard, Board, Color};

const KING_SAFETY_TABLE: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -20, -30, -30, -40, -40, -30, -30, -20,
    -10, -20, -20, -20, -20, -20, -20, -10,
     20,  20,   0,   0,   0,   0,  20,  20,
     20,  30,  10,   0,   0,  10,  30,  20,
];

#[inline(always)]
pub fn evaluate_king_safety(board: &Board) -> i32 {
    let mut score = 0;

    for color in [Color::White, Color::Black].iter() {
        let king_square = board.king_square(*color);
        let index = king_square.to_index();

        // Add bonuses or penalties based on the king's safety
        score += KING_SAFETY_TABLE[index] * if *color == Color::White { 1 } else { -1 };

        // Check pawn shield
        let pawn_shield = board.pawn_shield(*color, king_square);
        score += evaluate_pawn_shield(pawn_shield) * if *color == Color::White { 1 } else { -1 };
    }

    score
}

fn evaluate_pawn_shield(pawn_shield: Bitboard) -> i32 {
    let shield_strength = pawn_shield.count_ones() as i32;
    shield_strength * 10
}
