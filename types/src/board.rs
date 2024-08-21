use crate::{clear_bit, count_bits, get_lsb, is_bit_set, set_bit, Bitboard, Color, Piece, Square};

pub struct Board {
    pieces: [Bitboard; 12], // 6 pieces for each color
    occupancy: [Bitboard; 2], // Occupancy for each color
    side_to_move: Color,
    castling_rights: u8, // 4 bits for castling rights
    en_passant: Option<Square>,
    halfmove_clock: u32,
    fullmove_number: u32,
}

impl Default for Board {
    fn default() -> Self {
        Self {
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
    fn get_piece_index(&self, piece: Piece, color: Color) -> usize {
        match color {
            Color::White => piece as usize,
            Color::Black => piece as usize + 6,
        }
    }

    #[inline(always)]
    fn update_occupancy(&mut self) {
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

    fn index_to_piece(&self, index: usize) -> Piece {
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
}
