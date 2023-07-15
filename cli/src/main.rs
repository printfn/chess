use std::io::Write;

use chess_core::{Board, Player};

fn main() {
	let mut board = Board::initial_position();
	let mut moves = vec![];
	let mut current_player = Player::White;
	loop {
		println!("{}", board);
		board.all_moves(current_player, None, &mut moves);
		println!("Count: {}", moves.len());
		for (i, m) in moves.iter().enumerate() {
			println!("{i:2}: {}", m.format(board));
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
		board.apply_move(m);
		current_player = !current_player;
		moves.clear();
	}
}
