use crate::transposition::TranspositionTable;
use types::{Board, Color};
use crate::Search;

impl Search{
    pub fn null_move_pruning(
    board: &mut Board,
    depth: i32,
    alpha: i32,
    beta: i32,
    color: Color,
    transposition_table: &mut TranspositionTable,
) -> i32 {
    if depth <= 1 || board.is_endgame() {
        return Self::alpha_beta(board, depth, alpha, beta, color, transposition_table).0;
    }

    // Perform a null move (skip opponent's turn)
    board.do_null_move();
    let score = -Self::alpha_beta(board, depth - 1 - 2, -beta, -alpha, color.opponent(), transposition_table).0;
    board.undo_null_move();

    if score >= beta {
        return beta;
    }

    Self::alpha_beta(board, depth, alpha, beta, color, transposition_table).0
}
}
