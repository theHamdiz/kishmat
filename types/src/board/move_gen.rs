use crate::board::Board;
use crate::{clear_bit, get_lsb, set_bit, Bitboard, Color, Piece, Square};

impl Board{
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
    pub fn generate_legal_moves(&self, color: Color) -> Vec<(Square, Square)> {
        let mut moves = Vec::new();
        for piece in 0..6 {
            let piece_bb = self.pieces[self.get_piece_index(Piece::from_u8(piece as u8).expect("Invalid piece index"), color)];
            let mut bb = piece_bb;
            while bb != 0 {
                let from_square = Square::from_index(get_lsb(bb));

                self.generate_move_from_square(color, &mut moves, piece, from_square);

                // Clear the bit corresponding to the current piece's square
                clear_bit(&mut bb, from_square.to_index());
            }
        }
        moves
    }

    #[inline(always)]
    fn generate_move_from_square(&self, color: Color, mut moves: &mut Vec<(Square, Square)>, piece: i32, from_square: Square) {
        // Generate moves for this piece from this square
        match Piece::from_u8(piece as u8).expect("Invalid piece index") {
            Piece::Pawn => self.generate_pawn_moves(from_square, color, &mut moves),
            Piece::Knight => self.generate_knight_moves(from_square, color, &mut moves),
            Piece::Bishop => self.generate_bishop_moves(from_square, color, &mut moves),
            Piece::Rook => self.generate_rook_moves(from_square, color, &mut moves),
            Piece::Queen => self.generate_queen_moves(from_square, color, &mut moves),
            Piece::King => self.generate_king_moves(from_square, color, &mut moves),
        }
    }

    /// Generates all pseudo-legal moves for a specific piece from a given square.
    #[inline(always)]
    pub fn generate_piece_moves(&self, piece: Piece, from_square: Square, color: Color) -> Vec<(Square, Square)> {
        let mut moves = Vec::new();

        self.generate_move_from_square(color, &mut moves, piece as i32, from_square);

        moves
    }

    #[inline(always)]
    pub fn generate_pawn_moves(&self, from_square: Square, color: Color, moves: &mut Vec<(Square, Square)>) {
        let from_index = from_square.to_index();
        let direction = if color == Color::White { 8 } else { -8 };

        // One-square forward move
        let target_square = Square::from_index((from_index as isize + direction) as usize);
        if !self.is_occupied(target_square) {
            moves.push((from_square, target_square));

            // Two-square forward move from starting position
            if (color == Color::White && from_square.rank_usize() == 1) ||
               (color == Color::Black && from_square.rank_usize() == 6) {
                let double_target = Square::from_index((from_index as isize + 2 * direction) as usize);
                if !self.is_occupied(double_target) {
                    moves.push((from_square, double_target));
                }
            }
        }

        // Captures
        for &offset in &[-1, 1] {
            let capture_square = Square::from_index((from_index as isize + direction + offset) as usize);
            if self.is_occupied_by_opponent(capture_square, color) {
                moves.push((from_square, capture_square));
            }
        }
    }

     #[inline(always)]
    pub fn generate_knight_moves(&self, from_square: Square, color: Color, moves: &mut Vec<(Square, Square)>) {
        let from_index = from_square.to_index();
        let knight_moves = [15, 17, 10, 6, -15, -17, -10, -6];

        for &offset in &knight_moves {
            let to_index = from_index as isize + offset;
            if to_index >= 0 && to_index < 64 { // Ensure the index is within bounds
                let to_square = Square::from_index(to_index as usize);
                if !self.is_occupied_by_friendly(to_square, color) {
                    moves.push((from_square, to_square));
                }
            }
        }
    }

    #[inline(always)]
    pub fn generate_bishop_moves(&self, from_square: Square, color: Color, moves: &mut Vec<(Square, Square)>) {
        let bishop_moves = self.get_bishop_attacks(from_square.to_index(), self.occupancy());
        self.add_sliding_piece_moves(from_square, bishop_moves, color, moves);
    }

    #[inline(always)]
    pub fn generate_rook_moves(&self, from_square: Square, color: Color, moves: &mut Vec<(Square, Square)>) {
        let rook_moves = self.get_rook_attacks(from_square.to_index(), self.occupancy());
        self.add_sliding_piece_moves(from_square, rook_moves, color, moves);
    }

    #[inline(always)]
    pub fn generate_queen_moves(&self, from_square: Square, color: Color, moves: &mut Vec<(Square, Square)>) {
        let queen_moves = self.get_bishop_attacks(from_square.to_index(), self.occupancy())
                         | self.get_rook_attacks(from_square.to_index(), self.occupancy());
        self.add_sliding_piece_moves(from_square, queen_moves, color, moves);
    }

    #[inline(always)]
    pub fn generate_king_moves(&self, from_square: Square, color: Color, moves: &mut Vec<(Square, Square)>) {
        let from_index = from_square.to_index();
        let king_moves = [1, -1, 8, -8, 9, -9, 7, -7];

        for &offset in &king_moves {
            let to_index = from_index as isize + offset;

            // Ensure the index is within bounds and that it doesn't wrap around rows
            if to_index >= 0 && to_index < 64 {
                let to_square = Square::from_index(to_index as usize);

                // Check for row wrapping: Ensure that moves don't wrap horizontally across the board
                let from_file = from_square.file();
                let to_file = to_square.file();
                if (offset.abs() == 1 || offset.abs() == 9 || offset.abs() == 7) && from_file != to_file {
                    continue; // Skip move if it wraps around the edge of the board
                }

                if !self.is_occupied_by_friendly(to_square, color) {
                    moves.push((from_square, to_square));
                }
            }
        }
    }

    #[inline(always)]
    pub fn add_sliding_piece_moves(&self, from_square: Square, attacks: Bitboard, color: Color, moves: &mut Vec<(Square, Square)>) {
        let mut attack_bitboard = attacks;
        while attack_bitboard != 0 {
            let to_square_index = get_lsb(attack_bitboard);
            let to_square = Square::from_index(to_square_index);
            if !self.is_occupied_by_friendly(to_square, color) {
                moves.push((from_square, to_square));
            }
            clear_bit(&mut attack_bitboard, to_square_index);
        }
    }
}