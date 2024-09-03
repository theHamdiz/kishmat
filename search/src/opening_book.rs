use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read};
use types::Square;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct OpeningBook {
    file_path: String, // Path to the opening book file
    book: HashMap<u64, Vec<(u16, String)>>, // Hash of position -> (move, opening name)
}

impl OpeningBook {
    // Constructor to create an OpeningBook with a file path
    pub fn new(file_path: &str) -> Self {
        let exe_path = env::current_exe().expect("Failed to get current executable path");
        let exe_dir = exe_path.parent().expect("Failed to get the executable directory").parent().unwrap().parent().unwrap();
         
        let relative_path = Path::new(file_path);
        let book_file: PathBuf = exe_dir.join(relative_path);
        OpeningBook {
            file_path: book_file.to_str().unwrap().to_string(),
            book: HashMap::new(),
        }
    }

    // Method to load a Polyglot book from the specified file
    pub fn load_from_file(&mut self) -> Result<(), io::Error> {
        println!("Opening book file: {}", self.file_path);
        let file = File::open(&self.file_path)?;
        let mut reader = BufReader::new(file);

        let mut buffer = [0; 16]; // Each entry in Polyglot book is 16 bytes
        while reader.read_exact(&mut buffer).is_ok() {
            let position_hash = u64::from_be_bytes(buffer[0..8].try_into().unwrap());
            let polyglot_move = u16::from_be_bytes(buffer[8..10].try_into().unwrap());
            let opening_name = String::from_utf8_lossy(&buffer[10..]).to_string();

            // Insert the move into the book with its associated opening name
            self.book.entry(position_hash)
                .or_default()
                .push((polyglot_move, opening_name));
        }

        println!("Opening book loaded successfully");
        Ok(())
    }

    // Method to retrieve a move from the book based on the current board position
    pub fn get_move(&self, position_hash: u64) -> Option<(u16, String)> {
        self.book.get(&position_hash).and_then(|moves| moves.first().cloned())
    }

    // Method to get the opening name based on the current board position
    pub fn get_opening_name(&self, position_hash: u64) -> Option<String> {
        self.book.get(&position_hash).and_then(|moves| moves.first().map(|(_, name)| name.clone()))
    }
    
    // Convert a u16 Polyglot move to (Square, Square) format
    pub fn polyglot_move_to_squares(polyglot_move: u16) -> (Square, Square) {
        let from_square = Square::from_index(((polyglot_move >> 6) & 0x3F) as usize); // Extract the source square
        let to_square = Square::from_index((polyglot_move & 0x3F) as usize); // Extract the destination square
        (from_square, to_square)
    }
}
