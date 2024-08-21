use crate::{Bitboard, Piece, Color, Square, set_bit, clear_bit, count_bits, is_bit_set};

pub struct Board {
    pieces: [Bitboard; 12], // 6 pieces for each color
    occupancy: [Bitboard; 2], // Occupancy for each color
    side_to_move: Color,
    castling_rights: u8, // 4 bits for castling rights
    en_passant: Option<Square>,
    halfmove_clock: u32,
    fullmove_number: u32,
}

impl Board {
    pub fn new() -> Self {
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

    pub fn make_move(&mut self, from: Square, to: Square, piece: Piece, color: Color) {
        let piece_index = self.get_piece_index(piece, color);
        clear_bit(&mut self.pieces[piece_index], from.to_index());
        set_bit(&mut self.pieces[piece_index], to.to_index());
        self.update_occupancy();
    }

    pub fn unmake_move(&mut self, from: Square, to: Square, piece: Piece, color: Color) {
        let piece_index = self.get_piece_index(piece, color);
        clear_bit(&mut self.pieces[piece_index], to.to_index());
        set_bit(&mut self.pieces[piece_index], from.to_index());
        self.update_occupancy();
    }

    fn get_piece_index(&self, piece: Piece, color: Color) -> usize {
        match color {
            Color::White => piece as usize,
            Color::Black => piece as usize + 6,
        }
    }

    fn update_occupancy(&mut self) {
        self.occupancy[0] = 0;
        self.occupancy[1] = 0;
        for i in 0..6 {
            self.occupancy[0] |= self.pieces[i];       // White pieces
            self.occupancy[1] |= self.pieces[i + 6];   // Black pieces
        }
    }

    pub fn generate_legal_moves(&self, color: Color) -> Vec<(Square, Square)> {
        // Placeholder implementation
        vec![]
    }

    pub fn evaluate(&self, color: Color) -> i32 {
        // Placeholder evaluation
        0
    }

    pub fn is_checkmate(&self, color: Color) -> bool {
        // Placeholder checkmate detection
        false
    }

    pub fn is_stalemate(&self, color: Color) -> bool {
        // Placeholder stalemate detection
        false
    }

    pub fn generate_captures(&self, color: Color) -> Vec<(Square, Square)> {
        // Placeholder capture generation
        vec![]
    }

    pub fn material_count(&self) -> i32 {
        // Placeholder material counting
        0
    }

    pub fn mobility(&self) -> u32 {
        // Placeholder mobility calculation
        0
    }

    pub fn is_complex(&self) -> bool {
        // Placeholder complexity determination
        false
    }

    pub fn piece_count(&self, piece: Piece) -> usize {
        count_bits(self.pieces[self.get_piece_index(piece, self.side_to_move)]) as usize
    }

    pub fn pawn_shield(&self, color: Color, king_square: Square) -> Bitboard {
        // Placeholder pawn shield calculation
        0
    }

    pub fn king_square(&self, color: Color) -> Square {
        // Placeholder king square retrieval
        Square::E1
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
}
