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

                for pieceMove in possible_moves {
                    if self.is_capture(pieceMove.0, color) {
                        captures.push((from_square, pieceMove.0));
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

    #[inline(always)]
    pub(crate) fn capture_piece(&mut self, square: Square) {
        let square_index = square.to_index();

        // Check and remove the piece from the board
        for piece_type in 0..6 {
            // Check if a white piece is on the square
            if is_bit_set(self.pieces[piece_type], square_index) {
                clear_bit(&mut self.pieces[piece_type], square_index);
                break;
            }
            // Check if a black piece is on the square
            if is_bit_set(self.pieces[piece_type + 6], square_index) {
                clear_bit(&mut self.pieces[piece_type + 6], square_index);
                break;
            }
        }

        // Update the occupancy bitboards
        self.update_occupancy();
    }
}