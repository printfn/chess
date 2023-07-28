use core::fmt;

use crate::Player;

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

	pub fn ascii_char(self, player: Player) -> char {
		match (player, self) {
			(Player::White, Piece::Pawn) => 'P',
			(Player::White, Piece::Knight) => 'N',
			(Player::White, Piece::Bishop) => 'B',
			(Player::White, Piece::Rook) => 'R',
			(Player::White, Piece::Queen) => 'Q',
			(Player::White, Piece::King) => 'K',
			(Player::Black, Piece::Pawn) => 'p',
			(Player::Black, Piece::Knight) => 'n',
			(Player::Black, Piece::Bishop) => 'b',
			(Player::Black, Piece::Rook) => 'r',
			(Player::Black, Piece::Queen) => 'q',
			(Player::Black, Piece::King) => 'k',
		}
	}

	pub fn from_ascii_char(ch: char) -> (Player, Self) {
		match ch {
			'P' => (Player::White, Piece::Pawn),
			'N' => (Player::White, Piece::Knight),
			'B' => (Player::White, Piece::Bishop),
			'R' => (Player::White, Piece::Rook),
			'Q' => (Player::White, Piece::Queen),
			'K' => (Player::White, Piece::King),
			'p' => (Player::Black, Piece::Pawn),
			'n' => (Player::Black, Piece::Knight),
			'b' => (Player::Black, Piece::Bishop),
			'r' => (Player::Black, Piece::Rook),
			'q' => (Player::Black, Piece::Queen),
			'k' => (Player::Black, Piece::King),
			_ => panic!("Invalid piece character: {ch}"),
		}
	}

	pub fn emoji(self, player: Player) -> char {
		match (player, self) {
			(Player::White, Piece::Pawn) => '♙',
			(Player::White, Piece::Knight) => '♘',
			(Player::White, Piece::Bishop) => '♗',
			(Player::White, Piece::Rook) => '♖',
			(Player::White, Piece::Queen) => '♕',
			(Player::White, Piece::King) => '♔',
			(Player::Black, Piece::Pawn) => '♟',
			(Player::Black, Piece::Knight) => '♞',
			(Player::Black, Piece::Bishop) => '♝',
			(Player::Black, Piece::Rook) => '♜',
			(Player::Black, Piece::Queen) => '♛',
			(Player::Black, Piece::King) => '♚',
		}
	}
}

pub const HOME_ROW: [Piece; 8] = [
	Piece::Rook,
	Piece::Knight,
	Piece::Bishop,
	Piece::Queen,
	Piece::King,
	Piece::Bishop,
	Piece::Knight,
	Piece::Rook,
];

impl fmt::Display for Piece {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.name())
	}
}
