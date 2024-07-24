use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

pub type Square = usize;

/// Core struct of the engine. Represents a chessboard with a u64.
/// A set bit is equivalent to a piece on the board.
/// MSB is in the bottom left corner (A1), while LSB is in the top right (63)
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn to_str(&self) -> String {
        let mut str = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                str.push(
                    if self.square_has_piece(square) {
                        '1'
                    } else {
                        '0'
                    }
                )
            }
            str.push('\n');
        }
        str
    }

    #[inline]
    fn square_has_piece(&self, square: Square) -> bool {
        let bit_square = 1 << square;
        return self.0 & bit_square != 0;
    }

    #[inline]
    pub fn from_square(square: Square) -> Self {
        Bitboard(1 << square)
    }

    #[inline]
    pub fn from_squares(squares: &[Square]) -> Self {
        let mut board = 0;
        for square in squares {
            board |= 1 << square;
        }
        Bitboard(board)
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    #[inline]
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(rhs.0 ^ self.0)
    }
}

impl BitXorAssign for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}