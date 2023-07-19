use core::{fmt, ops};

use crate::Pos;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Bitboard {
	value: u64,
}

const NOT_TOP_ROW: u64 = 0x7f7f_7f7f_7f7f_7f7f;
const NOT_BOTTOM_ROW: u64 = 0xfefe_fefe_fefe_fefe;

impl Bitboard {
	/// Constructs a new bitboard using the given u64
	pub fn new(value: u64) -> Self {
		Self { value }
	}

	pub fn single_bit(pos: Pos) -> Self {
		Self::new(1 << pos.value())
	}

	/// Returns an empty bitboard
	pub fn empty() -> Self {
		Self::new(0)
	}

	/// Returns a full bitboard
	pub fn full() -> Self {
		Self::new(u64::MAX)
	}

	/// Returns the bit value at the given position
	pub fn get(&self, pos: Pos) -> bool {
		self.value & (1 << pos.value()) != 0
	}

	/// Sets the bit at the given position
	pub fn set(&mut self, pos: Pos) {
		self.value |= 1 << pos.value();
	}

	/// Unsets the bit at the given pos
	pub fn clear(&mut self, pos: Pos) {
		self.value &= !(1 << pos.value());
	}

	/// Returns the number of bits set in this bitboard
	pub fn count(&self) -> u8 {
		self.value.count_ones() as u8
	}

	pub fn ilog2(&self) -> u8 {
		self.value.ilog2() as u8
	}

	pub fn shift_left(&self) -> Self {
		Self::new(self.value << 8)
	}

	pub fn shift_right(&self) -> Self {
		Self::new(self.value >> 8)
	}

	pub fn shift_up(&self) -> Self {
		Self::new((self.value & NOT_TOP_ROW) << 1)
	}

	pub fn shift_down(&self) -> Self {
		Self::new((self.value & NOT_BOTTOM_ROW) >> 1)
	}

	pub fn knight_shifts(&self) -> Self {
		let d1 = (self.value >> 1) & 0x7f7f7f7f7f7f7f7f;
		let d2 = (self.value >> 2) & 0x3f3f3f3f3f3f3f3f;
		let u1 = (self.value << 1) & 0xfefefefefefefefe;
		let u2 = (self.value << 2) & 0xfcfcfcfcfcfcfcfc;
		let a = d1 | u1;
		let b = d2 | u2;
		Self::new(a << 16 | a >> 16 | b << 8 | b >> 8)
	}

	pub fn white_pawn_attack_shifts(&self) -> Self {
		self.shift_up().shift_right() | self.shift_up().shift_left()
	}

	pub fn black_pawn_attack_shifts(&self) -> Self {
		self.shift_down().shift_right() | self.shift_down().shift_left()
	}

	pub fn is_zero(&self) -> bool {
		self.value == 0
	}
}

impl fmt::Debug for Bitboard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:016x}", self.value)
	}
}

impl ops::BitOr for Bitboard {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self::Output {
		Self::new(self.value | rhs.value)
	}
}

impl ops::BitAnd for Bitboard {
	type Output = Self;

	fn bitand(self, rhs: Self) -> Self::Output {
		Self::new(self.value & rhs.value)
	}
}

impl ops::Not for Bitboard {
	type Output = Self;

	fn not(self) -> Self::Output {
		Self::new(!self.value)
	}
}

impl fmt::Display for Bitboard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:016x}", self.value)
	}
}

impl IntoIterator for Bitboard {
	type Item = Pos;
	type IntoIter = BitboardIterator;

	fn into_iter(self) -> Self::IntoIter {
		let leading_zeroes = self.value.leading_zeros();
		let trailing_zeroes = self.value.trailing_zeros();
		BitboardIterator {
			bitboard: self,
			index: trailing_zeroes as u8,
			end: (64 - leading_zeroes) as u8,
		}
	}
}

pub struct BitboardIterator {
	bitboard: Bitboard,
	index: u8,
	end: u8,
}

impl Iterator for BitboardIterator {
	type Item = Pos;

	fn next(&mut self) -> Option<Self::Item> {
		while self.index < self.end {
			let index = self.index;
			self.index += 1;
			if self.bitboard.get(Pos::from_value(index)) {
				return Some(Pos::from_value(index));
			}
		}
		None
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

	#[test]
	fn iterator() {
		assert_eq!(
			Bitboard::new(1).into_iter().collect::<Vec<_>>(),
			vec![Pos::from_value(0)]
		);
	}
}
