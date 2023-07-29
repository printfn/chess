use chess_core::{search, Board, Player};
use std::ops;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn valid_moves(fen: &str) -> String {
	let board = Board::from_fen(fen);
	let mut moves = vec![];
	board.all_moves(|m| {
		moves.push(format!("[\"{}\", \"{}\"]", m.from, m.to));
		ops::ControlFlow::Continue(())
	});
	format!("[{}]", moves.join(","))
}

#[wasm_bindgen]
pub fn apply_move(fen: &str, from: &str, to: &str, promotion: Option<char>) -> String {
	let mut board = Board::from_fen(fen);
	let mut mov = None;
	board.all_moves(|m| {
		if m.from.to_string() != from
			|| m.to.to_string() != to
			|| m.promotion.map(|p| p.ascii_char(Player::White)) != promotion
		{
			return ops::ControlFlow::Continue(());
		}
		mov = Some(m);
		ops::ControlFlow::Break(())
	});
	let Some(mov) = mov else {
		return "".to_string();
	};
	board.apply_move(mov);
	board.to_fen()
}

#[wasm_bindgen]
pub fn calculate_move(fen: &str) -> String {
	let mut board = Board::from_fen(fen);
	let mov = search(&board, 3);
	let Some(mov) = mov else {
		return "".to_string();
	};
	board.apply_move(mov);
	board.to_fen()
}
