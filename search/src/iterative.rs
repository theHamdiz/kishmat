use crate::transposition::TranspositionTable;
use types::{Board, Color, Square, Zobrist};
use crate::Search;

impl Search{
     pub fn iterative_deepening(
        board: &mut Board,
        max_depth: i32,
        color: Color,
        transposition_table: &mut TranspositionTable,
    ) -> (Square, Square) {
       let mut best_move = None;
    let mut best_eval = i32::MIN;
    let mut alpha = i32::MIN;
    let beta = i32::MAX;
    let zobrist_key = board.compute_zobrist_hash(&Zobrist::new());

    for depth in 1..=max_depth {
        let eval = Self::alpha_beta(board, depth, alpha, beta, color, transposition_table).0;

        // Assuming we're maximizing for the current color
        if eval > best_eval {
            best_eval = eval;
            if let Some(m) = transposition_table.lookup_best_move(zobrist_key) {
                best_move = Some(m);
            }
        }

        // Optionally update alpha for more aggressive pruning
        alpha = alpha.max(best_eval);
    }

    best_move.expect("No valid move found")
}
}
