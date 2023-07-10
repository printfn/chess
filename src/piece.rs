use core::fmt;

/// Represents a chess piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Piece {
	Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
}

impl Piece {
	/// Returns the piece's notation as used in algebraic notation.
	/// Pawns return an empty string.
	pub fn notation(self) -> &'static str {
		match self {
			Piece::Pawn => "",
			Piece::Knight => "N",
			Piece::Bishop => "B",
			Piece::Rook => "R",
			Piece::Queen => "Q",
			Piece::King => "K",
		}
	}

	/// Returns the piece's name.
	pub fn name(self) -> &'static str {
		match self {
			Piece::Pawn => "Pawn",
			Piece::Knight => "Knight",
			Piece::Bishop => "Bishop",
			Piece::Rook => "Rook",
			Piece::Queen => "Queen",
			Piece::King => "King",
		}
	}
}

impl fmt::Display for Piece {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.name())
	}
}
