use crate::{clear_bit, count_bits, get_lsb, is_bit_set, set_bit, Bitboard, Color, Piece, Square};
use crate::zobrist::Zobrist;

pub struct Board {
    bishop_masks: [Bitboard; 64],
    bishop_magics: [u64; 64],
    bishop_shifts: [u8; 64],
    bishop_attack_tables: Vec<Vec<Bitboard>>,

    rook_masks: [Bitboard; 64],
    rook_magics: [u64; 64],
    rook_shifts: [u8; 64],
    rook_attack_tables: Vec<Vec<Bitboard>>,
    
    pieces: [Bitboard; 12], // 6 pieces for each color
    occupancy: [Bitboard; 2], // Occupancy for each color
    pub side_to_move: Color,
    castling_rights: u8, // 4 bits for castling rights
    en_passant: Option<Square>,
    halfmove_clock: u32,
    fullmove_number: u32,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            bishop_masks: [0; 64],
            bishop_magics: [0; 64],
            bishop_shifts: [0; 64],
            bishop_attack_tables: vec![vec![0; 512]; 64], // Size depends on magic bitboard setup

            rook_masks: [0; 64],
            rook_magics: [0; 64],
            rook_shifts: [0; 64],
            rook_attack_tables: vec![vec![0; 4096]; 64], // Size depends on magic bitboard setup

