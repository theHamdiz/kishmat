use crate::alpha_beta::alpha_beta;
use crate::transposition::TranspositionTable;
use types::{Board, Color};

pub fn null_move_pruning(
    board: &mut Board,
    depth: i32,
    alpha: i32,
    beta: i32,
    color: Color,
    transposition_table: &mut TranspositionTable,
) -> i32 {
    if depth <= 1 || board.is_endgame() {
        return alpha_beta(board, depth, alpha, beta, color, transposition_table);
    }

    // Perform a null move (skip opponent's turn)
    board.do_null_move();
    let score = -alpha_beta(board, depth - 1 - 2, -beta, -alpha, color.opponent(), transposition_table);
    board.undo_null_move();

    if score >= beta {
        return beta;
    }

    alpha_beta(board, depth, alpha, beta, color, transposition_table)
}
