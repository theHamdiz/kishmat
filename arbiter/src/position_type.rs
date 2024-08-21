use types::{Board, Piece};

#[derive(Debug, PartialEq, Eq)]
pub enum PositionType {
    Open,
    Closed,
    SemiOpen,
    SemiClosed,
    Endgame,
    Complex,
    Trivial,
}

pub fn determine_position_type(board: &Board) -> PositionType {
    let material_count = board.material_count();
    let pawn_count = board.piece_count(Piece::Pawn);
    let mobility = board.mobility();

    if material_count < 20 {
        PositionType::Endgame
    } else if mobility > 50 && pawn_count < 6 {
        PositionType::Open
    } else if mobility < 30 && pawn_count > 8 {
        PositionType::Closed
    } else if mobility > 50 && pawn_count > 6 {
        PositionType::SemiOpen
    } else if mobility < 50 && pawn_count > 6 {
        PositionType::SemiClosed
    } else if board.is_complex() {
        PositionType::Complex
    } else {
        PositionType::Trivial
    }
}
