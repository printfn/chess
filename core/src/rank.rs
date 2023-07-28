use core::fmt;

/// Represents a chess rank (1 to 8)
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Ord, PartialOrd)]
pub enum Rank {
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
}

/// An array of all ranks in order.
pub const RANKS: [Rank; 8] = [
	Rank::One,
	Rank::Two,
	Rank::Three,
	Rank::Four,
	Rank::Five,
	Rank::Six,
	Rank::Seven,
	Rank::Eight,
];

impl Rank {
	/// Convert the rank to a u8 in the range 0..8
	pub fn value(self) -> u8 {
		match self {
			Rank::One => 0,
			Rank::Two => 1,
			Rank::Three => 2,
			Rank::Four => 3,
			Rank::Five => 4,
			Rank::Six => 5,
			Rank::Seven => 6,
			Rank::Eight => 7,
		}
	}

	/// Construct a rank from the given u8.
	/// If the value is not in the range 0..8, this function panics.
	pub fn from_value(value: u8) -> Self {
		match value {
			0 => Rank::One,
			1 => Rank::Two,
			2 => Rank::Three,
			3 => Rank::Four,
			4 => Rank::Five,
			5 => Rank::Six,
			6 => Rank::Seven,
			7 => Rank::Eight,
			_ => panic!("{value} is not a valid rank (must be in 0..8)"),
		}
	}

	pub fn next(self) -> Option<Self> {
		if self == Rank::Eight {
			None
		} else {
			Some(Self::from_value(self.value() + 1))
		}
	}

	pub fn prev(self) -> Option<Self> {
		if self == Rank::One {
			None
		} else {
			Some(Self::from_value(self.value() - 1))
		}
	}
}

impl fmt::Display for Rank {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", self.value() + 1)
	}
}

impl From<Rank> for char {
	fn from(rank: Rank) -> Self {
		char::from(rank.value() + b'1')
	}
}

impl From<Rank> for &str {
	fn from(value: Rank) -> Self {
		match value {
			Rank::One => "1",
			Rank::Two => "2",
			Rank::Three => "3",
			Rank::Four => "4",
			Rank::Five => "5",
			Rank::Six => "6",
			Rank::Seven => "7",
			Rank::Eight => "8",
		}
	}
}

impl TryFrom<char> for Rank {
	type Error = &'static str;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		Ok(match value {
			'1' => Rank::One,
			'2' => Rank::Two,
			'3' => Rank::Three,
			'4' => Rank::Four,
			'5' => Rank::Five,
			'6' => Rank::Six,
			'7' => Rank::Seven,
			'8' => Rank::Eight,
			_ => return Err("invalid rank"),
		})
	}
}

#[cfg(test)]
mod tests {
	extern crate alloc;
	use super::*;
	use alloc::string::ToString;

	#[test]
	fn to_string() {
		for rank in RANKS {
			let a = char::from(rank).to_string();
			let b = <&str>::from(rank);
			let c = rank.to_string();
			assert_eq!(a, b);
			assert_eq!(a, c);
		}
	}
}
