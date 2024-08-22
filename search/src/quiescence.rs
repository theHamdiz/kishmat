use eval::evaluation::Evaluation;
use types::{Board, Color};
use crate::Search;

impl Search{
    pub fn quiescence_search(board: &mut Board, alpha: i32, beta: i32, color: Color) -> i32 {

        let stand_pat = Evaluation::evaluate(&board, color);
    if stand_pat >= beta {
        return beta;
    }

    let mut alpha = alpha.max(stand_pat);
    let mut best_eval = stand_pat;

    let moves = board.generate_captures(color);

    for (from, to) in moves {
        let (piece, color) = board.get_piece_at_square(from).expect("Could not get piece at a given square");
        board.make_move(from, to, piece, color);
        let eval = -Self::quiescence_search(board, -beta, -alpha, color.opponent());
        board.unmake_move(from, to, piece, color);

        best_eval = best_eval.max(eval);
        alpha = alpha.max(eval);
        if alpha >= beta {
            break; // Beta cutoff
        }
    }

    best_eval
}
}
