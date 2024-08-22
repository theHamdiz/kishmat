use types::{Board, Color, Square};
use search::{Search};
use eval::{Evaluation};
use crate::position_type::PositionType;
use search::transposition::TranspositionTable;

pub fn choose_search_strategy(position_type: PositionType) -> fn(&mut Board, i32, Color, &mut TranspositionTable) -> (Square, Square) {
    match position_type {
        PositionType::Open | PositionType::SemiOpen => iterative_deepening_strategy,
        PositionType::Closed | PositionType::SemiClosed => lmr_strategy,
        PositionType::Endgame => negamax_strategy,
        PositionType::Complex => iterative_deepening_strategy,
        PositionType::Trivial => negamax_strategy,
    }
}

fn iterative_deepening_strategy(board: &mut Board, max_depth: i32, color: Color, transposition_table: &mut TranspositionTable) -> (Square, Square) {
    Search::iterative_deepening(board, max_depth, color, transposition_table)
}

fn negamax_strategy(board: &mut Board, max_depth: i32, color: Color, transposition_table: &mut TranspositionTable) -> (Square, Square) {
    Search::best_move(board, max_depth, color, transposition_table)
}

fn lmr_strategy(board: &mut Board, max_depth: i32, color: Color, transposition_table: &mut TranspositionTable) -> (Square, Square) {
    let mut best_move = None;
    let mut best_score = i32::MIN;
    let mut alpha = i32::MIN;
    let beta = i32::MAX;

    let legal_moves = board.generate_legal_moves(color);
    for m in legal_moves {
        let piece = board.get_piece_at_square(m.0).expect("No piece found at source square").0;

        board.make_move(m.0, m.1, piece, color);
        let score = -Search::late_move_reductions(board, max_depth - 1, -beta, -alpha, color.opponent(), transposition_table);
        board.unmake_move(m.0, m.1, piece, color);

        if score > best_score {
            best_score = score;
            best_move = Some(m);
        }

        alpha = alpha.max(score);
        if alpha >= beta {
            break; // Beta cutoff
        }
    }

    best_move.expect("No valid move found")
}


pub fn choose_evaluation_strategy(position_type: PositionType) -> fn(&Board, color: Color) -> i32 {
    match position_type {
        PositionType::Endgame => endgame_evaluation_strategy,
        PositionType::Complex => complex_evaluation_strategy,
        _ => general_evaluation_strategy,
    }
}

fn general_evaluation_strategy(board: &Board, color: Color) -> i32 {
    Evaluation::evaluate(board, color)
}

fn endgame_evaluation_strategy(board: &Board, color: Color) -> i32 {
    Evaluation::evaluate(board, color)
}

fn complex_evaluation_strategy(board: &Board, color: Color) -> i32 {
    Evaluation::evaluate(board, color)
}
