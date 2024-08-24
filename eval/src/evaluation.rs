use types::{clear_bit, count_bits, get_lsb, Board, Color, Piece, Square};

#[derive(Copy, Clone, Default)]
pub struct Evaluation {
    pub material: i32,
    pub psqt: i32,
    pub king_safety: i32,
    pub pawn_structure: i32,
}

impl Evaluation {
    #[inline(always)]
    pub fn evaluate(board: &Board, color: Color) -> i32 {
        let mut score = 0;

        // Material evaluation
        score += Self::evaluate_material(board, color);

        // Piece mobility evaluation
        score += Self::evaluate_mobility(board, color);

        // Pawn structure evaluation
        score += Self::evaluate_pawn_structure(board, color);

        // King safety evaluation
        score += Self::evaluate_king_safety(board, color);

        // Flip score for the opponent's perspective
        if color == Color::Black {
            score = -score;
        }

        score
    }

    #[inline(always)]
    fn evaluate_material(board: &Board, color: Color) -> i32 {
        // Define material values
        const PAWN_VALUE: i32 = 100;
        const KNIGHT_VALUE: i32 = 320;
        const BISHOP_VALUE: i32 = 330;
        const ROOK_VALUE: i32 = 500;
        const QUEEN_VALUE: i32 = 900;

        let mut material_score = 0;

        // Count pieces and add to material score
        material_score += PAWN_VALUE * board.piece_count(Piece::Pawn, color) as i32;
        material_score += KNIGHT_VALUE * board.piece_count(Piece::Knight, color) as i32;
        material_score += BISHOP_VALUE * board.piece_count(Piece::Bishop, color) as i32;
        material_score += ROOK_VALUE * board.piece_count(Piece::Rook, color) as i32;
        material_score += QUEEN_VALUE * board.piece_count(Piece::Queen, color) as i32;

        material_score
    }

    #[inline(always)]
    fn evaluate_mobility(board: &Board, color: Color) -> i32 {
        let moves = board.generate_legal_moves(color);
        let mobility = moves.len() as i32;

        // A simple mobility heuristic: more mobility is generally better
        mobility * 10
    }

    #[inline(always)]
    fn evaluate_pawn_structure(board: &Board, color: Color) -> i32 {
        let mut score = 0;

        let pawns = board.pawns(color);

        // Evaluate isolated, doubled, and passed pawns
        let mut pawn_bitboard = pawns;
        while pawn_bitboard != 0 {
            let pawn_square_index = get_lsb(pawn_bitboard);
            let pawn_square = Square::from_index(pawn_square_index);

            // Check for isolated pawns
            if Self::is_isolated(board, color, pawn_square) {
                score -= 10;
            }

            // Check for doubled pawns
            if Self::is_doubled(board, color, pawn_square) {
                score -= 20;
            }

            // Check for passed pawns
            if Self::is_passed(board, color, pawn_square) {
                score += 50;
            }

            clear_bit(&mut pawn_bitboard, pawn_square_index);
        }

        score
    }

    #[inline(always)]
    fn evaluate_king_safety(board: &Board, color: Color) -> i32 {
        let king_square = board.king_square(color);
        let pawn_shield = board.pawn_shield(color, king_square);

        let king_safety = count_bits(pawn_shield) as i32 * 10;

        // Basic heuristic: more pawns near the king is generally better
        king_safety
    }

    #[inline(always)]
    fn is_isolated(board: &Board, color: Color, square: Square) -> bool {
        let file = square.to_index() % 8;
        let pawns = board.pawns(color);

        let left_file = if file > 0 { file - 1 } else { 8 };
        let right_file = if file < 7 { file + 1 } else { 8 };

        let left_file_mask = if left_file < 8 { 0x0101_0101_0101_0101u64 << left_file } else { 0 };
        let right_file_mask = if right_file < 8 { 0x0101_0101_0101_0101u64 << right_file } else { 0 };

        let adjacent_pawns = pawns & (left_file_mask | right_file_mask);
        adjacent_pawns == 0
    }

    #[inline(always)]
    fn is_doubled(board: &Board, color: Color, square: Square) -> bool {
        let file = square.to_index() % 8;
        let pawns = board.pawns(color);
        let file_mask = 0x0101_0101_0101_0101u64 << file;

        // A file with more than one pawn is considered doubled
        count_bits(pawns & file_mask) > 1
    }

    #[inline(always)]
    fn is_passed(board: &Board, color: Color, square: Square) -> bool {
        let rank = square.rank_usize();
        let file = square.to_index() % 8;
        let pawns = board.pawns(color.opponent());

        let left_file_mask = if file > 0 { 0x0101_0101_0101_0101u64 << (file - 1) } else { 0 };
        let right_file_mask = if file < 7 { 0x0101_0101_0101_0101u64 << (file + 1) } else { 0 };

        let forward_mask = if color == Color::White {
            0xFFu64 << ((rank + 1) * 8)
        } else {
            0xFFu64 >> ((7 - rank) * 8)
        };

        let opposing_pawns_mask = pawns & (left_file_mask | right_file_mask | (1u64 << square.to_index()));
        (opposing_pawns_mask & forward_mask) == 0
    }
}