            pieces: [0; 12],
            occupancy: [0; 2],
            side_to_move: Color::White,
            castling_rights: 0b1111,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }

     pub fn material_count(&self) -> i32 {
        const PAWN_VALUE: i32 = 100;
        const KNIGHT_VALUE: i32 = 315;
        const BISHOP_VALUE: i32 = 325;
        const ROOK_VALUE: i32 = 500;
        const QUEEN_VALUE: i32 = 900;
         
         const KING_VALUE: i32 = 1000_000;

        let mut material_score = 0;

        // Sum the material for white pieces
        material_score += PAWN_VALUE * self.piece_count(Piece::Pawn, Color::White) as i32;
        material_score += KNIGHT_VALUE * self.piece_count(Piece::Knight, Color::White) as i32;
        material_score += BISHOP_VALUE * self.piece_count(Piece::Bishop, Color::White) as i32;
        material_score += ROOK_VALUE * self.piece_count(Piece::Rook, Color::White) as i32;
        material_score += QUEEN_VALUE * self.piece_count(Piece::Queen, Color::White) as i32;

        // Subtract the material for black pieces
        material_score -= PAWN_VALUE * self.piece_count(Piece::Pawn, Color::Black) as i32;
        material_score -= KNIGHT_VALUE * self.piece_count(Piece::Knight, Color::Black) as i32;
        material_score -= BISHOP_VALUE * self.piece_count(Piece::Bishop, Color::Black) as i32;
        material_score -= ROOK_VALUE * self.piece_count(Piece::Rook, Color::Black) as i32;
        material_score -= QUEEN_VALUE * self.piece_count(Piece::Queen, Color::Black) as i32;

        material_score
    }
    
    pub fn mobility(&self, color: Color) -> i32 {
        let mut mobility_score = 0;

        // Generate all legal moves for the given color
        let legal_moves = self.generate_legal_moves(color);

        // Mobility is simply the count of all legal moves
        mobility_score += legal_moves.len() as i32;

        mobility_score
    }
    
    pub fn is_complex(&self) -> bool {
        const COMPLEXITY_THRESHOLD: i32 = 20;

        // Calculate total piece count for both sides
        let total_pieces = self.total_piece_count();

        // Calculate the mobility for both sides
        let white_mobility = self.mobility(Color::White);
        let black_mobility = self.mobility(Color::Black);

        // A simple complexity heuristic: 
        // - If there are a lot of pieces on the board,
        // - If mobility is high,
        // Then the position is considered complex.
        let complexity_score = total_pieces + white_mobility + black_mobility;

        complexity_score > COMPLEXITY_THRESHOLD
    }

    fn total_piece_count(&self) -> i32 {
        // Count the total number of pieces for both sides
        let mut total_count = 0;

        // Iterate over all piece types and colors
        for piece_type in [Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen, Piece::King].iter() {
            total_count += self.piece_count(*piece_type, Color::White) as i32;
            total_count += self.piece_count(*piece_type, Color::Black) as i32;
        }

        total_count
    }
    
    #[inline(always)]
    pub fn make_move(&mut self, from: Square, to: Square, piece: Piece, color: Color) {
        let piece_index = self.get_piece_index(piece, color);
        clear_bit(&mut self.pieces[piece_index], from.to_index());
        set_bit(&mut self.pieces[piece_index], to.to_index());

        // Handle en passant, castling, and other special moves
        self.en_passant = None; // Reset en passant target square
        if piece == Piece::Pawn && (to.to_index() as i32 - from.to_index() as i32).abs() == 16 {
            self.en_passant = Some(Square::from_index((from.to_index() + to.to_index()) / 2));
        }

        self.update_occupancy();
        self.side_to_move = self.side_to_move.opponent();
        self.halfmove_clock += 1;
        if color == Color::Black {
            self.fullmove_number += 1;
        }
    }

    #[inline(always)]
    pub fn unmake_move(&mut self, from: Square, to: Square, piece: Piece, color: Color) {
        let piece_index = self.get_piece_index(piece, color);
        clear_bit(&mut self.pieces[piece_index], to.to_index());
        set_bit(&mut self.pieces[piece_index], from.to_index());

        self.update_occupancy();
        self.side_to_move = self.side_to_move.opponent();
        if color == Color::Black {
            self.fullmove_number -= 1;
        }
    }

    #[inline(always)]
    pub fn do_null_move(&mut self) {
        self.side_to_move = self.side_to_move.opponent();
    }

    #[inline(always)]
    pub fn undo_null_move(&mut self) {
        self.side_to_move = self.side_to_move.opponent();
    }

    #[inline(always)]
    pub fn get_piece_index(&self, piece: Piece, color: Color) -> usize {
        match color {
            Color::White => piece as usize,
            Color::Black => piece as usize + 6,
        }
    }

    #[inline(always)]
    pub fn update_occupancy(&mut self) {
        self.occupancy[0] = 0;
        self.occupancy[1] = 0;
        for i in 0..6 {
            self.occupancy[0] |= self.pieces[i];       // White pieces
            self.occupancy[1] |= self.pieces[i + 6];   // Black pieces
        }
    }

    pub fn generate_legal_moves(&self, color: Color) -> Vec<(Square, Square)> {
        let moves = Vec::new();
        for piece in 0..6 {
            let piece_bb = self.pieces[self.get_piece_index(Piece::from_u8(piece as u8).expect("Invalid piece index"), color)];
            let mut bb = piece_bb;
            while bb != 0 {
                let from_square = Square::from_index(get_lsb(bb));
                // Generate moves for this piece from this square
                // (Implement specific move generation logic here, ex: pawn moves, knight moves...)
                clear_bit(&mut bb, from_square.to_index());
            }
        }
        moves
    }

    pub fn get_piece_at_square(&self, square: Square) -> Option<(Piece, Color)> {
        let index = square.to_index();

        // Check all white pieces
        for piece in 0..6 {
            if is_bit_set(self.pieces[piece], index) {
                return Some((self.index_to_piece(piece), Color::White));
            }
        }

        // Check all black pieces
        for piece in 6..12 {
            if is_bit_set(self.pieces[piece], index) {
                return Some((self.index_to_piece(piece - 6), Color::Black));
            }
        }

        None // No piece found on this square
    }

    pub fn index_to_piece(&self, index: usize) -> Piece {
        match index {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => panic!("Invalid piece index"),
        }
    }
    
    /// Counts the number of a specific piece type on the board for a given color.
    #[inline(always)]
    pub fn piece_count(&self, piece: Piece, color: Color) -> usize {
        let piece_index = self.get_piece_index(piece, color);
        count_bits(self.pieces[piece_index]) as usize
    }
    
     /// Returns the square where the king of the given color is located.
    #[inline(always)]
    pub fn king_square(&self, color: Color) -> Square {
        let king_index = self.get_piece_index(Piece::King, color);
        let king_bb = self.pieces[king_index];
        debug_assert!(king_bb != 0, "King must exist on the board.");
        Square::from_index(get_lsb(king_bb))
    }

    
    /// Returns a bitboard representing the pawns that form a shield in front of the given king.
    #[inline(always)]
    pub fn pawn_shield(&self, color: Color, king_square: Square) -> Bitboard {
        let rank_shift: i8 = if color == Color::White { 8 } else { -8 };
        let king_index: i8 = king_square.to_index() as i8;
        let shield_mask: Bitboard = match king_index % 8 {
            0 => 0x3,        // King on A-file (000...0011)
            7 => 0x3 << 1,   // King on H-file (000...0110)
            _ => 0x7,        // King on other files (000...0111)
        } << (king_index + rank_shift);

        self.pieces[self.get_piece_index(Piece::Pawn, color)] & shield_mask
    }
    
    #[inline(always)]
    pub fn pawns(&self, color: Color) -> Bitboard {
        self.pieces[self.get_piece_index(Piece::Pawn, color)]
    }
    
    #[inline(always)]
    pub fn piece_squares(&self, piece: Piece, color: Color) -> Vec<Square> {
        let mut squares = Vec::new();
        let mut bitboard = self.pieces[self.get_piece_index(piece, color)];

        while bitboard != 0 {
            let square_index = get_lsb(bitboard);
            squares.push(Square::from_index(square_index));
            clear_bit(&mut bitboard, square_index);
        }

        squares
    }

     /// Determines if the game is in the endgame phase based on the remaining material.
    pub fn is_endgame(&self) -> bool {
        // Simple heuristic: if both sides have no queens or very few pieces, it's an endgame
        let white_major_pieces = self.piece_count(Piece::Queen, Color::White)
            + self.piece_count(Piece::Rook, Color::White);
        let black_major_pieces = self.piece_count(Piece::Queen, Color::Black)
            + self.piece_count(Piece::Rook, Color::Black);

        let total_minor_pieces = self.piece_count(Piece::Bishop, Color::White)
            + self.piece_count(Piece::Knight, Color::White)
            + self.piece_count(Piece::Bishop, Color::Black)
            + self.piece_count(Piece::Knight, Color::Black);

        // A very basic condition: if there are very few major pieces and no queens, it's an endgame
        white_major_pieces == 0 && black_major_pieces == 0 && total_minor_pieces <= 4
    }

    /// Generates all legal capture moves for the given color.
    pub fn generate_captures(&self, color: Color) -> Vec<(Square, Square)> {
        let mut captures = Vec::new();

        for piece in [Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen, Piece::King].iter() {
            let piece_bitboard = self.pieces[self.get_piece_index(*piece, color)];

            let mut piece_positions = piece_bitboard;
            while piece_positions != 0 {
                let from_square_index = get_lsb(piece_positions);
                let from_square = Square::from_index(from_square_index);

                // Generate all potential moves for this piece
                let possible_moves = self.generate_piece_moves(*piece, from_square, color);

                for to_square in possible_moves {
                    if self.is_capture(to_square, color) {
                        captures.push((from_square, to_square));
                    }
                }

                clear_bit(&mut piece_positions, from_square_index);
            }
        }

        captures
    }

    #[inline(always)]
    pub fn is_capture(&self, to_square: Square, color: Color) -> bool {
        let opponent_color = color.opponent();
        let opponent_pieces = self.occupancy[opponent_color as usize];
        is_bit_set(opponent_pieces, to_square.to_index())
    }

     /// Generates all pseudo-legal moves for a specific piece from a given square.
    pub fn generate_piece_moves(&self, piece: Piece, from_square: Square, color: Color) -> Vec<Square> {
        let mut moves = Vec::new();

        match piece {
            Piece::Pawn => self.generate_pawn_moves(from_square, color, &mut moves),
            Piece::Knight => self.generate_knight_moves(from_square, color, &mut moves),
            Piece::Bishop => self.generate_bishop_moves(from_square, color, &mut moves),
            Piece::Rook => self.generate_rook_moves(from_square, color, &mut moves),
            Piece::Queen => self.generate_queen_moves(from_square, color, &mut moves),
            Piece::King => self.generate_king_moves(from_square, color, &mut moves),
        }

        moves
    }

    #[inline(always)]
    pub fn generate_pawn_moves(&self, from_square: Square, color: Color, moves: &mut Vec<Square>) {
        // Implementation for pawn moves (both quiet moves and captures)
        let from_index = from_square.to_index();
        let direction = if color == Color::White { 8 } else { -8 };
        let target_square = Square::from_index((from_index as isize + direction) as usize);

        // Add the move if the target square is empty
        if !self.is_occupied(target_square) {
            moves.push(target_square);

            // Handle double move from the starting position
            if (color == Color::White && from_square.rank() == 1) ||
               (color == Color::Black && from_square.rank() == 6) {
                let double_target = Square::from_index((from_index as isize + 2 * direction) as usize);
                if !self.is_occupied(double_target) {
                    moves.push(double_target);
                }
            }
        }

        // Handle captures
        for &offset in &[-1, 1] {
            let capture_square = Square::from_index((from_index as isize + direction + offset) as usize);
            if self.is_occupied_by_opponent(capture_square, color) {
                moves.push(capture_square);
            }
        }
    }

    #[inline(always)]
    pub fn generate_knight_moves(&self, from_square: Square, color: Color, moves: &mut Vec<Square>) {
        // Implementation for knight moves
        let from_index = from_square.to_index();
        let knight_moves = [15, 17, 10, 6, -15, -17, -10, -6];

        for &offset in &knight_moves {
            let to_square = Square::from_index((from_index as isize + offset) as usize);
            if !self.is_occupied_by_friendly(to_square, color) {
                moves.push(to_square);
            }
        }
    }

    #[inline(always)]
    pub fn generate_bishop_moves(&self, from_square: Square, color: Color, moves: &mut Vec<Square>) {
        // Implementation for bishop moves (diagonals)
        let bishop_moves = self.get_bishop_attacks(from_square.to_index(), self.occupancy());
        self.add_sliding_piece_moves(bishop_moves, color, moves);
    }

    #[inline(always)]
    pub fn generate_rook_moves(&self, from_square: Square, color: Color, moves: &mut Vec<Square>) {
        // Implementation for rook moves (ranks and files)
        let rook_moves = self.get_rook_attacks(from_square.to_index(), self.occupancy());
        self.add_sliding_piece_moves(rook_moves, color, moves);
    }

    #[inline(always)]
    pub fn generate_queen_moves(&self, from_square: Square, color: Color, moves: &mut Vec<Square>) {
        // Implementation for queen moves (combines rook and bishop moves)
        let queen_moves = self.get_bishop_attacks(from_square.to_index(), self.occupancy())
                         | self.get_rook_attacks(from_square.to_index(), self.occupancy());
        self.add_sliding_piece_moves(queen_moves, color, moves);
    }

    #[inline(always)]
    pub fn occupancy(&self) -> Bitboard {
        self.occupancy[0] | self.occupancy[1]
    }

    #[inline(always)]
    pub fn generate_king_moves(&self, from_square: Square, color: Color, moves: &mut Vec<Square>) {
        // Implementation for king moves (one square in any direction)
        let from_index = from_square.to_index();
        let king_moves = [1, -1, 8, -8, 9, -9, 7, -7];

        for &offset in &king_moves {
            let to_square = Square::from_index((from_index as isize + offset) as usize);
            if !self.is_occupied_by_friendly(to_square, color) {
                moves.push(to_square);
            }
        }
    }

    #[inline(always)]
    pub fn add_sliding_piece_moves(&self, attacks: Bitboard, color: Color, moves: &mut Vec<Square>) {
        let mut attack_bitboard = attacks;
        while attack_bitboard != 0 {
            let to_square_index = get_lsb(attack_bitboard);
            let to_square = Square::from_index(to_square_index);
            if !self.is_occupied_by_friendly(to_square, color) {
                moves.push(to_square);
            }
            clear_bit(&mut attack_bitboard, to_square_index);
        }
    }

    #[inline(always)]
    pub fn is_occupied(&self, square: Square) -> bool {
        let all_pieces = self.occupancy[0] | self.occupancy[1];
        is_bit_set(all_pieces, square.to_index())
    }

    #[inline(always)]
    pub fn is_occupied_by_friendly(&self, square: Square, color: Color) -> bool {
        is_bit_set(self.occupancy[color as usize], square.to_index())
    }

    #[inline(always)]
    pub fn is_occupied_by_opponent(&self, square: Square, color: Color) -> bool {
        is_bit_set(self.occupancy[color.opponent() as usize], square.to_index())
    }

    pub fn get_bishop_attacks(&self, square_index: usize, occupancy: Bitboard) -> Bitboard {
        // The relevant occupancy bits for the bishop
        let relevant_occupancy = occupancy & self.bishop_masks[square_index];
        let magic_index = (relevant_occupancy.wrapping_mul(self.bishop_magics[square_index]))
            >> self.bishop_shifts[square_index];
        self.bishop_attack_tables[square_index][magic_index as usize]
    }

    pub fn get_rook_attacks(&self, square_index: usize, occupancy: Bitboard) -> Bitboard {
        // The relevant occupancy bits for the rook
        let relevant_occupancy = occupancy & self.rook_masks[square_index];
        let magic_index = (relevant_occupancy.wrapping_mul(self.rook_magics[square_index]))
            >> self.rook_shifts[square_index];
        self.rook_attack_tables[square_index][magic_index as usize]
    }

    pub fn compute_zobrist_hash(&self, zobrist: &Zobrist) -> u64 {
        let mut hash: u64 = 0;

        // XOR the piece positions
        for color in 0..2 {
            for piece in 0..6 {
                let piece_bitboard = self.pieces[self.get_piece_index(Piece::from_u8(piece as u8).expect("Could not get the piece"), Color::from_u8(color as u8).expect("Could not get the color"))];
                let mut bitboard = piece_bitboard;
                while bitboard != 0 {
                    let square_index = get_lsb(bitboard);
                    hash ^= zobrist.piece_keys[color][piece][square_index];
                    clear_bit(&mut bitboard, square_index);
                }
            }
        }

        // XOR the castling rights
        let castling_index = self.castling_rights as usize;
        hash ^= zobrist.castling_keys[castling_index];

        // XOR the en passant square, if any
        if let Some(en_passant_square) = self.en_passant {
            let file = en_passant_square.to_index() % 8;
            hash ^= zobrist.en_passant_keys[file];
        }

        // XOR the side to move
        if self.side_to_move == Color::Black {
            hash ^= zobrist.side_to_move_key;
        }

        hash
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Square, Piece, Color};

    #[test]
    fn test_make_and_unmake_move() {
        let mut board = Board::new();
        let from = Square::E2;
        let to = Square::E4;
        let piece = Piece::Pawn;
        let color = Color::White;

        board.make_move(from, to, piece, color);

        assert_eq!(board.get_piece_at_square(to), Some((piece, color)));
        assert_eq!(board.get_piece_at_square(from), None);

        board.unmake_move(from, to, piece, color);

        assert_eq!(board.get_piece_at_square(from), Some((piece, color)));
        assert_eq!(board.get_piece_at_square(to), None);
    }

    #[test]
    fn test_null_move() {
        let mut board = Board::new();
        let initial_side = board.side_to_move;

        board.do_null_move();
        assert_ne!(board.side_to_move, initial_side);

        board.undo_null_move();
        assert_eq!(board.side_to_move, initial_side);
    }

    #[test]
    fn test_get_piece_at_square() {
        let mut board = Board::new();
        let square = Square::E2;
        let piece = Piece::Pawn;
        let color = Color::White;

        board.make_move(square, Square::E4, piece, color);

        assert_eq!(board.get_piece_at_square(Square::E4), Some((piece, color)));
        assert_eq!(board.get_piece_at_square(Square::E2), None);
    }

    #[test]
    fn test_update_occupancy() {
        let mut board = Board::new();
        let from = Square::E2;
        let to = Square::E4;
        let piece = Piece::Pawn;
        let color = Color::White;

        board.make_move(from, to, piece, color);
        board.update_occupancy();

        assert!(is_bit_set(board.occupancy[0], to.to_index()));
        assert!(!is_bit_set(board.occupancy[0], from.to_index()));
    }

    #[test]
    fn test_generate_legal_moves() {
        let board = Board::new();
        let moves = board.generate_legal_moves(Color::White);
        
        // Here you would add assertions based on the expected legal moves in the starting position.
        // This is simplified since the actual move generation logic isn't fully implemented.
        assert!(moves.is_empty()); // Assuming no logic is added yet.
    }

    #[test]
    fn test_piece_indexing() {
        let board = Board::new();

        assert_eq!(board.get_piece_index(Piece::Pawn, Color::White), 0);
        assert_eq!(board.get_piece_index(Piece::Pawn, Color::Black), 6);
        assert_eq!(board.get_piece_index(Piece::Queen, Color::White), 4);
        assert_eq!(board.get_piece_index(Piece::King, Color::Black), 11);
    }
    
    #[test]
    fn test_piece_count_initial_position() {
        let board = Board::new();

        assert_eq!(board.piece_count(Piece::Pawn, Color::White), 8);
        assert_eq!(board.piece_count(Piece::Pawn, Color::Black), 8);

        assert_eq!(board.piece_count(Piece::Rook, Color::White), 2);
        assert_eq!(board.piece_count(Piece::Rook, Color::Black), 2);

        assert_eq!(board.piece_count(Piece::Knight, Color::White), 2);
        assert_eq!(board.piece_count(Piece::Knight, Color::Black), 2);

        assert_eq!(board.piece_count(Piece::Bishop, Color::White), 2);
        assert_eq!(board.piece_count(Piece::Bishop, Color::Black), 2);

        assert_eq!(board.piece_count(Piece::Queen, Color::White), 1);
        assert_eq!(board.piece_count(Piece::Queen, Color::Black), 1);

        assert_eq!(board.piece_count(Piece::King, Color::White), 1);
        assert_eq!(board.piece_count(Piece::King, Color::Black), 1);
    }

    #[test]
    fn test_piece_count_after_moves() {
        let mut board = Board::new();

        // Simulate some moves
        board.make_move(Square::E2, Square::E4, Piece::Pawn, Color::White);
        board.make_move(Square::E7, Square::E5, Piece::Pawn, Color::Black);
        board.make_move(Square::D2, Square::D4, Piece::Pawn, Color::White);

        assert_eq!(board.piece_count(Piece::Pawn, Color::White), 7);
        assert_eq!(board.piece_count(Piece::Pawn, Color::Black), 7);

        // Test after capturing a pawn
        board.make_move(Square::E4, Square::E5, Piece::Pawn, Color::White);

        assert_eq!(board.piece_count(Piece::Pawn, Color::White), 7);
        assert_eq!(board.piece_count(Piece::Pawn, Color::Black), 6);
    }

    #[test]
    fn test_piece_count_empty_board() {
        let mut board = Board::new();

        // Manually clear the board (for testing purposes only)
        for i in 0..12 {
            board.pieces[i] = 0;
        }
        board.update_occupancy();

        for piece in [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ] {
            assert_eq!(board.piece_count(piece, Color::White), 0);
            assert_eq!(board.piece_count(piece, Color::Black), 0);
        }
    }
    
    #[test]
    fn test_pawn_shield() {
        let board = Board::new();
    
        // Test the initial pawn shield in front of the White king
        let king_square = board.king_square(Color::White);
        let shield = board.pawn_shield(Color::White, king_square);
        let expected_shield = (1 << Square::F2.to_index()) | (1 << Square::E2.to_index()) | (1 << Square::D2.to_index());
        assert_eq!(shield, expected_shield);
    
        // Test the initial pawn shield in front of the Black king
        let king_square = board.king_square(Color::Black);
        let shield = board.pawn_shield(Color::Black, king_square);
        let expected_shield = (1 << Square::F7.to_index()) | (1 << Square::E7.to_index()) | (1 << Square::D7.to_index());
        assert_eq!(shield, expected_shield);
        
        // Move the White king to G1 and a pawn to G2, then check the shield
        let mut board = Board::new();
        board.make_move(Square::E1, Square::G1, Piece::King, Color::White);
        board.make_move(Square::H2, Square::G2, Piece::Pawn, Color::White);
        let shield = board.pawn_shield(Color::White, Square::G1);
        let expected_shield = (1 << Square::G2.to_index()) | (1 << Square::F2.to_index());
        assert_eq!(shield, expected_shield);
    }

    #[test]
    fn test_king_square() {
        let board = Board::new();
    
        assert_eq!(board.king_square(Color::White), Square::E1);
        assert_eq!(board.king_square(Color::Black), Square::E8);
        
        // Move the white king to D2
        let mut board = Board::new();
        board.make_move(Square::E1, Square::D2, Piece::King, Color::White);
        assert_eq!(board.king_square(Color::White), Square::D2);
    }

    #[test]
    fn test_pawns_initial_position() {
        let board = Board::new();
        
        // White pawns in the initial position (row 2)
        let expected_white_pawns = 0x0000_0000_0000_FF00;
        assert_eq!(board.pawns(Color::White), expected_white_pawns);
        
        // Black pawns in the initial position (row 7)
        let expected_black_pawns = 0x00FF_0000_0000_0000;
        assert_eq!(board.pawns(Color::Black), expected_black_pawns);
    }

    #[test]
    fn test_pawns_after_move() {
        let mut board = Board::new();
        
        // Move white pawn from E2 to E4
        board.make_move(Square::E2, Square::E4, Piece::Pawn, Color::White);
        let expected_white_pawns = 0x0000_0000_0000_DF00 | 0x0000_0000_0010_0000;
        assert_eq!(board.pawns(Color::White), expected_white_pawns);
        
        // Move black pawn from E7 to E5
        board.make_move(Square::E7, Square::E5, Piece::Pawn, Color::Black);
        let expected_black_pawns = 0x00EF_0000_0000_0000 | 0x0000_1000_0000_0000;
        assert_eq!(board.pawns(Color::Black), expected_black_pawns);
    }

    #[test]
    fn test_pawns_empty_board() {
        let mut board = Board::new();

        // Manually clear all pawns from the board
        board.pieces[board.get_piece_index(Piece::Pawn, Color::White)] = 0;
        board.pieces[board.get_piece_index(Piece::Pawn, Color::Black)] = 0;

        assert_eq!(board.pawns(Color::White), 0);
        assert_eq!(board.pawns(Color::Black), 0);
    }
    
    #[test]
    fn test_piece_squares_initial_position() {
        let board = Board::new();

        // White rooks in the initial position (A1, H1)
        let white_rooks = board.piece_squares(Piece::Rook, Color::White);
        let expected_white_rooks = vec![Square::A1, Square::H1];
        assert_eq!(white_rooks, expected_white_rooks);

        // Black knights in the initial position (B8, G8)
        let black_knights = board.piece_squares(Piece::Knight, Color::Black);
        let expected_black_knights = vec![Square::B8, Square::G8];
        assert_eq!(black_knights, expected_black_knights);
    }

    #[test]
    fn test_piece_squares_after_moves() {
        let mut board = Board::new();

        // Move white rook from A1 to A4
        board.make_move(Square::A1, Square::A4, Piece::Rook, Color::White);
        let white_rooks = board.piece_squares(Piece::Rook, Color::White);
        let expected_white_rooks = vec![Square::A4, Square::H1];
        assert_eq!(white_rooks, expected_white_rooks);

        // Move black knight from B8 to C6
        board.make_move(Square::B8, Square::C6, Piece::Knight, Color::Black);
        let black_knights = board.piece_squares(Piece::Knight, Color::Black);
        let expected_black_knights = vec![Square::C6, Square::G8];
        assert_eq!(black_knights, expected_black_knights);
    }

    #[test]
    fn test_piece_squares_empty_board() {
        let mut board = Board::new();

        // Manually clear all pieces from the board
        for i in 0..12 {
            board.pieces[i] = 0;
        }
        board.update_occupancy();

        let white_pawns = board.piece_squares(Piece::Pawn, Color::White);
        let black_pawns = board.piece_squares(Piece::Pawn, Color::Black);

        assert!(white_pawns.is_empty());
        assert!(black_pawns.is_empty());
    }

     #[test]
    fn test_generate_pawn_moves() {
        let board = Board::new();

        // Test white pawns from initial position
        let moves = board.generate_piece_moves(Piece::Pawn, Square::E2, Color::White);
        let expected_moves = vec![Square::E3, Square::E4];
        assert_eq!(moves, expected_moves);

        // Test black pawns from initial position
        let moves = board.generate_piece_moves(Piece::Pawn, Square::E7, Color::Black);
        let expected_moves = vec![Square::E6, Square::E5];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_generate_knight_moves() {
        let board = Board::new();

        // Test white knight from initial position
        let moves = board.generate_piece_moves(Piece::Knight, Square::G1, Color::White);
        let expected_moves = vec![Square::F3, Square::H3];
        assert_eq!(moves, expected_moves);

        // Test knight in the center of the board
        let moves = board.generate_piece_moves(Piece::Knight, Square::E4, Color::White);
        let expected_moves = vec![
            Square::D6, Square::F6, Square::C5, Square::G5,
            Square::C3, Square::G3, Square::D2, Square::F2,
        ];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_generate_bishop_moves() {
        let board = Board::new();
        // Assuming the bishop is on D4 and no pieces are blocking it.
        let moves = board.generate_piece_moves(Piece::Bishop, Square::D4, Color::White);
        // Calculate expected moves based on bishop's potential sliding moves.
        // Add expected moves here.
        let expected_moves = vec![/* list expected squares */];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_generate_rook_moves() {
        let board = Board::new();
        // Assuming the rook is on A1 and no pieces are blocking it.
        let moves = board.generate_piece_moves(Piece::Rook, Square::A1, Color::White);
        // Calculate expected moves based on rook's potential sliding moves.
        // Add expected moves here.
        let expected_moves = vec![/* list expected squares */];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_generate_queen_moves() {
        let board = Board::new();
        // Assuming the queen is on D1 and no pieces are blocking it.
        let moves = board.generate_piece_moves(Piece::Queen, Square::D1, Color::White);
        // Calculate expected moves based on queen's potential sliding moves.
        // Add expected moves here.
        let expected_moves = vec![/* list expected squares */];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_generate_king_moves() {
        let board = Board::new();

        // Test king in the center of the board
        let moves = board.generate_piece_moves(Piece::King, Square::E4, Color::White);
        let expected_moves = vec![
            Square::E5, Square::D5, Square::D4, Square::D3,
            Square::E3, Square::F3, Square::F4, Square::F5,
        ];
        assert_eq!(moves, expected_moves);
    }
}
