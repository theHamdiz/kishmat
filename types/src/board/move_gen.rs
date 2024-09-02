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
    fn generate_move_from_square(&self, color: Color, moves: &mut Vec<(Square, Square)>, piece: i32, from_square: Square) {
        // Generate moves for this piece from this square
        match Piece::from_u8(piece as u8).expect("Invalid piece index") {
            Piece::Pawn => self.generate_pawn_moves(from_square, color, moves),
            Piece::Knight => self.generate_knight_moves(from_square, color, moves),
            Piece::Bishop => self.generate_bishop_moves(from_square, color, moves),
            Piece::Rook => self.generate_rook_moves(from_square, color, moves),
            Piece::Queen => self.generate_queen_moves(from_square, color, moves),
            Piece::King => self.generate_king_moves(from_square, color, moves),
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
        let target_index = (from_index as isize + direction) as usize;
        if target_index < 64 {
            let target_square = Square::from_index(target_index);
            if !self.is_occupied(target_square) {
                moves.push((from_square, target_square));
    
                // Two-square forward move from starting position
                // println!("Color is: {:?}", color);
                // println!("Generating moves for {:?} during {:?}", color, self.side_to_move);

                let starting_rank = if color == Color::White { 1 } else { 6 };
                if from_square.rank_usize() == starting_rank {
                    let double_target_index = (from_index as isize + 2 * direction) as usize;
                    if double_target_index < 64 {
                        let double_target_square = Square::from_index(double_target_index);
                        if !self.is_occupied(double_target_square) {
                            moves.push((from_square, double_target_square));
                        }
                    }
                }
            }
        }
    
        // Captures
        for &offset in &[-1, 1] {
            let capture_index = (from_index as isize + direction + offset) as usize;
            if capture_index < 64 {
                let capture_square = Square::from_index(capture_index);
                if self.is_occupied_by_opponent(capture_square, color) {
                    moves.push((from_square, capture_square));
                }
            }
        }
    }


    #[inline(always)]
    pub fn generate_knight_moves(&self, from_square: Square, color: Color, moves: &mut Vec<(Square, Square)>) {
        let from_index = from_square.to_index();
        let knight_moves = [15, 17, 10, 6, -15, -17, -10, -6];
    
        for &offset in &knight_moves {
            if let Some(to_index) = from_index.checked_add(offset as usize) {
                if to_index < 64 {
                    let to_square = Square::from_index(to_index);
                    if !self.is_occupied_by_friendly(to_square, color) {
                        moves.push((from_square, to_square));
                    }
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
            if (0..64).contains(&to_index) { // Same as: to_index >= 0 && to_index < 64
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
    
    #[inline(always)]
    pub fn apply_move(&mut self, mv: (Square, Square), color: Color) {
        let (from_square, to_square) = mv;

        // Get the piece at the starting square
        if let Some((piece, _)) = self.get_piece_at_square(from_square) {
            // Handle special cases: Castling, En Passant, Promotion
           if piece == Piece::King {
                if from_square == Square::E1 && color == Color::White {
                    if to_square == Square::G1 {
                        self.castle_kingside();
                        return;
                    } else if to_square == Square::C1 {
                        self.castle_queenside();
                        return;
                    }
                } else if from_square == Square::E8 && color == Color::Black {
                    if to_square == Square::G8 {
                        self.castle_kingside();
                        return;
                    } else if to_square == Square::C8 {
                        self.castle_queenside();
                        return;
                    }
                }
           }

           if piece == Piece::Pawn {
                // Handle en passant
                if let Some(ep_square) = self.en_passant {
                    if to_square == ep_square {
                        let capture_index = (from_square.to_index() as isize + if color == Color::White { -8 } else { 8 }) as usize;
                        if capture_index < 64 {
                            let capture_square = Square::from_index(capture_index);
                            self.capture_piece(capture_square);
                        }
                    }
                }
            
                // Handle promotion
                if (color == Color::White && to_square.rank_usize() == 7) || (color == Color::Black && to_square.rank_usize() == 0) {
                    self.promote_pawn(to_square, Piece::Queen, color);
                    self.side_to_move = color.opponent();
                    return;
                }
            }

            // Regular move: Update piece positions and handle capture
            if self.is_occupied(to_square) {
                self.capture_piece(to_square);
            }
            self.make_move(from_square, to_square, piece, color);

            // Update en passant square if applicable
            if piece == Piece::Pawn && (to_square.to_index() as isize - from_square.to_index() as isize).abs() == 16 {
                self.en_passant = Some(Square::from_index((from_square.to_index() + to_square.to_index()) / 2));
            } else {
                self.en_passant = None;
            }

            // Update castling rights if necessary
            self.update_castling_rights(from_square, to_square, piece, color);

            // Switch sides
            self.side_to_move = color.opponent();
        }
    }

    #[inline(always)]
    fn update_castling_rights(&mut self, from_square: Square, to_square: Square, piece: Piece, color: Color) {
        // Update castling rights if a rook or king moves
        if piece == Piece::King {
            if color == Color::White {
                self.castling_rights &= !0b11; // White king moves, lose both castling rights
            } else {
                self.castling_rights &= !0b1100; // Black king moves, lose both castling rights
            }
        } else if piece == Piece::Rook {
            if color == Color::White {
                if from_square == Square::A1 {
                    self.castling_rights &= !0b01; // Lose White queenside castling right
                } else if from_square == Square::H1 {
                    self.castling_rights &= !0b10; // Lose White kingside castling right
                }
            } else if from_square == Square::A8 {
                self.castling_rights &= !0b0100; // Lose Black queenside castling right
            } else if from_square == Square::H8 {
                self.castling_rights &= !0b1000; // Lose Black kingside castling right
            }
        }

        // Handle rook capture that impacts castling rights
        if self.is_occupied(to_square) && self.get_piece_at_square(to_square).unwrap().0 == Piece::Rook {
            if to_square == Square::A1 {
                self.castling_rights &= !0b01; // Capturing White queenside rook
            } else if to_square == Square::H1 {
                self.castling_rights &= !0b10; // Capturing White kingside rook
            } else if to_square == Square::A8 {
                self.castling_rights &= !0b0100; // Capturing Black queenside rook
            } else if to_square == Square::H8 {
                self.castling_rights &= !0b1000; // Capturing Black kingside rook
            }
        }
    }
}