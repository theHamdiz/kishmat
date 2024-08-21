use std::collections::HashMap;
use types::{Board, Square};

pub struct TranspositionTable {
    table: HashMap<u64, TranspositionEntry>,
}

#[derive(Copy, Clone)]
pub struct TranspositionEntry {
    pub depth: i32,
    pub score: i32,
    pub best_move: Option<(Square, Square)>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn get(&self, key: u64) -> Option<TranspositionEntry> {
        self.table.get(&key).copied()
    }

    pub fn insert(&mut self, key: u64, entry: TranspositionEntry) {
        self.table.insert(key, entry);
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new()
    }
}
