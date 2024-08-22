use std::collections::HashMap;
use types::Square;

pub struct TranspositionTable {
    table: HashMap<u64, TranspositionEntry>,
}

#[derive(Copy, Clone)]
pub struct TranspositionEntry {
    pub depth: i32,
    pub score: i32,
    pub best_move: Option<(Square, Square)>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Exact,
    Alpha,
    Beta,
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

    pub fn store(&mut self, zobrist_key: u64, entry: TranspositionEntry) {
        self.table.insert(zobrist_key, entry);
    }

    pub fn lookup_best_move(&self, zobrist_key: u64) -> Option<(Square, Square)> {
        if let Some(entry) = self.table.get(&zobrist_key) {
            entry.best_move
        } else {
            None
        }
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new()
    }
}
