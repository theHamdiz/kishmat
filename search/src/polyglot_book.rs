use std::fs::File;
use std::io::{self, Read, BufReader};
use std::path::Path;
use types::Square;

#[derive(Debug)]
struct PolyglotEntry {
    key: u64,
    move_: u16,
    weight: u16,
    learn: u32,
    next: u32,
}

impl PolyglotEntry {
    fn from_bytes(bytes: &[u8]) -> PolyglotEntry {
        let key = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
        let move_ = u16::from_be_bytes(bytes[8..10].try_into().unwrap());
        let weight = u16::from_be_bytes(bytes[10..12].try_into().unwrap());
        let learn = u32::from_be_bytes(bytes[12..16].try_into().unwrap());

        PolyglotEntry {
            key,
            move_,
            weight,
            learn,
            next: 0, // Polyglot doesn't use this field, but we keep it for completeness.
        }
    }

    fn polyglot_move_to_squares(mv: u16) -> (Square, Square) {
        let from_square = Square::from_index(((mv >> 6) & 0x3f) as usize);
        let to_square = Square::from_index((mv & 0x3f) as usize);
        (from_square, to_square)
    }
}

fn read_polyglot_book<P: AsRef<Path>>(path: P) -> io::Result<Vec<PolyglotEntry>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut entries = Vec::new();
    let mut buffer = [0; 16];

    while reader.read_exact(&mut buffer).is_ok() {
        let entry = PolyglotEntry::from_bytes(&buffer);
        entries.push(entry);
    }

    Ok(entries)
}

fn find_polyglot_move(entries: &[PolyglotEntry], zobrist_hash: u64) -> Option<PolyglotEntry> {
    let mut best_entry: Option<PolyglotEntry> = None;

    for entry in entries {
        if entry.key == zobrist_hash {
            match &best_entry {
                Some(current_best) => {
                    // Replace the current best entry if the new one has a higher weight
                    if entry.weight > current_best.weight {
                        best_entry = Some(entry.clone());
                    }
                }
                None => {
                    // First matching entry found
                    best_entry = Some(*entry.clone());
                }
            }
        }
    }

    best_entry
}
