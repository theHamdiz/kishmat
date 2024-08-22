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
}