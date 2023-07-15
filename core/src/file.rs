use core::fmt;

/// Represents a chess file (A to H)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum File {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
}

impl File {
	/// Returns the internal value of this file (0..8)
	pub fn value(self) -> u8 {
		match self {
			File::A => 0,
			File::B => 1,
			File::C => 2,
			File::D => 3,
			File::E => 4,
			File::F => 5,
			File::G => 6,
			File::H => 7,
		}
	}

	/// Construct a file from the given u8. If the value is not in the range 0..8, this function panics.
	pub fn from_value(value: u8) -> Self {
		match value {
			0 => File::A,
			1 => File::B,
			2 => File::C,
			3 => File::D,
			4 => File::E,
			5 => File::F,
			6 => File::G,
			7 => File::H,
			_ => panic!("{value} is not a valid file (must be in 0..8)"),
		}
	}

	pub fn next(self) -> Option<Self> {
		if self == File::H {
			None
		} else {
			Some(Self::from_value(self.value() + 1))
		}
	}

	pub fn prev(self) -> Option<Self> {
		if self == File::A {
			None
		} else {
			Some(Self::from_value(self.value() - 1))
		}
	}
}

/// An array of all files in order.
pub const FILES: [File; 8] = [
	File::A,
	File::B,
	File::C,
	File::D,
	File::E,
	File::F,
	File::G,
	File::H,
];

impl From<File> for &str {
	fn from(value: File) -> Self {
		match value {
			File::A => "a",
			File::B => "b",
			File::C => "c",
			File::D => "d",
			File::E => "e",
			File::F => "f",
			File::G => "g",
			File::H => "h",
		}
	}
}

impl TryFrom<char> for File {
	type Error = &'static str;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		Ok(match value {
			'a' | 'A' => File::A,
			'b' | 'B' => File::B,
			'c' | 'C' => File::C,
			'd' | 'D' => File::D,
			'e' | 'E' => File::E,
			'f' | 'F' => File::F,
			'g' | 'G' => File::G,
			'h' | 'H' => File::H,
			_ => return Err("invalid file"),
		})
	}
}

impl From<&File> for &str {
	fn from(value: &File) -> Self {
		Self::from(*value)
	}
}

impl fmt::Display for File {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let s: &str = self.into();
		write!(f, "{s}")
	}
}
