use crate::{is_bit_set, Board, Color, Piece, Square};

impl Board{
    #[inline(always)]
    pub(crate) fn is_valid_move(&self, from_square: Square, to_square: Square, piece: Piece, color: Color) -> bool {
        if from_square == to_square {
            return false; // A piece can't move to the same square
        }

        // Determine if the destination square is occupied by a friendly piece
        if self.is_occupied_by_friendly(to_square, color) {
            return false; // Can't move to a square occupied by a friendly piece
        }

        match piece {
            Piece::Pawn => self.is_valid_pawn_move(from_square, to_square, color),
            Piece::Knight => self.is_valid_knight_move(from_square, to_square),
            Piece::Bishop => self.is_valid_bishop_move(from_square, to_square),
            Piece::Rook => self.is_valid_rook_move(from_square, to_square),
            Piece::Queen => self.is_valid_queen_move(from_square, to_square),
            Piece::King => self.is_valid_king_move(from_square, to_square),
        }
    }
    
    #[inline(always)]
    pub fn is_occupied_by_friendly(&self, square: Square, color: Color) -> bool {
        is_bit_set(self.occupancy[color as usize], square.to_index())
    }

    #[inline(always)]
    pub fn is_occupied(&self, square: Square) -> bool {
        let all_pieces = self.occupancy[0] | self.occupancy[1];
        is_bit_set(all_pieces, square.to_index())
    }
    
    #[inline(always)]
    pub fn is_occupied_by_opponent(&self, square: Square, color: Color) -> bool {
        is_bit_set(self.occupancy[color.opponent() as usize], square.to_index())
    }

    
    #[inline(always)]
    fn is_valid_knight_move(&self, from_square: Square, to_square: Square) -> bool {
        let from_index = from_square.to_index();
        let to_index = to_square.to_index();
        let diff = (from_index as isize - to_index as isize).abs();

        // Valid knight moves are 6 or 10 squares away (in bitboard representation)
        diff == 6 || diff == 10
    }

    #[inline(always)]
    fn is_valid_bishop_move(&self, from_square: Square, to_square: Square) -> bool {
        // Bishops move diagonally, so the difference between ranks and files should be equal
        self.is_diagonal_move(from_square, to_square) && !self.is_path_obstructed(from_square, to_square)
    }

    #[inline(always)]
    fn is_valid_rook_move(&self, from_square: Square, to_square: Square) -> bool {
        // Rooks move horizontally or vertically
        self.is_straight_move(from_square, to_square) && !self.is_path_obstructed(from_square, to_square)
    }

    #[inline(always)]
    fn is_valid_queen_move(&self, from_square: Square, to_square: Square) -> bool {
        // Queens move like both a rook and a bishop
        (self.is_straight_move(from_square, to_square) || self.is_diagonal_move(from_square, to_square))
            && !self.is_path_obstructed(from_square, to_square)
    }

    #[inline(always)]
    fn is_valid_king_move(&self, from_square: Square, to_square: Square) -> bool {
        let from_index = from_square.to_index();
        let to_index = to_square.to_index();
        let diff = (from_index as isize - to_index as isize).abs();

        // Kings move one square in any direction
        diff == 1 || diff == 7 || diff == 8 || diff == 9
    }

    #[inline(always)]
    fn is_straight_move(&self, from_square: Square, to_square: Square) -> bool {
        from_square.rank_usize() == to_square.rank_usize() || from_square.file() == to_square.file()
    }

    #[inline(always)]
    fn is_diagonal_move(&self, from_square: Square, to_square: Square) -> bool {
        (from_square.rank_usize() as isize - to_square.rank_usize() as isize).abs() == 
        (from_square.file() as isize - to_square.file() as isize).abs()
    }

    #[inline(always)]
    fn is_path_obstructed(&self, from_square: Square, to_square: Square) -> bool {
        let from_index = from_square.to_index();
        let to_index = to_square.to_index();
        let direction = self.get_move_direction(from_index, to_index);

        let mut current_index = (from_index as isize + direction) as usize;

        while current_index != to_index {
            if self.is_occupied(Square::from_index(current_index)) {
                return true; // Path is obstructed
            }
            current_index = (current_index as isize + direction) as usize;
        }

        false
    }

    #[inline(always)]
    fn get_move_direction(&self, from_index: usize, to_index: usize) -> isize {
        let rank_diff = to_index as isize / 8 - from_index as isize / 8;
        let file_diff = to_index as isize % 8 - from_index as isize % 8;

        match (rank_diff, file_diff) {
            (0, _) => file_diff.signum(),   // Horizontal move
            (_, 0) => rank_diff.signum() * 8, // Vertical move
            (_, _) => rank_diff.signum() * 8 + file_diff.signum(), // Diagonal move
        }
    }
}