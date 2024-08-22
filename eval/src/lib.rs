pub mod material;
pub mod psqt;
pub mod king_safety;
pub mod pawn_structure;
pub mod nnue;
pub mod evaluation;

pub use evaluation::Evaluation;




impl Evaluation {
    pub fn new() -> Self {
      Self::default()
    }
}
