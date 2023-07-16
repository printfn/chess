use crate::*;
use core::{fmt, ops};

#[derive(Debug, Copy, Clone)]
pub struct Board {
	current_player: Player,
	en_passant_target: Option<Pos>,
	pieces: [Option<(Player, Piece)>; 64],
	white_kingside_castle: bool,
	white_queenside_castle: bool,
	black_kingside_castle: bool,
	black_queenside_castle: bool,
}

impl Board {
	pub fn initial_position() -> Self {
		let mut board = Self {
			current_player: Player::White,
			en_passant_target: None,
			pieces: [None; 64],
			white_kingside_castle: true,
			white_queenside_castle: true,
			black_kingside_castle: true,
			black_queenside_castle: true,
		};
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

	fn square_in_check(&self, king_pos: Pos) -> bool {
		for pos in king_pos.all_moves() {
			if let Some((p, _)) = self[pos] {
				// whether or not en passant is possible does not affect whether or not the king is in check
				if p != self.current_player && self.simple_piece_moves(pos, None).get(king_pos) {
					return true;
				}
			}
		}
		false
	}

	pub fn in_check(&self) -> bool {
		let king_pos = Pos::from_value(
			self.pieces
				.into_iter()
				.position(|piece| {
					if let Some((p, piece)) = piece {
						p == self.current_player && piece == Piece::King
					} else {
						false
					}
				})
				.expect("could not find king") as u8,
		);
		self.square_in_check(king_pos)
	}

	pub fn all_moves(&self, moves: &mut Vec<Move>) {
		for (i, piece) in self.pieces.iter().enumerate() {
			let Some((p, original_piece)) = piece else { continue; };
			if *p != self.current_player {
				continue;
			}
			let pos = Pos::from_value(i as u8);
			let targets = self.simple_piece_moves(pos, self.en_passant_target);
			for target in targets {
				let mut new_board = self.clone();
				new_board.pieces[i] = None;
				new_board.pieces[target.value() as usize] =
					Some((self.current_player, *original_piece));
				if new_board.in_check() {
					continue;
				}
				if *original_piece == Piece::Pawn
					&& (target.rank() == Rank::Eight || target.rank() == Rank::One)
				{
					for promotion in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
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
		let (kingside_castle, queenside_castle, rank) = match self.current_player {
			Player::White => (
				self.white_kingside_castle,
				self.white_queenside_castle,
				Rank::One,
			),
			Player::Black => (
				self.black_kingside_castle,
				self.black_queenside_castle,
				Rank::Eight,
			),
		};
		if kingside_castle
			&& self[Pos::new(File::F, rank)].is_none()
			&& self[Pos::new(File::G, rank)].is_none()
			&& !self.square_in_check(Pos::new(File::E, rank))
			&& !self.square_in_check(Pos::new(File::F, rank))
			&& !self.square_in_check(Pos::new(File::G, rank))
		{
			moves.push(Move {
				from: Pos::new(File::E, rank),
				to: Pos::new(File::G, rank),
				promotion: None,
			});
		}
		if queenside_castle
			&& self[Pos::new(File::D, rank)].is_none()
			&& self[Pos::new(File::C, rank)].is_none()
			&& self[Pos::new(File::B, rank)].is_none()
			&& !self.square_in_check(Pos::new(File::E, rank))
			&& !self.square_in_check(Pos::new(File::D, rank))
			&& !self.square_in_check(Pos::new(File::C, rank))
			&& !self.square_in_check(Pos::new(File::B, rank))
		{
			moves.push(Move {
				from: Pos::new(File::E, rank),
				to: Pos::new(File::G, rank),
				promotion: None,
			});
		}
	}

	pub fn apply_move(&mut self, mov: Move) {
		let (player, piece) = self[mov.from].expect("no piece at from");
		self[mov.from] = None;
		self[mov.to] = Some((player, mov.promotion.unwrap_or(piece)));
		let back_dir = match player {
			Player::White => Direction::S,
			Player::Black => Direction::N,
		};
		if piece == Piece::Pawn && Some(mov.to) == self.en_passant_target {
			let capture_pos = mov.to.offset(back_dir).expect("invalid en passant move");
			assert!(self[capture_pos] == Some((!player, Piece::Pawn)));
			self[capture_pos] = None;
		}
		if piece == Piece::Pawn && mov.to.value().abs_diff(mov.from.value()) == 2 {
			self.en_passant_target = Some(mov.to.offset(back_dir).expect("invalid pawn move"));
		} else {
			self.en_passant_target = None;
		}
		if piece == Piece::King {
			if mov.from.file() == File::E && mov.to.file() == File::G {
				let rook_pos = Pos::new(File::H, mov.from.rank());
				let (player, piece) = self[rook_pos].expect("no rook at h1");
				assert!(player == self.current_player && piece == Piece::Rook);
				self[rook_pos] = None;
				self[Pos::new(File::F, mov.from.rank())] = Some((player, piece));
			} else if mov.from.file() == File::E && mov.to.file() == File::C {
				let rook_pos = Pos::new(File::A, mov.from.rank());
				let (player, piece) = self[rook_pos].expect("no rook at a1");
				assert!(player == self.current_player && piece == Piece::Rook);
				self[rook_pos] = None;
				self[Pos::new(File::D, mov.from.rank())] = Some((player, piece));
			}
		}
		self.current_player = !self.current_player;
		if (player, piece) == (Player::White, Piece::King) {
			self.white_kingside_castle = false;
			self.white_queenside_castle = false;
		} else if (player, piece) == (Player::Black, Piece::King) {
			self.black_kingside_castle = false;
			self.black_queenside_castle = false;
		} else if (player, piece) == (Player::White, Piece::Rook) {
			if mov.from == Pos::new(File::A, Rank::One) {
				self.white_queenside_castle = false;
			} else if mov.from == Pos::new(File::H, Rank::One) {
				self.white_kingside_castle = false;
			}
		} else if (player, piece) == (Player::Black, Piece::Rook) {
			if mov.from == Pos::new(File::A, Rank::Eight) {
				self.black_queenside_castle = false;
			} else if mov.from == Pos::new(File::H, Rank::Eight) {
				self.black_kingside_castle = false;
			}
		}
	}

	pub fn game_result(&self) -> Option<GameResult> {
		let mut moves = vec![];
		self.all_moves(&mut moves);
		if moves.is_empty() {
			Some(if self.in_check() {
				GameResult::Win {
					winner: !self.current_player,
					win: WinReason::Checkmate,
				}
			} else {
				GameResult::Draw {
					draw: DrawReason::Stalemate,
				}
			})
		} else {
			None
		}
	}

	#[cfg(test)]
	fn perft(&self, depth: usize) -> usize {
		if depth == 0 {
			return 1;
		}
		let mut moves = vec![];
		self.all_moves(&mut moves);
		if depth == 1 {
			return moves.len();
		}
		let mut count = 0;
		for mov in moves {
			let mut board = self.clone();
			board.apply_move(mov);
			count += board.perft(depth - 1);
		}
		count
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
		assert_eq!(board.perft(1), 20);
		assert_eq!(board.perft(2), 400);
		assert_eq!(board.perft(3), 8902);
		assert_eq!(board.perft(4), 197_281);
		assert_eq!(board.perft(5), 4_865_609);
	}
}
