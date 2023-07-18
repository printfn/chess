use crate::{Bitboard, Piece, Player, Pos};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Repr {
	pieces: [Option<(Player, Piece)>; 64],
	white_bitboard: Bitboard,
	black_bitboard: Bitboard,
}

impl Repr {
	pub fn empty() -> Self {
		Self {
			pieces: [None; 64],
			black_bitboard: Bitboard::empty(),
			white_bitboard: Bitboard::empty(),
		}
	}

	pub fn get(&self, index: usize) -> Option<(Player, Piece)> {
		self.pieces[index]
	}

	pub fn set(&mut self, index: usize, piece: Option<(Player, Piece)>) {
		self.pieces[index] = piece;
		match piece {
			None => {
				self.white_bitboard.clear(Pos::from_value(index as u8));
				self.black_bitboard.clear(Pos::from_value(index as u8));
			}
			Some((Player::White, _)) => {
				self.white_bitboard.set(Pos::from_value(index as u8));
				self.black_bitboard.clear(Pos::from_value(index as u8));
			}
			Some((Player::Black, _)) => {
				self.white_bitboard.clear(Pos::from_value(index as u8));
				self.black_bitboard.set(Pos::from_value(index as u8));
			}
		}
	}

	/// Bitboard of all pieces belonging to the given player
	pub fn player_pieces(&self, player: Player) -> Bitboard {
		match player {
			Player::White => self.white_bitboard,
			Player::Black => self.black_bitboard,
		}
	}

	pub fn king_pos(&self, colour: Player) -> Pos {
		Pos::from_value(
			self.pieces
				.into_iter()
				.position(|piece| {
					if let Some((p, piece)) = piece {
						p == colour && piece == Piece::King
					} else {
						false
					}
				})
				.expect("could not find king") as u8,
		)
	}
}
