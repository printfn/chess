use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Player {
	White,
	Black,
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
