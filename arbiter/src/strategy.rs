use types::{Board, Color, Square};
use search::{negamax, iterative_deepening, late_move_reductions};
use eval::{Evaluation};
use crate::position_type::PositionType;
use search::TranspositionTable;

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
    iterative_deepening(board, max_depth, color, transposition_table)
}

fn negamax_strategy(board: &mut Board, max_depth: i32, color: Color, transposition_table: &mut TranspositionTable) -> (Square, Square) {
    let mut best_move = None;
    let eval = negamax(board, max_depth, i32::MIN, i32::MAX, color, transposition_table);
    if let Some(m) = board.best_move() {
        best_move = Some(m);
    }
    best_move.expect("No valid move found")
}

fn lmr_strategy(board: &mut Board, max_depth: i32, color: Color, transposition_table: &mut TranspositionTable) -> (Square, Square) {
    let mut best_move = None;
    let eval = late_move_reductions(board, max_depth, i32::MIN, i32::MAX, color, transposition_table);
    if let Some(m) = board.best_move() {
        best_move = Some(m);
    }
    best_move.expect("No valid move found")
}

pub fn choose_evaluation_strategy(position_type: PositionType) -> fn(&Board) -> i32 {
    match position_type {
        PositionType::Endgame => endgame_evaluation_strategy,
        PositionType::Complex => complex_evaluation_strategy,
        _ => general_evaluation_strategy,
    }
}

fn general_evaluation_strategy(board: &Board) -> i32 {
    let mut evaluation = Evaluation::new();
    evaluation.evaluate(board)
}

fn endgame_evaluation_strategy(board: &Board) -> i32 {
    let mut evaluation = Evaluation::new();
    evaluation.evaluate(board)
}

fn complex_evaluation_strategy(board: &Board) -> i32 {
    let mut evaluation = Evaluation::new();
    evaluation.evaluate(board)
}
