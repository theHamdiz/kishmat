pub type Bitboard = u64;

pub const FULL_BOARD: Bitboard = 0xFFFF_FFFF_FFFF_FFFF;

#[inline(always)]
pub fn set_bit(board: &mut Bitboard, square: usize) {
    *board |= 1u64 << square;
}

#[inline(always)]
pub fn clear_bit(board: &mut Bitboard, square: usize) {
    *board &= !(1u64 << square);
}

#[inline(always)]
pub fn is_bit_set(board: Bitboard, square: usize) -> bool {
    (board & (1u64 << square)) != 0
}

#[inline(always)]
pub fn count_bits(board: Bitboard) -> u32 {
    board.count_ones() // Modern CPUs are optimized for this intrinsic
}

#[inline(always)]
pub fn get_lsb(board: Bitboard) -> usize {
    board.trailing_zeros() as usize
}
