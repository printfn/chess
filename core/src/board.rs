#[cfg(test)]
mod tests;

mod repr;

use crate::*;
use core::{fmt, ops};
use repr::Repr;

#[derive(Debug, Copy, Clone)]
pub struct Board {
	pub current_player: Player,
	en_passant_target: Option<Pos>,
	pub repr: Repr,
	white_kingside_castle: bool,
	white_queenside_castle: bool,
	black_kingside_castle: bool,
	black_queenside_castle: bool,
}

impl Board {
	pub fn empty() -> Self {
		Self {
			current_player: Player::White,
			en_passant_target: None,
			repr: Repr::empty(),
			white_kingside_castle: true,
			white_queenside_castle: true,
			black_kingside_castle: true,
			black_queenside_castle: true,
		}
	}

	pub fn from_fen(fen: &str) -> Self {
		let mut result = Self::empty();
		result.white_kingside_castle = false;
		result.white_queenside_castle = false;
		result.black_kingside_castle = false;
		result.black_queenside_castle = false;
		let mut fen_iter = fen.chars();
		let mut pos: Option<Pos> = None;
		let mut next = || {
			pos = Some(if let Some(pos) = pos {
				if let Some(x) = pos.offset(Direction::E) {
					x
				} else {
					Pos::new(File::A, pos.rank().prev().expect("invalid fen"))
				}
			} else {
				Pos::new(File::A, Rank::Eight)
			});
			pos.unwrap()
		};
		for ch in fen_iter.by_ref() {
			match ch {
				' ' => break,
				'1'..='8' => {
					let n = ch as u8 - b'0';
					for _ in 0..n {
						result.setp(next(), None);
					}
				}
				'/' => (),
				_ => {
					let piece = Piece::from_ascii_char(ch);
					result.setp(next(), Some(piece));
				}
			}
		}
		result.current_player = match fen_iter.next() {
			Some('w') => Player::White,
			Some('b') => Player::Black,
			_ => panic!("invalid fen"),
		};
		assert_eq!(fen_iter.next().unwrap(), ' ');
		while let Some(ch) = fen_iter.next() {
			match ch {
				'-' => {
					assert_eq!(fen_iter.next().unwrap(), ' ');
					break;
				}
				'K' => result.white_kingside_castle = true,
				'Q' => result.white_queenside_castle = true,
				'k' => result.black_kingside_castle = true,
				'q' => result.black_queenside_castle = true,
				' ' => break,
				_ => panic!("invalid fen"),
			}
		}
		match fen_iter.next().unwrap() {
			'-' => (),
			file @ 'a'..='h' => {
				let rank = fen_iter.next().unwrap();
				result.en_passant_target = Some(Pos::new(
					File::from_value(file as u8 - b'a'),
					Rank::from_value(rank as u8 - b'1'),
				));
			}
			_ => panic!("invalid fen"),
		}
		// TODO: halfmove clock and fullmove number
		result
	}

	pub fn to_fen(&self) -> String {
		let mut result = String::new();
		for rank in (0..8).rev() {
			let mut empty = 0;
			for file in 'a'..='h' {
				let pos = Pos::new(File::from_value(file as u8 - b'a'), Rank::from_value(rank));
				if let Some((player, piece)) = self.getp(pos) {
					if empty > 0 {
						result.push((b'0' + empty) as char);
						empty = 0;
					}
					result.push(piece.ascii_char(player));
				} else {
					empty += 1;
				}
			}
			if empty > 0 {
				result.push((b'0' + empty) as char);
			}
			if rank > 0 {
				result.push('/');
			}
		}
		result.push(' ');
		result.push(match self.current_player {
			Player::White => 'w',
			Player::Black => 'b',
		});
		result.push(' ');
		if self.white_kingside_castle {
			result.push('K');
		}
		if self.white_queenside_castle {
			result.push('Q');
		}
		if self.black_kingside_castle {
			result.push('k');
		}
		if self.black_queenside_castle {
			result.push('q');
		}
		if result.ends_with(' ') {
			result.push('-');
		}
		result.push(' ');
		if let Some(pos) = self.en_passant_target {
			result.push(pos.file().into());
			result.push(pos.rank().into());
		} else {
			result.push('-');
		}
		// TODO: halfmove clock and fullmove number
		result
	}

	pub fn initial_position() -> Self {
		let mut board = Self::empty();
		for (i, piece) in HOME_ROW.iter().copied().enumerate() {
			board.repr.set(i * 8, Some((Player::White, piece)));
			board
				.repr
				.set(i * 8 + 1, Some((Player::White, Piece::Pawn)));
			board
				.repr
				.set(i * 8 + 6, Some((Player::Black, Piece::Pawn)));
			board.repr.set(i * 8 + 7, Some((Player::Black, piece)));
		}
		board
	}

