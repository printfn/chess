mod bitboard;
mod board;
mod consts;
mod direction;
mod file;
mod piece;
mod player;
mod pos;
mod rank;

use std::io::Write;

pub use bitboard::Bitboard;
pub use board::Board;
pub use consts::{
	ADJACENT_BITBOARDS, BISHOP_MOVE_BITBOARDS, KNIGHT_BITBOARDS, ROOK_MOVE_BITBOARDS,
};
pub use direction::{Direction, ADJACENT_DIRECTIONS, DIAGONAL_DIRECTIONS, ORTHOGONAL_DIRECTIONS};
pub use file::{File, FILES};
pub use piece::{Piece, HOME_ROW};
pub use player::Player;
pub use pos::Pos;
pub use rank::{Rank, RANKS};

fn main() {
	let mut board = Board::initial_position();
	let mut moves = vec![];
	let mut current_player = Player::White;
	loop {
		println!("{}", board);
		board.all_moves(current_player, None, &mut moves);
		println!("Count: {}", moves.len());
		for (i, m) in moves.iter().enumerate() {
			let piece = board[m.from].expect("no piece at from");
			let promotion = if let Some(p) = m.promotion {
				format!(" (promotion: {:?})", p)
			} else {
				"".to_string()
			};
			println!(
				"{i:2}: {}{}-{}{promotion}",
				piece.1.notation(),
				m.from,
				m.to
			);
		}
		let m = loop {
			print!("Enter move: ");
			std::io::stdout().flush().unwrap();
			let mut input = String::new();
			std::io::stdin().read_line(&mut input).unwrap();
			let input = input.trim();
			let input = input.parse::<usize>();
			if let Ok(input) = input {
				if input < moves.len() {
					break moves[input];
				}
			}
		};
		board[m.to] = board[m.from];
		board[m.from] = None;
		current_player = !current_player;
		moves.clear();
	}
}
