use core::fmt;

use crate::Player;

#[derive(Debug)]
pub enum GameResult {
	Win { winner: Player, win: WinReason },
	Draw { draw: DrawReason },
}

impl fmt::Display for GameResult {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			GameResult::Win { winner, win } => write!(f, "{winner} wins by {win}"),
			GameResult::Draw { draw } => write!(f, "{draw}"),
		}
	}
}

#[derive(Debug)]
pub enum DrawReason {
	Stalemate,
	InsufficientMaterial,
	ThreefoldRepetition,
	FiftyMoveRule,
	Agreement,
}

impl fmt::Display for DrawReason {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = match self {
			DrawReason::Stalemate => "stalemate",
			DrawReason::InsufficientMaterial => "draw by insufficient material",
			DrawReason::ThreefoldRepetition => "draw by threefold repetition",
			DrawReason::FiftyMoveRule => "draw by fifty move rule",
			DrawReason::Agreement => "draw by agreement",
		};
		write!(f, "{s}")
	}
}

#[derive(Debug)]
pub enum WinReason {
	Checkmate,
	Resignation,
}

impl fmt::Display for WinReason {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = match self {
			WinReason::Checkmate => "checkmate",
			WinReason::Resignation => "resignation",
		};
		write!(f, "{s}")
	}
}
