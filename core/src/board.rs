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
			let Some((player, _)) = self[pos] else {
				continue;
			};
			if player == self.current_player {
				continue;
			}
			// whether or not en passant is possible does not affect whether or not the king is in check
			if self.simple_piece_moves(pos, None).get(king_pos) {
				return true;
			}
		}
		false
	}

	fn king_pos(&self) -> Pos {
		Pos::from_value(
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
		)
	}

	pub fn in_check(&self) -> bool {
		let king_pos = self.king_pos();
		self.square_in_check(king_pos)
	}

	pub fn all_moves(&self, mut add_move: impl FnMut(Move) -> ops::ControlFlow<()>) {
		let king_pos = self.king_pos();
		for (i, piece) in self.pieces.iter().enumerate() {
			let Some((p, original_piece)) = piece else {
				continue;
			};
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
				if new_board.square_in_check(if *original_piece == Piece::King {
					target
				} else {
					king_pos
				}) {
					continue;
				}
				if *original_piece == Piece::Pawn
					&& (target.rank() == Rank::Eight || target.rank() == Rank::One)
				{
					for promotion in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
						if add_move(Move {
							from: pos,
							to: target,
							promotion: Some(promotion),
						})
						.is_break()
						{
							return;
						}
					}
				} else {
					if add_move(Move {
						from: pos,
						to: target,
						promotion: None,
					})
					.is_break()
					{
						return;
					};
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
			if add_move(Move {
				from: Pos::new(File::E, rank),
				to: Pos::new(File::G, rank),
				promotion: None,
			})
			.is_break()
			{
				return;
			};
		}
		if queenside_castle
			&& self[Pos::new(File::D, rank)].is_none()
			&& self[Pos::new(File::C, rank)].is_none()
			&& self[Pos::new(File::B, rank)].is_none()
			&& !self.square_in_check(Pos::new(File::E, rank))
			&& !self.square_in_check(Pos::new(File::D, rank))
			&& !self.square_in_check(Pos::new(File::C, rank))
		{
			if add_move(Move {
				from: Pos::new(File::E, rank),
				to: Pos::new(File::C, rank),
				promotion: None,
			})
			.is_break()
			{
				return;
			};
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
				let (player, piece) = self[rook_pos].expect("no rook on h1");
				assert!(player == self.current_player && piece == Piece::Rook);
				self[rook_pos] = None;
				self[Pos::new(File::F, mov.from.rank())] = Some((player, piece));
			} else if mov.from.file() == File::E && mov.to.file() == File::C {
				let rook_pos = Pos::new(File::A, mov.from.rank());
				let (player, piece) = self[rook_pos].expect("no rook on a1");
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
		let mut any_moves = false;
		self.all_moves(|_| {
			any_moves = true;
			ops::ControlFlow::Break(())
		});
		if any_moves {
			None
		} else {
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
	use rayon::prelude::*;

	fn perft(board: Board, depth: usize) -> usize {
		if depth == 0 {
			return 1;
		}
		let mut moves = vec![];
		board.all_moves(|m| {
			moves.push(m);
			ops::ControlFlow::Continue(())
		});
		if depth == 1 {
			return moves.len();
		}
		let count = moves
			.par_iter()
			.map(|mov| {
				let mut board = board.clone();
				board.apply_move(*mov);
				perft(board, depth - 1)
			})
			.sum();
		count
	}

	#[track_caller]
	fn assert_move(mut board: Board, mov: &str) -> Board {
		let mut moves = vec![];
		board.all_moves(|m| {
			moves.push(m);
			ops::ControlFlow::Continue(())
		});
		for m in moves.iter() {
			if m.format(board, &moves).to_string() == mov {
				board.apply_move(*m);
				return board;
			}
		}
		panic!(
			"move not found: {} (found moves {:?})",
			mov,
			moves
				.iter()
				.map(|m| m.format(board, &moves).to_string())
				.reduce(|a, b| format!("{a}, {b}"))
				.unwrap_or("".to_string())
		);
	}

	#[track_caller]
	fn assert_moves(board: Board, moves: &[&str]) {
		let mut board = board;
		for mov in moves {
			board = assert_move(board, mov);
		}
	}

	#[test]
	fn initial_position() {
		let board = Board::initial_position();
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
		assert_eq!(perft(board, 1), 20);
		assert_eq!(perft(board, 2), 400);
		assert_eq!(perft(board, 3), 8902);
		assert_eq!(perft(board, 4), 197_281);
		assert_eq!(perft(board, 5), 4_865_609);
		assert_eq!(perft(board, 6), 119_060_324);
		assert_eq!(perft(board, 7), 3_195_901_860);
	}

	#[test]
	fn position_2() {
		let mut board = Board {
			current_player: Player::White,
			black_kingside_castle: true,
			black_queenside_castle: true,
			en_passant_target: None,
			white_kingside_castle: true,
			white_queenside_castle: true,
			pieces: [None; 64],
		};
		board.pieces[0] = Some((Player::White, Piece::Rook));
		board.pieces[1] = Some((Player::White, Piece::Pawn));
		board.pieces[5] = Some((Player::Black, Piece::Bishop));
		board.pieces[6] = Some((Player::Black, Piece::Pawn));
		board.pieces[7] = Some((Player::Black, Piece::Rook));

		board.pieces[9] = Some((Player::White, Piece::Pawn));
		board.pieces[11] = Some((Player::Black, Piece::Pawn));
		board.pieces[13] = Some((Player::Black, Piece::Knight));

		board.pieces[17] = Some((Player::White, Piece::Pawn));
		board.pieces[18] = Some((Player::White, Piece::Knight));
		board.pieces[22] = Some((Player::Black, Piece::Pawn));

		board.pieces[25] = Some((Player::White, Piece::Bishop));
		board.pieces[28] = Some((Player::White, Piece::Pawn));
		board.pieces[30] = Some((Player::Black, Piece::Pawn));

		board.pieces[32] = Some((Player::White, Piece::King));
		board.pieces[33] = Some((Player::White, Piece::Bishop));
		board.pieces[35] = Some((Player::White, Piece::Pawn));
		board.pieces[36] = Some((Player::White, Piece::Knight));
		board.pieces[37] = Some((Player::Black, Piece::Pawn));
		board.pieces[38] = Some((Player::Black, Piece::Queen));
		board.pieces[39] = Some((Player::Black, Piece::King));

		board.pieces[41] = Some((Player::White, Piece::Pawn));
		board.pieces[42] = Some((Player::White, Piece::Queen));
		board.pieces[45] = Some((Player::Black, Piece::Knight));
		board.pieces[46] = Some((Player::Black, Piece::Pawn));

		board.pieces[49] = Some((Player::White, Piece::Pawn));
		board.pieces[53] = Some((Player::Black, Piece::Pawn));
		board.pieces[54] = Some((Player::Black, Piece::Bishop));

		board.pieces[56] = Some((Player::White, Piece::Rook));
		board.pieces[57] = Some((Player::White, Piece::Pawn));
		board.pieces[58] = Some((Player::Black, Piece::Pawn));
		board.pieces[63] = Some((Player::Black, Piece::Rook));

		assert_eq!(
			board.to_string(),
			"+---+---+---+---+---+---+---+---+
| r | . | . | . | k | . | . | r |
+---+---+---+---+---+---+---+---+
| p | . | p | p | q | p | b | . |
+---+---+---+---+---+---+---+---+
| b | n | . | . | p | n | p | . |
+---+---+---+---+---+---+---+---+
| . | . | . | P | N | . | . | . |
+---+---+---+---+---+---+---+---+
| . | p | . | . | P | . | . | . |
+---+---+---+---+---+---+---+---+
| . | . | N | . | . | Q | . | p |
+---+---+---+---+---+---+---+---+
| P | P | P | B | B | P | P | P |
+---+---+---+---+---+---+---+---+
| R | . | . | . | K | . | . | R |
+---+---+---+---+---+---+---+---+\n",
			"got: \n{board}"
		);

		assert_moves(board, &["a4", "bxa3 e.p."]);
		assert_moves(board, &["Nxd7", "0-0-0"]);

		assert_eq!(perft(board, 1), 48);
		assert_eq!(perft(board, 2), 2039);
		assert_eq!(perft(board, 3), 97_862);
		assert_eq!(perft(board, 4), 4_085_603);
		assert_eq!(perft(board, 5), 193_690_690);
		assert_eq!(perft(board, 6), 8_031_647_685);
	}
}
