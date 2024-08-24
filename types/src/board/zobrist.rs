use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub struct Zobrist {
    pub(crate) piece_keys: [[[u64; 64]; 12]; 2], // [color][piece_type][square]
    pub(crate) castling_keys: [u64; 16],         // Castling rights (4 bits: KQkq)
    pub(crate) en_passant_keys: [u64; 8],        // En passant file (8 possible files)
    pub(crate) side_to_move_key: u64,            // Side to move
}

impl Default for Zobrist {
    fn default() -> Self {
        Self::new()
    }
}

impl Zobrist {
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(0); // Use a fixed seed for reproducibility
        let mut piece_keys = [[[0u64; 64]; 12]; 2];
        let mut castling_keys = [0u64; 16];
        let mut en_passant_keys = [0u64; 8];
        let side_to_move_key = rng.random();

        // Initialize piece keys
        for color in 0..2 {
            for piece in 0..12 {
                for square in 0..64 {
                    piece_keys[color][piece][square] = rng.random();
                }
            }
        }

        // Initialize castling keys
        for i in 0..16 {
            castling_keys[i] = rng.random();
        }

        // Initialize en passant keys
        for i in 0..8 {
            en_passant_keys[i] = rng.random();
        }

        Self {
            piece_keys,
            castling_keys,
            en_passant_keys,
            side_to_move_key,
        }
    }
}
