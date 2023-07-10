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
