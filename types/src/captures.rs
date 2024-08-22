use crate::{clear_bit, get_lsb, is_bit_set, Board, Color, Piece, Square};

impl Board{
    
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
}