use chess_core::Board;
use std::{io::Write, ops};

fn main() {
	let mut board = Board::initial_position();
	let mut moves = vec![];
	loop {
		println!("{}", board);
		if let Some(game_result) = board.game_result() {
			println!("Game over: {}", game_result);
			break;
		}
		board.all_moves(|m| {
			moves.push(m);
			ops::ControlFlow::Continue(())
		});
		println!("Count: {}", moves.len());
		for (i, m) in moves.iter().enumerate() {
			println!("{i:2}: {}", m.format(board, &moves));
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
		moves.clear();
	}
}
