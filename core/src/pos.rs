use crate::*;
use core::fmt;

/// Represents a chess position (A1 to H8)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
	pos: u8,
}

impl Pos {
	/// Constructs a new position from a file and a rank
	pub fn new(file: File, rank: Rank) -> Self {
		let r: u8 = rank.value();
		let f: u8 = file.value();
		Self { pos: f << 3 | r }
	}

	/// Returns this position's file
	pub fn file(self) -> File {
		debug_assert!(self.pos < 64);
		File::from_value(self.pos >> 3)
	}

	/// Returns this position's rank
	pub fn rank(self) -> Rank {
		debug_assert!(self.pos < 64);
		Rank::from_value(self.pos & 7)
	}

	/// Returns this position's value (0..64)
	pub fn value(self) -> u8 {
		self.pos
	}

	/// Constructs a new position from the given value. Panics if the value is not in 0..64.
	pub fn from_value(value: u8) -> Self {
		debug_assert!(value < 64);
		Self { pos: value }
	}

	pub fn adjacent(self) -> Bitboard {
		Bitboard::new(ADJACENT_BITBOARDS[self.value() as usize])
	}

	pub fn knight_moves(self) -> Bitboard {
		Bitboard::new(KNIGHT_BITBOARDS[self.value() as usize])
	}

	pub fn bishop_moves(self) -> Bitboard {
		Bitboard::new(BISHOP_MOVE_BITBOARDS[self.value() as usize])
	}

	pub fn rook_moves(self) -> Bitboard {
		Bitboard::new(ROOK_MOVE_BITBOARDS[self.value() as usize])
	}

	pub fn queen_moves(self) -> Bitboard {
		self.bishop_moves() | self.rook_moves()
	}

	pub fn all_moves(self) -> Bitboard {
		self.knight_moves() | self.bishop_moves() | self.rook_moves()
	}

	pub fn white_pawn_checks(self) -> Bitboard {
		Bitboard::new(WHITE_PAWN_CHECK_BITBOARDS[self.value() as usize])
	}

	pub fn black_pawn_checks(self) -> Bitboard {
		Bitboard::new(BLACK_PAWN_CHECK_BITBOARDS[self.value() as usize])
	}

	pub fn offset(self, direction: Direction) -> Option<Self> {
		Some(match direction {
			Direction::N => Self::new(self.file(), self.rank().next()?),
			Direction::NE => Self::new(self.file().next()?, self.rank().next()?),
			Direction::E => Self::new(self.file().next()?, self.rank()),
			Direction::SE => Self::new(self.file().next()?, self.rank().prev()?),
			Direction::S => Self::new(self.file(), self.rank().prev()?),
			Direction::SW => Self::new(self.file().prev()?, self.rank().prev()?),
			Direction::W => Self::new(self.file().prev()?, self.rank()),
			Direction::NW => Self::new(self.file().prev()?, self.rank().next()?),
		})
	}
}

impl TryFrom<&str> for Pos {
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		if value.len() != 2 {
			return Err("expected a string of length 2");
		}
		let mut iterator = value.chars();
		let file = File::try_from(iterator.next().unwrap())?;
		let rank = Rank::try_from(iterator.next().unwrap())?;
		Ok(Pos::new(file, rank))
	}
}

impl fmt::Display for Pos {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}{}", self.file(), self.rank())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ranks_and_files() {
		let mut i = 0;
		for file in FILES {
			for rank in RANKS {
				let pos = Pos::new(file, rank);
				assert_eq!(i, pos.pos);
				assert_eq!(pos.rank(), rank);
				assert_eq!(pos.file(), file);
				i += 1;
			}
		}
	}
}
