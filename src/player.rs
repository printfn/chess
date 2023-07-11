use core::{fmt, ops};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Player {
	White,
	Black,
}

impl ops::Not for Player {
	type Output = Self;

	fn not(self) -> Self::Output {
		match self {
			Player::White => Player::Black,
			Player::Black => Player::White,
		}
	}
}

impl fmt::Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s = match self {
			Player::White => "White",
			Player::Black => "Black",
		};
		write!(f, "{s}")
	}
}
