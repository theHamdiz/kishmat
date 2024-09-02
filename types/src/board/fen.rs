use std::str::FromStr;
use crate::board::Board;
use crate::{set_bit, Color, Piece, Square};

impl Board{
        pub fn from_fen(fen: &str) -> Option<Self> {
            let mut board = Self::default(); // Start with a default board
    
            let parts: Vec<&str> = fen.split_whitespace().collect();
            assert_eq!(parts.len(), 6, "Invalid FEN string");
    
            // 1. Piece placement
            let rows: Vec<&str> = parts[0].split('/').collect();
            for (rank, row) in rows.iter().enumerate() {
                let mut file = 0;
                for ch in row.chars() {
                    match ch {
                        '1'..='8' => file += ch.to_digit(10)? as usize,
                        'r' => board.set_piece(Square::new(rank, file), Piece::Rook, Color::Black),
                        'n' => board.set_piece(Square::new(rank, file), Piece::Knight, Color::Black),
                        'b' => board.set_piece(Square::new(rank, file), Piece::Bishop, Color::Black),
                        'q' => board.set_piece(Square::new(rank, file), Piece::Queen, Color::Black),
                        'k' => board.set_piece(Square::new(rank, file), Piece::King, Color::Black),
                        'p' => board.set_piece(Square::new(rank, file), Piece::Pawn, Color::Black),
                        'R' => board.set_piece(Square::new(rank, file), Piece::Rook, Color::White),
                        'N' => board.set_piece(Square::new(rank, file), Piece::Knight, Color::White),
                        'B' => board.set_piece(Square::new(rank, file), Piece::Bishop, Color::White),
                        'Q' => board.set_piece(Square::new(rank, file), Piece::Queen, Color::White),
                        'K' => board.set_piece(Square::new(rank, file), Piece::King, Color::White),
                        'P' => board.set_piece(Square::new(rank, file), Piece::Pawn, Color::White),
                        _ => panic!("Invalid character in FEN"),
                    }
                    file += 1;
                }
        }

        // 2. Active color
        board.side_to_move = if parts[1] == "w" { Color::White } else { Color::Black };

        // 3. Castling availability
        board.castling_rights = 0;
        if parts[2].contains('K') { board.castling_rights |= 1 << 0; }
        if parts[2].contains('Q') { board.castling_rights |= 1 << 1; }
        if parts[2].contains('k') { board.castling_rights |= 1 << 2; }
        if parts[2].contains('q') { board.castling_rights |= 1 << 3; }

        // 4. En passant target square
        board.en_passant = if parts[3] != "-" {
            Some(Square::from_str(parts[3]).unwrap())
        } else {
            None
        };

        // 5. Halfmove clock
        board.halfmove_clock = parts[4].parse().unwrap();

        // 6. Fullmove number
        board.fullmove_number = parts[5].parse().unwrap();

        Some(board)
    }

    fn set_piece(&mut self, square: Square, piece: Piece, color: Color) {
        let piece_index = self.get_piece_index(piece, color);
        set_bit(&mut self.pieces[piece_index], square.to_index());
        self.update_occupancy();
    }
}