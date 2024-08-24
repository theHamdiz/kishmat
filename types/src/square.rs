use std::str::FromStr;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    
     #[inline(always)]
    pub const fn new(rank: usize, file: usize) -> Self {
        debug_assert!(rank < 8 && file < 8, "Rank and file must be within 0..7");

        // Convert rank and file to a square index
        unsafe { std::mem::transmute((rank * 8 + file) as u8) }
    }
    
    #[inline(always)]
    pub fn to_index(self) -> usize {
        self as usize
    }

    #[inline(always)]
    pub fn from_index(index: usize) -> Self {
        debug_assert!(index < 64, "Index out of bounds for Square");
        unsafe { std::mem::transmute(index as u8) }
    }

    #[inline(always)]
    pub fn from_str(s: &str) -> Self {
        debug_assert!(s.len() == 2, "Invalid square string length");

        let bytes = s.as_bytes();
        let file = (bytes[0] - b'a') as usize; // Convert 'a'-'h' to 0-7
        let rank = (bytes[1] - b'1') as usize; // Convert '1'-'8' to 0-7

        debug_assert!(file < 8 && rank < 8, "Invalid file or rank");

        Square::new(rank, file)
    }
    
    #[inline(always)]
    pub fn rank_usize(self) -> usize {
        self.to_index() / 8
    }
    
     #[inline(always)]
    pub fn matches_hint(&self, hint: &str) -> bool {
        let hint_bytes = hint.as_bytes();

        match hint_bytes.len() {
            0 => true, // No hint, always matches
            1 => {
                // Hint could be either a file or a rank
                let hint_char = hint_bytes[0];
                self.file() == hint_char || self.rank_usize() == hint_char as usize
            }
            2 => {
                // Hint is both a file and a rank
                self.file() == hint_bytes[0] && self.rank_usize() == hint_bytes[1] as usize
            }
            _ => false, // Invalid hint
        }
    }

    #[inline(always)]
    pub fn file(&self) -> u8 {
        // Assuming Square::A1 = 0, A2 = 8, etc.
        // Return the file as 'a' to 'h' (0..7 + 'a')
        b'a' + (self.to_index() % 8) as u8
    }

    #[inline(always)]
    pub fn rank_ascii(&self) -> u8 {
        // Assuming Square::A1 = 0, A2 = 8, etc.
        // Return the rank as '1' to '8' (0..7 + '1')
        b'1' + (self.to_index() / 8) as u8
    }
}


impl FromStr for Square {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 2 {
            let bytes = s.as_bytes();
            let file = (bytes[0] - b'a') as usize;
            let rank = (bytes[1] - b'1') as usize;

            if file < 8 && rank < 8 {
                return Ok(Square::new(rank, file));
            }
        }
        Err(())
    }
}