use eval::Evaluation;
use crate::transposition::TranspositionTable;
use types::{Board, Color, GameState, Square};
use crate::Search;

impl Search{
 /// This function returns the best move for the given board and color by performing a search.
    pub fn best_move(
        board: &mut Board,
        max_depth: i32,
        color: Color,
        transposition_table: &mut TranspositionTable,
    ) -> (Square, Square) {
        let (_, best_move) = Self::negamax(board, max_depth, i32::MIN, i32::MAX, color, transposition_table);
        best_move.expect("No valid move found")
    }

 pub fn negamax(
    board: &mut Board,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    color: Color,
    transposition_table: &mut TranspositionTable,
) -> (i32, Option<(Square, Square)>) {
    if depth == 0 {
        return (Evaluation::evaluate(board, color), None);
    }

    if GameState::is_checkmate(board, color) {
        return (-9999, None); // Negative high value for checkmate
    } else if GameState::is_stalemate(board, color) {
        return (0, None); // Stalemate results in a draw
    }

    let mut best_score = i32::MIN;
    let mut best_move = None;

    let legal_moves = board.generate_legal_moves(color);
    if legal_moves.is_empty() {
        return (0, None); // No legal moves, return a draw score
    }

    // Handle potential overflow for alpha and beta when negating
    let mut neg_alpha = if alpha == i32::MIN { i32::MAX } else { -alpha };
    let mut neg_beta = if beta == i32::MIN { i32::MAX } else { -beta };

    for m in legal_moves {
        let piece = board.get_piece_at_square(m.0).expect("No piece found at source square").0;

        board.make_move(m.0, m.1, piece, color);
        let mut score = Self::negamax(board, depth - 1, neg_beta, neg_alpha, color.opponent(), transposition_table).0;
        board.unmake_move(m.0, m.1, piece, color);

        // Handle the potential overflow condition for the score
        if score == i32::MIN {
            score = i32::MAX;
        } else {
            score = -score;
        }

        if score > best_score {
            best_score = score;
            best_move = Some((m.0, m.1));
        }

        alpha = alpha.max(score);
        if alpha >= beta {
            break; // Beta cutoff
        }

        // Update neg_alpha for the next iteration
        neg_alpha = if alpha == i32::MIN { i32::MAX } else { -alpha };
    }

    (best_score, best_move)
}


    // fn evaluate_terminal(board: &Board, color: Color) -> i32 {
    //      if GameState::is_checkmate(board, color) {
    //         return -9999; // Negative high value for checkmate
    //     } else if GameState::is_stalemate(board, color) {
    //         return 0; // Stalemate results in a draw
    //     }
    //     0
    // }

}