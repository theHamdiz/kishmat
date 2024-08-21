pub mod strategy;
pub mod position_type;

use strategy::{choose_search_strategy, choose_evaluation_strategy};
use types::{Board, Color, Square};
use search::TranspositionTable;

pub struct AdaptiveEngine {
    transposition_table: TranspositionTable,
}

impl AdaptiveEngine {
    pub fn new() -> Self {
        Self {
            transposition_table: TranspositionTable::new(),
        }
    }

    pub fn search_best_move(&mut self, board: &mut Board, max_depth: i32, color: Color) -> (Square, Square) {
        let position_type = position_type::determine_position_type(board);
        let search_strategy = choose_search_strategy(position_type);
        search_strategy(board, max_depth, color, &mut self.transposition_table)
    }

    pub fn evaluate_position(&self, board: &Board) -> i32 {
        let position_type = position_type::determine_position_type(board);
        let evaluation_strategy = choose_evaluation_strategy(position_type);
        evaluation_strategy(board)
    }
}
