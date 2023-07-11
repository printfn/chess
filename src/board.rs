use crate::*;
use core::{fmt, ops};

#[derive(Debug, Copy, Clone)]
pub struct Board {
	pieces: [Option<(Player, Piece)>; 64],
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
	pub from: Pos,
	pub to: Pos,
	pub promotion: Option<Piece>,
}

impl Board {
	pub fn empty() -> Self {
		Self { pieces: [None; 64] }
	}

	pub fn initial_position() -> Self {
		let mut board = Self { pieces: [None; 64] };
		for (i, piece) in HOME_ROW.iter().copied().enumerate() {
			board.pieces[i * 8] = Some((Player::White, piece));
			board.pieces[i * 8 + 1] = Some((Player::White, Piece::Pawn));
			board.pieces[i * 8 + 6] = Some((Player::Black, Piece::Pawn));
			board.pieces[i * 8 + 7] = Some((Player::Black, piece));
		}
		board
	}

	fn player_pieces(&self, player: Player) -> Bitboard {
		let mut result = Bitboard::empty();
		for (i, piece) in self.pieces.iter().enumerate() {
			if let Some((p, _)) = piece {
				if *p == player {
					result.set(i as u8);
				}
			}
		}
		result
	}

	fn simple_piece_moves(&self, pos: Pos) -> Bitboard {
		let (player, piece) = self[pos].expect("no piece at position");
		match piece {
			Piece::Pawn => todo!(),
			Piece::Knight => !self.player_pieces(player) & pos.knight_moves(),
			Piece::Bishop => todo!(),
			Piece::Rook => todo!(),
			Piece::Queen => todo!(),
			Piece::King => !self.player_pieces(player) & pos.adjacent(),
		}
	}
}

impl ops::Index<Pos> for Board {
	type Output = Option<(Player, Piece)>;

	fn index(&self, pos: Pos) -> &Self::Output {
		&self.pieces[pos.value() as usize]
	}
}

impl ops::Index<&str> for Board {
	type Output = Option<(Player, Piece)>;

	fn index(&self, pos: &str) -> &Self::Output {
		&self[Pos::try_from(pos).expect("failed to parse position")]
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "+---+---+---+---+---+---+---+---+\n")?;
		for rank in RANKS.iter().copied().rev() {
			write!(f, "|")?;
			for file in FILES {
				let ch = match self[Pos::new(file, rank)] {
					None => '.',
					Some((player, piece)) => piece.ascii_char(player),
				};
				write!(f, " {ch} |")?;
			}
			write!(f, "\n+---+---+---+---+---+---+---+---+\n")?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	extern crate alloc;
	use super::*;
	use alloc::string::ToString;

	#[test]
	fn initial_position() {
		let board = &Board::initial_position();
		assert_eq!(board["a1"], Some((Player::White, Piece::Rook)));
		assert_eq!(board["a8"], Some((Player::Black, Piece::Rook)));
		assert_eq!(board["e1"], Some((Player::White, Piece::King)));
		assert_eq!(board["h1"], Some((Player::White, Piece::Rook)));
		assert_eq!(board["h8"], Some((Player::Black, Piece::Rook)));
		let actual = board.to_string();
		assert_eq!(
			actual,
			"+---+---+---+---+---+---+---+---+
| r | n | b | q | k | b | n | r |
+---+---+---+---+---+---+---+---+
| p | p | p | p | p | p | p | p |
+---+---+---+---+---+---+---+---+
| . | . | . | . | . | . | . | . |
+---+---+---+---+---+---+---+---+
| . | . | . | . | . | . | . | . |
+---+---+---+---+---+---+---+---+
| . | . | . | . | . | . | . | . |
+---+---+---+---+---+---+---+---+
| . | . | . | . | . | . | . | . |
+---+---+---+---+---+---+---+---+
| P | P | P | P | P | P | P | P |
+---+---+---+---+---+---+---+---+
| R | N | B | Q | K | B | N | R |
+---+---+---+---+---+---+---+---+\n",
			"got: \n{actual}"
		);
	}
}
