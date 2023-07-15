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

	/// Bitboard of all pieces belonging to the given player
	fn player_pieces(&self, player: Player) -> Bitboard {
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

	/// Returns all possible moves for the given piece, ignoring checks. Ignores castling.
	fn simple_piece_moves(&self, pos: Pos, en_passant_target: Option<Pos>) -> Bitboard {
		let (player, piece) = self[pos].expect("no piece at position");
		let iterator = match piece {
			Piece::Pawn => {
				let mut result = Bitboard::empty();
				let (direction, starting_rank, capture_dirs) = match player {
					Player::White => (Direction::N, Rank::Two, [Direction::NW, Direction::NE]),
					Player::Black => (Direction::S, Rank::Seven, [Direction::SW, Direction::SE]),
				};
				let forward_one = pos.offset(direction).expect("pawn at far edge");
				if self[forward_one].is_none() {
					result.set(forward_one);
					if pos.rank() == starting_rank {
						let forward_two = forward_one.offset(direction).expect("invalid pawn");
						if self[forward_two].is_none() {
							result.set(forward_two);
						}
					}
				}
				for direction in capture_dirs {
					let Some(target_pos) = pos.offset(direction) else { continue };
					let Some((target_player, _)) = self[target_pos] else { continue };
					if target_player == player {
						continue;
					}
					result.set(target_pos);
				}
				if let Some(en_passant_target) = en_passant_target {
					if pos.offset(capture_dirs[0]) == Some(en_passant_target) {
						result.set(en_passant_target);
					} else if pos.offset(capture_dirs[1]) == Some(en_passant_target) {
						result.set(en_passant_target);
					}
				}
				return result;
			}
			Piece::Knight => return !self.player_pieces(player) & pos.knight_moves(),
			Piece::Bishop => DIAGONAL_DIRECTIONS.iter(),
			Piece::Rook => ORTHOGONAL_DIRECTIONS.iter(),
			Piece::Queen => ADJACENT_DIRECTIONS.iter(),
			Piece::King => return !self.player_pieces(player) & pos.adjacent(),
		};
		let mut result = Bitboard::empty();
		for direction in iterator.copied() {
			let mut current = pos;
			loop {
				current = match current.offset(direction) {
					Some(current) => current,
					None => break,
				};
				if let Some((target_player, _)) = self[current] {
					if target_player == player {
						// own piece
						break;
					} else {
						// opponent piece
						result.set(current);
						break;
					}
				}
				result.set(current);
			}
		}
		result
	}

	fn in_check(&self, player: Player) -> bool {
		let king_pos = Pos::from_value(
			self.pieces
				.into_iter()
				.position(|piece| {
					if let Some((p, piece)) = piece {
						p == player && piece == Piece::King
					} else {
						false
					}
				})
				.expect("could not find king") as u8,
		);
		for pos in king_pos.all_moves() {
			if let Some((p, _)) = self[pos] {
				// whether or not en passant is possible does not affect whether or not the king is in check
				if p != player && self.simple_piece_moves(pos, None).get(king_pos) {
					return true;
				}
			}
		}
		false
	}

	pub fn all_moves(&self, player: Player, en_passant_target: Option<Pos>, moves: &mut Vec<Move>) {
		for (i, piece) in self.pieces.iter().enumerate() {
			if let Some((p, original_piece)) = piece {
				if *p == player {
					let pos = Pos::from_value(i as u8);
					let targets = self.simple_piece_moves(pos, en_passant_target);
					for target in targets {
						let mut new_board = self.clone();
						new_board.pieces[i] = None;
						new_board.pieces[target.value() as usize] = Some((player, *original_piece));
						if new_board.in_check(player) {
							continue;
						}
						if *original_piece == Piece::Pawn
							&& (target.rank() == Rank::Eight || target.rank() == Rank::One)
						{
							for promotion in
								[Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight]
							{
								moves.push(Move {
									from: pos,
									to: target,
									promotion: Some(promotion),
								});
							}
						} else {
							moves.push(Move {
								from: pos,
								to: target,
								promotion: None,
							});
						}
					}
				}
			}
		}
	}
}

impl ops::Index<Pos> for Board {
	type Output = Option<(Player, Piece)>;

	fn index(&self, pos: Pos) -> &Self::Output {
		&self.pieces[pos.value() as usize]
	}
}

impl ops::IndexMut<Pos> for Board {
	fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
		&mut self.pieces[index.value() as usize]
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
