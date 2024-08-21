pub mod material;
pub mod psqt;
pub mod king_safety;
pub mod pawn_structure;
pub mod nnue;

use material::evaluate_material;
use psqt::evaluate_psqt;
use king_safety::evaluate_king_safety;
use pawn_structure::evaluate_pawn_structure;
use types::Board;

#[derive(Copy, Clone, Default)]
pub struct Evaluation {
    pub material: i32,
    pub psqt: i32,
    pub king_safety: i32,
    pub pawn_structure: i32,
}


impl Evaluation {
    pub fn new() -> Self {
      Self::default()
    }

    #[inline(always)]
    pub fn evaluate(&mut self, board: &Board) -> i32 {
        self.material = evaluate_material(board);
        self.psqt = evaluate_psqt(board);
        self.king_safety = evaluate_king_safety(board);
        self.pawn_structure = evaluate_pawn_structure(board);

        // Aggregate the scores
        self.material + self.psqt + self.king_safety + self.pawn_structure
    }
}
