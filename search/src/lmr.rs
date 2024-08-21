use crate::alpha_beta::alpha_beta;
use crate::transposition::TranspositionTable;
use types::{Board, Color};

pub fn late_move_reductions(
    board: &mut Board,
    depth: i32,
    alpha: i32,
    beta: i32,
    color: Color,
    transposition_table: &mut TranspositionTable,
) -> i32 {
    if depth <= 1 {
        return alpha_beta(board, depth, alpha, beta, color, transposition_table);
    }

    let moves = board.generate_legal_moves(color);
    let mut best_eval = i32::MIN;
    let mut alpha = alpha;

    for (index, (from, to)) in moves.iter().enumerate() {
        let reduction = if index > 3 { 1 } else { 0 };
        let (piece, color) = board.get_piece_at_square(*from).expect("Could not get piece at a given square");
        board.make_move(*from, *to, piece, color);
        let eval = -alpha_beta(board, depth - 1 - reduction, -beta, -alpha, color.opponent(), transposition_table);
        board.unmake_move(*from, *to, piece, color);

        best_eval = best_eval.max(eval);
        alpha = alpha.max(eval);
        if alpha >= beta {
            break; // Beta cutoff
        }
    }

    best_eval
}
