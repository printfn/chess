use core::fmt;
use std::ops;

use crate::{Board, File, Piece, Pos};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Move {
	pub from: Pos,
	pub to: Pos,
	pub promotion: Option<Piece>,
}

struct FormattedMove {
	mov: Move,
	piece: Piece,
	capture: bool,
	specify_rank: bool,
	specify_file: bool,
	kingside: bool,
	queenside: bool,
	en_passant: bool,
	check: bool,
	checkmate: bool,
}

impl Move {
	pub fn is_capture(self, board: &Board) -> bool {
		board.getp(self.to).is_some()
	}

	pub fn format(self, board: Board, all_moves: &[Move]) -> impl fmt::Display + Send + Sync {
		let (player, piece) = board.getp(self.from).expect("no piece at from");

		let (check, checkmate) = {
			let mut new_board: Board = board;
			new_board.apply_move(self);
			if new_board.in_check() {
				let mut any_moves = false;
				new_board.all_moves(|_| {
					any_moves = true;
					ops::ControlFlow::Break(())
				});
				(true, !any_moves)
			} else {
				(false, false)
			}
		};

		// find potentially ambiguous moves
		let mut specify_something = false;
		let mut specify_rank = false;
		let mut specify_file = false;
		for mov in all_moves {
			if mov.from == self.from {
				// same origin square, cannot be ambiguous
				continue;
			}
			if (player, piece) != board.getp(mov.from).expect("no piece at from") {
				// different piece (and/or different player)
				continue;
			}
			if self.to != mov.to {
				// different target square
				continue;
			}
			// we have a different move to the same square with an identical piece
			specify_something = true;
			if self.from.file() == mov.from.file() {
				specify_rank = true;
			}
			if self.from.rank() == mov.from.rank() {
				specify_file = true;
			}
		}
		if specify_something && !specify_rank && !specify_file {
			specify_file = true;
		}

		let en_passant = piece == Piece::Pawn
			&& self.from.file() != self.to.file()
			&& board.getp(self.to).is_none();
		FormattedMove {
			mov: self,
			piece,
			capture: board.getp(self.to).is_some() || en_passant,
			specify_file,
			specify_rank,
			kingside: piece == Piece::King
				&& self.from.file() == File::E
				&& self.to.file() == File::G,
			queenside: piece == Piece::King
				&& self.from.file() == File::E
				&& self.to.file() == File::C,
			en_passant,
			check,
			checkmate,
		}
	}
}

impl fmt::Display for FormattedMove {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.kingside {
			write!(f, "0-0")?;
		} else if self.queenside {
			write!(f, "0-0-0")?;
		} else {
			if self.piece != Piece::Pawn {
				write!(f, "{}", self.piece.notation())?;
				if self.specify_file {
					write!(f, "{}", self.mov.from.file())?;
				}
				if self.specify_rank {
					write!(f, "{}", self.mov.from.rank())?;
				}
			}
			if self.capture {
				if self.piece == Piece::Pawn {
					write!(f, "{}", self.mov.from.file())?;
				}
				write!(f, "x")?;
			}
			write!(f, "{}", self.mov.to)?;
			if let Some(p) = self.mov.promotion {
				write!(f, "={}", p.notation())?;
			}
		}
		if self.checkmate {
			write!(f, "#")?;
		} else if self.check {
			write!(f, "+")?;
		}
		if self.en_passant {
			write!(f, " e.p.")?;
		}
		Ok(())
	}
}
