use types::{Board, Color};
use crate::alpha_beta::alpha_beta;
use crate::transposition::TranspositionTable;
use crate::quiescence::quiescence_search;

pub fn negamax(
    board: &mut Board,
    depth: i32,
    alpha: i32,
    beta: i32,
    color: Color,
    transposition_table: &mut TranspositionTable,
) -> i32 {
    if depth == 0 {
        return quiescence_search(board, alpha, beta, color);
    }

    let mut alpha = alpha;
    let mut max_eval = i32::MIN;

    let moves = board.generate_legal_moves(color);

    if moves.is_empty() {
        return evaluate_terminal(board, color);
    }

    for (from, to) in moves {
        let piece = board.get_piece_at_square(from);
        board.make_move(from, to, piece, color);
        let eval = -negamax(board, depth - 1, -beta, -alpha, color.opponent(), transposition_table);
        board.unmake_move(from, to, piece, color);

        max_eval = max_eval.max(eval);
        alpha = alpha.max(eval);
        if alpha >= beta {
            break; // Beta cutoff
        }
    }

    max_eval
}

fn evaluate_terminal(board: &Board, color: Color) -> i32 {
    if board.is_checkmate(color) {
        return i32::MIN + 1; // Immediate checkmate
    } else if board.is_stalemate(color) {
        return 0; // Stalemate
    }
    0
}
