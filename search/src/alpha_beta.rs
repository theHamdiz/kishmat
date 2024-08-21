use crate::negamax::negamax;
use crate::transposition::TranspositionTable;
use types::{Board, Color};

pub fn alpha_beta(
    board: &mut Board,
    depth: i32,
    alpha: i32,
    beta: i32,
    color: Color,
    transposition_table: &mut TranspositionTable,
) -> i32 {
    if depth == 0 {
        return negamax(board, depth, alpha, beta, color, transposition_table);
    }

    negamax(board, depth, alpha, beta, color, transposition_table)
}