	/// Returns all possible moves for the given piece, ignoring checks.
	fn simple_piece_moves(&self, pos: Pos, en_passant_target: Option<Pos>) -> Bitboard {
		let (player, piece) = self.getp(pos).expect("no piece at position");
		let iterator = match piece {
			Piece::Pawn => {
				let mut result = Bitboard::empty();
				let (direction, starting_rank, capture_dirs) = match player {
					Player::White => (Direction::N, Rank::Two, [Direction::NW, Direction::NE]),
					Player::Black => (Direction::S, Rank::Seven, [Direction::SW, Direction::SE]),
				};
				let forward_one = pos.offset(direction).expect("pawn at far edge");
				if self.getp(forward_one).is_none() {
					result.set(forward_one);
					if pos.rank() == starting_rank {
						let forward_two = forward_one.offset(direction).expect("invalid pawn");
						if self.getp(forward_two).is_none() {
							result.set(forward_two);
						}
					}
				}
				for direction in capture_dirs {
					let Some(target_pos) = pos.offset(direction) else { continue };
					let Some((target_player, _)) = self.getp(target_pos) else { continue };
					if target_player == player {
						continue;
					}
					result.set(target_pos);
				}
				if let Some(en_passant_target) = en_passant_target {
					if pos.offset(capture_dirs[0]) == Some(en_passant_target)
						|| pos.offset(capture_dirs[1]) == Some(en_passant_target)
					{
						result.set(en_passant_target);
					}
				}
				return result;
			}
			Piece::Knight => {
				return !self.repr.player_pieces(player) & pos.knight_moves();
			}
			Piece::Bishop => DIAGONAL_DIRECTIONS.iter(),
			Piece::Rook => ORTHOGONAL_DIRECTIONS.iter(),
			Piece::Queen => ADJACENT_DIRECTIONS.iter(),
			Piece::King => {
				return !self.repr.player_pieces(player) & pos.adjacent();
			}
		};
		let mut result = Bitboard::empty();
		for direction in iterator.copied() {
			let mut current = pos;
			loop {
				current = match current.offset(direction) {
					Some(current) => current,
					None => break,
				};
				if let Some((target_player, _)) = self.getp(current) {
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
		let x = match self.current_player {
			Player::White => {
				self.repr.black_knights.knight_shifts()
					| self.repr.black_pawns.black_pawn_attack_shifts()
					| self.repr.king_pos(Player::Black).adjacent()
			}
			Player::Black => {
				self.repr.white_knights.knight_shifts()
					| self.repr.white_pawns.white_pawn_attack_shifts()
					| self.repr.king_pos(Player::White).adjacent()
			}
		};
		if !(x & Bitboard::single_bit(king_pos)).is_zero() {
			return true;
		}
		for pos in (king_pos.bishop_moves()
			& self.repr.player_pieces_checks_1(!self.current_player))
			| (king_pos.rook_moves() & self.repr.player_pieces_checks_2(!self.current_player))
		{
			if self.simple_piece_moves(pos, None).get(king_pos) {
				return true;
			}
		}
		false
	}

	pub fn in_check(&self) -> bool {
		let king_pos = self.repr.king_pos(self.current_player);
		self.square_in_check(king_pos)
	}

	pub fn all_moves(&self, mut add_move: impl FnMut(Move) -> ops::ControlFlow<()>) {
		let king_pos = self.repr.king_pos(self.current_player);
		for pos in self.repr.player_pieces(self.current_player) {
			let original_piece = self.getp(pos).unwrap().1;
			let targets = self.simple_piece_moves(pos, self.en_passant_target);
			for target in targets {
				let mut new_board = *self;
				new_board.setp(pos, None);
				new_board.setp(target, Some((self.current_player, original_piece)));
				if new_board.square_in_check(if original_piece == Piece::King {
					target
				} else {
					king_pos
				}) {
					continue;
				}
				if original_piece == Piece::Pawn
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
				} else if add_move(Move {
					from: pos,
					to: target,
					promotion: None,
				})
				.is_break()
				{
					return;
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
			&& self.getp(Pos::new(File::F, rank)).is_none()
			&& self.getp(Pos::new(File::G, rank)).is_none()
			&& match self.current_player {
				Player::White => self.repr.white_rooks.get(Pos::new(File::H, Rank::One)),
				Player::Black => self.repr.black_rooks.get(Pos::new(File::H, Rank::Eight)),
			} && !self.square_in_check(Pos::new(File::E, rank))
			&& !self.square_in_check(Pos::new(File::F, rank))
			&& !self.square_in_check(Pos::new(File::G, rank))
			&& add_move(Move {
				from: Pos::new(File::E, rank),
				to: Pos::new(File::G, rank),
				promotion: None,
			})
			.is_break()
		{
			return;
		}
		if queenside_castle
			&& self.getp(Pos::new(File::D, rank)).is_none()
			&& self.getp(Pos::new(File::C, rank)).is_none()
			&& self.getp(Pos::new(File::B, rank)).is_none()
			&& match self.current_player {
				Player::White => self.repr.white_rooks.get(Pos::new(File::A, Rank::One)),
				Player::Black => self.repr.black_rooks.get(Pos::new(File::A, Rank::Eight)),
			} && !self.square_in_check(Pos::new(File::E, rank))
			&& !self.square_in_check(Pos::new(File::D, rank))
			&& !self.square_in_check(Pos::new(File::C, rank))
			&& add_move(Move {
				from: Pos::new(File::E, rank),
				to: Pos::new(File::C, rank),
				promotion: None,
			})
			.is_break()
		{}
	}

	pub fn apply_move(&mut self, mov: Move) {
		let (player, piece) = self.getp(mov.from).expect("no piece at from");
		self.setp(mov.to, None);
		if piece == Piece::King {
			match player {
				Player::White => self.repr.white_king = mov.to,
				Player::Black => self.repr.black_king = mov.to,
			}
		} else {
			let b = self.repr.mut_bitboard((player, piece));
			b.clear(mov.from);
			match mov.promotion {
				None => b.set(mov.to),
				Some(promotion) => {
					let pb = self.repr.mut_bitboard((player, promotion));
					pb.set(mov.to);
				}
			}
		}
		let back_dir = match player {
			Player::White => Direction::S,
			Player::Black => Direction::N,
		};
		if piece == Piece::Pawn && Some(mov.to) == self.en_passant_target {
			let capture_pos = mov.to.offset(back_dir).expect("invalid en passant move");
			assert!(self.getp(capture_pos) == Some((!player, Piece::Pawn)));
			self.repr.black_pawns.clear(capture_pos);
			self.repr.white_pawns.clear(capture_pos);
		}
		if piece == Piece::Pawn && mov.to.value().abs_diff(mov.from.value()) == 2 {
			self.en_passant_target = Some(mov.to.offset(back_dir).expect("invalid pawn move"));
		} else {
			self.en_passant_target = None;
		}
		if piece == Piece::King {
			if mov.from.file() == File::E && mov.to.file() == File::G {
				let rook_pos = Pos::new(File::H, mov.from.rank());
				let (player, piece) = self.getp(rook_pos).expect("no rook on h1");
				assert!(player == self.current_player && piece == Piece::Rook);
				let b = self.repr.mut_bitboard((player, Piece::Rook));
				b.clear(rook_pos);
				b.set(Pos::new(File::F, mov.from.rank()));
			} else if mov.from.file() == File::E && mov.to.file() == File::C {
				let rook_pos = Pos::new(File::A, mov.from.rank());
				let (player, piece) = self.getp(rook_pos).expect("no rook on a1");
				assert!(player == self.current_player && piece == Piece::Rook);
				let b = self.repr.mut_bitboard((player, Piece::Rook));
				b.clear(rook_pos);
				b.set(Pos::new(File::D, mov.from.rank()));
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

	pub fn get(&self, index: usize) -> Option<(Player, Piece)> {
		self.repr.get(index)
	}

	pub fn getp(&self, pos: Pos) -> Option<(Player, Piece)> {
		self.repr.get(pos.value() as usize)
	}

	pub fn set(&mut self, index: usize, piece: Option<(Player, Piece)>) {
		self.repr.set(index, piece);
	}

	pub fn setp(&mut self, pos: Pos, piece: Option<(Player, Piece)>) {
		self.repr.set(pos.value() as usize, piece);
	}

	pub fn material_difference(&self) -> i32 {
		let white_value = self.repr.white_pawns.count()
			+ self.repr.white_knights.count() * 3
			+ self.repr.white_bishops.count() * 3
			+ self.repr.white_rooks.count() * 5
			+ self.repr.white_queens.count() * 9;

		let black_value = self.repr.black_pawns.count()
			+ self.repr.black_knights.count() * 3
			+ self.repr.black_bishops.count() * 3
			+ self.repr.black_rooks.count() * 5
			+ self.repr.black_queens.count() * 9;

		white_value as i32 - black_value as i32
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "+---+---+---+---+---+---+---+---+")?;
		for rank in RANKS.iter().copied().rev() {
			write!(f, "|")?;
			for file in FILES {
				let ch = match self.getp(Pos::new(file, rank)) {
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
