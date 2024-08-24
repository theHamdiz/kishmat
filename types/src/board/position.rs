use crate::{clear_bit, get_lsb, Bitboard, Board, Color, Piece, Square};

impl Board{
    pub fn starting_position() -> Option<Board> {
        // Initialize the board with the standard starting FEN string
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
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
    
    /// Returns the square where the king of the given color is located.
    #[inline(always)]
    pub fn king_square(&self, color: Color) -> Square {
        let king_index = self.get_piece_index(Piece::King, color);
        let king_bb = self.pieces[king_index];
        debug_assert!(king_bb != 0, "King must exist on the board.");
        Square::from_index(get_lsb(king_bb))
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

    #[inline(always)]
    pub fn occupancy(&self) -> Bitboard {
        self.occupancy[0] | self.occupancy[1]
    }
    
    #[inline(always)]
    pub(crate) fn is_en_passant_target(&self, square: Square) -> bool {
        if let Some(ep_square) = self.en_passant {
            return ep_square == square;
        }
        false
    }
}