use crate::transposition::TranspositionTable;
use types::{Board, Color, Square};
use crate::Search;

impl Search{
    pub fn alpha_beta(
        board: &mut Board,
        depth: i32,
        alpha: i32,
        beta: i32,
        color: Color,
        transposition_table: &mut TranspositionTable,
) -> (i32, Option<(Square, Square)>) {
    if depth == 0 {
        return Self::negamax(board, depth, alpha, beta, color, transposition_table);
    }

    Self::negamax(board, depth, alpha, beta, color, transposition_table)
}
}
