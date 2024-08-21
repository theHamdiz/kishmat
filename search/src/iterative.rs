use crate::alpha_beta::alpha_beta;
use crate::transposition::TranspositionTable;
use types::{Board, Color, Square};

pub fn iterative_deepening(
    board: &mut Board,
    max_depth: i32,
    color: Color,
    transposition_table: &mut TranspositionTable,
) -> (Square, Square) {
    let mut best_move = None;
    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;

    for depth in 1..=max_depth {
        let eval = alpha_beta(board, depth, alpha, beta, color, transposition_table);
        if let Some(m) = board.best_move() {
            best_move = Some(m);
        }
    }

    best_move.expect("No valid move found")
}
