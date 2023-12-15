use chess_core::{search, Board};
use std::{io::Write, ops};

fn random_u32() -> u32 {
	rand::random()
}

fn main() {
	let mut board = Board::initial_position();
	let mut moves = vec![];
	let mut input = String::new();
	loop {
		println!("{board}");
		if let Some(game_result) = board.game_result() {
			println!("Game over: {game_result}");
			break;
		}
		board.all_moves(|m| {
			moves.push(m);
			ops::ControlFlow::Continue(())
		});
		println!("Count: {}", moves.len());
		for (i, m) in moves.iter().enumerate() {
			println!("{:2}: {}", i + 1, m.format(board, &moves));
		}
		let m = loop {
			print!("Enter move: ");
			std::io::stdout().flush().unwrap();
			input.clear();
			std::io::stdin().read_line(&mut input).unwrap();
			let input = input.trim();
			let input = input.parse::<usize>();
			if let Ok(input) = input {
				if input == 0 {
					break search(&board, 5, true, random_u32).expect("Expected to find a move");
				} else if input <= moves.len() {
					break moves[input - 1];
				}
			}
		};
		board.apply_move(m);
		moves.clear();
	}
}
