use core::fmt;

pub struct Bitboard {
	value: u64,
}

impl Bitboard {
	/// Constructs a new bitboard using the given u64
	pub fn new(value: u64) -> Self {
		Self { value }
	}

	/// Returns an empty bitboard
	pub fn empty() -> Self {
		Self::new(0)
	}

	/// Returns a full bitboard
	pub fn full() -> Self {
		Self::new(u64::MAX)
	}

	/// Returns the bit value at the given index
	pub fn get(&self, index: u8) -> bool {
		self.value & (1 << index) != 0
	}

	/// Sets the bit at the given index
	pub fn set(&mut self, index: u8) {
		self.value |= 1 << index;
	}

	/// Unsets the bit at the given index
	pub fn clear(&mut self, index: u8) {
		self.value &= !(1 << index);
	}

	/// Returns the number of bits set in this bitboard
	pub fn count(&self) -> u8 {
		self.value.count_ones() as u8
	}
}

impl fmt::Display for Bitboard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:08x}", self.value)
	}
}

#[cfg(test)]
mod tests {
	extern crate alloc;
	use super::*;
	use alloc::format;

	#[test]
	fn display_impl() {
		assert_eq!(
			format!("{}", Bitboard::new(0x1234567890abcdef)),
			"1234567890abcdef"
		);
	}
}
