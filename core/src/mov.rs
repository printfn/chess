use core::fmt;

use crate::{Board, File, Piece, Pos};

#[derive(Debug, Copy, Clone)]
pub struct Move {
	pub from: Pos,
	pub to: Pos,
	pub promotion: Option<Piece>,
}

struct FormattedMove {
	mov: Move,
	board: Board,
}

impl Move {
	pub fn format(self, board: Board) -> impl fmt::Display + Send + Sync {
		FormattedMove { mov: self, board }
	}
}

impl fmt::Display for FormattedMove {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let (_, piece) = self.board[self.mov.from].expect("no piece at from");
		if piece == Piece::King {
			if self.mov.from.file() == File::E && self.mov.to.file() == File::G {
				return write!(f, "O-O");
			}
			if self.mov.from.file() == File::E && self.mov.to.file() == File::C {
				return write!(f, "O-O-O");
			}
		}
		let promotion = if let Some(p) = self.mov.promotion {
			format!(" (promotion: {})", p)
		} else {
			"".to_string()
		};
		write!(
			f,
			"{}{}-{}{promotion}",
			piece.notation(),
			self.mov.from,
			self.mov.to
		)?;
		Ok(())
	}
}
