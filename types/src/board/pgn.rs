use std::str::FromStr;
use crate::board::Board;
use crate::{clear_bit, get_lsb, Color, Piece, Square};

impl Board{
    #[inline(always)]
    pub fn from_pgn(pgn: &str) -> Self {
        let mut board = Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").expect("Invalid FEN string");
    
        let moves = pgn.split_whitespace();
        for mov in moves {
            board.apply_move_from_pgn(mov);
        }
    
        board
    }

    #[inline(always)]
    pub fn apply_move_from_pgn(&mut self, mov: &str) -> &Self {
        let mov = mov.trim();
    
        match mov {
            "O-O" => self.castle_kingside(),
            "O-O-O" => self.castle_queenside(),
              _ => {
                let color = self.side_to_move;
    
                let (from_square, to_square, promotion) = self.parse_pgn_move(mov, color);
    
                // Get the piece at the starting square
                let piece = self.get_piece_at_square(from_square).expect("No piece found at source square").0;
    
                // Handle promotion if applicable
                let piece_to_move = promotion.unwrap_or(piece);
    
                // Make the move
                self.make_move(from_square, to_square, piece_to_move, color);
    
                // Handle capture
                if self.is_capture(to_square, color) {
                    self.capture_piece(to_square);
                }
    
                // Handle promotion if applicable
                if let Some(promoted_piece) = promotion {
                    self.promote_pawn(to_square, promoted_piece, color);
                }
    
                // Switch sides after the move
                self.side_to_move = self.side_to_move.opponent();
    
                self // Explicitly return `self` here to ensure the last arm returns `&Self`
            }
        }
    }


    fn parse_pgn_move(&self, mov: &str, color: Color) -> (Square, Square, Option<Piece>) {
        let mut from_square = None;
        let mut to_square = None;
        let mut promotion = None;

        // Handle pawn moves with promotion (e.g., e8=Q)
        if mov.contains('=') {
            let parts: Vec<&str> = mov.split('=').collect();
            let promo_char = parts[1].chars().next().unwrap();
            promotion = Piece::from_char(promo_char);

            let to_square_str = &parts[0][parts[0].len() - 2..];
            to_square = Some(Square::from_str(to_square_str));

            let from_square_str = &parts[0][..parts[0].len() - 2];
            from_square = self.find_pawn_source_square(from_square_str, to_square.unwrap().unwrap(), color);
        } else if mov.len() == 2 {
            // Handle simple pawn moves like "e4"
            to_square = Some(Square::from_str(mov));
            from_square = self.find_pawn_source_square("", to_square.unwrap().unwrap(), color);
        } else if mov.len() >= 3 {
            // Handle all other moves (e.g., "Nf3", "Rae1", "Qxb7", etc.)
            let (piece_char, rest) = if mov.chars().next().unwrap().is_uppercase() {
                (mov.chars().next().unwrap(), &mov[1..])
            } else {
                ('P', mov) // Pawn moves don't start with a piece character
            };

            let piece = Piece::from_char(piece_char).expect("Invalid piece character");
            let last_two = &rest[rest.len() - 2..];
            to_square = Some(Square::from_str(last_two));

            let from_hint = &rest[..rest.len() - 2];
            from_square = self.find_source_square(piece, from_hint, to_square.unwrap().unwrap(), color);
        }

        (
            from_square.expect("Unable to determine source square"),
            to_square.unwrap().expect("Unable to determine target square"),
            promotion,
        )
    }

    #[inline(always)]
    fn find_source_square(&self, piece: Piece, from_hint: &str, to_square: Square, color: Color) -> Option<Square> {
        // Implementation that finds the source square based on the piece type and possible moves
        // This includes handling disambiguation like "Nbd2", "R1d1", etc.
        // For now, this is a placeholder to show where such logic would go.
        // Efficiently check all potential squares where the piece could have moved from
        let potential_squares = self.get_potential_squares(piece, color);
        potential_squares.iter().find(|&&square| self.is_valid_move(square, to_square, piece, color) && (from_hint.is_empty() || square.matches_hint(from_hint))).copied()
    }

    fn find_pawn_source_square(&self, from_hint: &str, to_square: Square, color: Color) -> Option<Square> {
        // Similar to `find_source_square`, but specific for pawns.
        // Handle cases where the move is ambiguous and disambiguation is necessary.
        let potential_squares = self.get_potential_squares(Piece::Pawn, color);
        potential_squares.iter().find(|&&square| self.is_valid_pawn_move(square, to_square, color) && (from_hint.is_empty() || square.matches_hint(from_hint))).copied()
    }

    #[inline(always)]
    fn get_potential_squares(&self, piece: Piece, color: Color) -> Vec<Square> {
        // Return a vector of squares where this piece might be, based on the bitboards.
        let piece_index = self.get_piece_index(piece, color);
        let mut piece_positions = self.pieces[piece_index];
        let mut squares = Vec::new();

        while piece_positions != 0 {
            let square_index = get_lsb(piece_positions);
            squares.push(Square::from_index(square_index));
            clear_bit(&mut piece_positions, square_index);
        }

        squares
    }
    
}