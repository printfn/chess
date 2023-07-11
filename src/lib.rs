#![no_std]

mod bitboard;
mod file;
mod piece;
mod pos;
mod rank;

pub use bitboard::Bitboard;
pub use file::{File, FILES};
pub use piece::Piece;
pub use pos::Pos;
pub use rank::{Rank, RANKS};

pub fn add(left: usize, right: usize) -> usize {
	left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
