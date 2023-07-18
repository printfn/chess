use crate::{Bitboard, Piece, Player, Pos};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Repr {
	pieces: [Option<(Player, Piece)>; 64],
}

impl Repr {
	pub fn empty() -> Self {
		Self { pieces: [None; 64] }
	}

	pub fn get(&self, index: usize) -> Option<(Player, Piece)> {
		self.pieces[index]
	}

	pub fn set(&mut self, index: usize, piece: Option<(Player, Piece)>) {
		self.pieces[index] = piece;
	}

	/// Bitboard of all pieces belonging to the given player
	pub fn player_pieces(&self, player: Player) -> Bitboard {
		let mut result = Bitboard::empty();
		for (i, piece) in self.pieces.iter().enumerate() {
			if let Some((p, _)) = piece {
				if *p == player {
					result.set(Pos::from_value(i as u8));
				}
			}
		}
		result
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

	pub fn enumerate_pieces(&self) -> impl Iterator<Item = Option<(Player, Piece)>> + '_ {
		self.pieces.iter().copied()
	}
}
