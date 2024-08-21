use types::{Board, Color};

pub fn quiescence_search(board: &mut Board, alpha: i32, beta: i32, color: Color) -> i32 {
    let stand_pat = board.evaluate(color);
    if stand_pat >= beta {
        return beta;
    }

    let mut alpha = alpha.max(stand_pat);
    let mut best_eval = stand_pat;

    let moves = board.generate_captures(color);

    for (from, to) in moves {
        let piece = board.get_piece_at_square(from);
        board.make_move(from, to, piece, color);
        let eval = -quiescence_search(board, -beta, -alpha, color.opponent());
        board.unmake_move(from, to, piece, color);

        best_eval = best_eval.max(eval);
        alpha = alpha.max(eval);
        if alpha >= beta {
            break; // Beta cutoff
        }
    }

    best_eval
}
