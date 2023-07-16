use core::fmt;

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
	board: Board,
	capture: bool,
	specify_rank: bool,
	specify_file: bool,
	kingside: bool,
	queenside: bool,
}

impl Move {
	pub fn format(self, board: Board, all_moves: &[Move]) -> impl fmt::Display + Send + Sync {
		let (player, piece) = board[self.from].expect("no piece at from");

		// find potentially ambiguous moves
		let mut specify_something = false;
		let mut specify_rank = false;
		let mut specify_file = false;
		for mov in all_moves {
			if mov.from == self.from {
				// same origin square, cannot be ambiguous
				continue;
			}
			if (player, piece) != board[mov.from].expect("no piece at from") {
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
		FormattedMove {
			mov: self,
			board,
			piece,
			capture: board[self.to].is_some(),
			specify_file,
			specify_rank,
			kingside: piece == Piece::King
				&& self.from.file() == File::E
				&& self.to.file() == File::G,
			queenside: piece == Piece::King
				&& self.from.file() == File::E
				&& self.to.file() == File::C,
		}
	}
}

impl fmt::Display for FormattedMove {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let check_char = {
			let mut new_board: Board = self.board;
			new_board.apply_move(self.mov);
			if new_board.in_check() {
				let mut moves = Vec::new();
				new_board.all_moves(None, &mut moves);
				Some(if moves.is_empty() { '#' } else { '+' })
			} else {
				None
			}
		};
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
			} else {
				if self.capture {
					write!(f, "{}", self.mov.from.file())?;
				}
			}
			if self.capture {
				write!(f, "x")?;
			}
			write!(f, "{}", self.mov.to)?;
			if let Some(p) = self.mov.promotion {
				write!(f, "={}", p.notation())?;
			}
		}
		if let Some(ch) = check_char {
			write!(f, "{}", ch)?;
		}
		// if self.piece == Piece::Pawn && self.capture {
		// 	write!(f, " e.p.")?;
		// }
		Ok(())
	}
}
