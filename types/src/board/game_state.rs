use crate::{Board, Color};

pub struct GameState;

impl GameState {
    pub fn is_checkmate(board: &Board, color: Color) -> bool {
        // Check if the current player has no legal moves and is in check
        board.generate_legal_moves(color).is_empty() && Self::is_in_check(board, color)
    }

    pub fn is_stalemate(board: &Board, color: Color) -> bool {
        // Check if the current player has no legal moves and is not in check
        board.generate_legal_moves(color).is_empty() && !Self::is_in_check(board, color)
    }

    pub fn is_in_check(board: &Board, color: Color) -> bool {
        let king_square = board.king_square(color);
        let opponent_color = color.opponent();

        // Check if any opponent's move attacks the king's square
        let opponent_moves = board.generate_legal_moves(opponent_color);
        opponent_moves.iter().any(|&(_, to_square)| to_square == king_square)
    }
    
    pub fn is_game_over(board: &Board, color: Color) -> bool {
        Self::is_checkmate(board, color) || Self::is_stalemate(board, color)
    }
}
