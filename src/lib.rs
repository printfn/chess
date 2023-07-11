#![no_std]

mod bitboard;
mod board;
mod file;
mod piece;
mod player;
mod pos;
mod rank;

pub use bitboard::Bitboard;
pub use board::Board;
pub use file::{File, FILES};
pub use piece::{Piece, HOME_ROW};
pub use player::Player;
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
