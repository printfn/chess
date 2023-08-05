use chess_core::{search, Board, Player};
use std::ops;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"
type File = 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h';
type Rank = '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8';
type Square = `${File}${Rank}`;

type GameState = {
	moves: {
		from: Square,
		to: Square,
	}[]
}

export function game_state(fen: string): GameState;
"#;

#[wasm_bindgen(skip_typescript)]
pub fn game_state(fen: &str) -> JsValue {
	let board = Board::from_fen(fen);
	let moves = js_sys::Array::new();
	board.all_moves(|m| {
		let mov = js_sys::Object::new();
		set(&mov, "from", m.from.to_string());
		set(&mov, "to", m.to.to_string());
		moves.push(&mov);
		ops::ControlFlow::Continue(())
	});
	let result = js_sys::Object::new();
	set(&result, "moves", &moves);
	result.into()
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

fn random_u32() -> u32 {
	let random_f64 = js_sys::Math::random();
	(random_f64 * f64::from(u32::MAX)) as u32
}

fn set(target: &JsValue, property_key: &str, value: impl Into<JsValue>) {
	js_sys::Reflect::set(target, &JsValue::from(property_key), &value.into()).unwrap();
}

#[wasm_bindgen]
pub fn calculate_move(fen: &str) -> String {
	let mut board = Board::from_fen(fen);
	let mov = search(&board, 3, random_u32);
	let Some(mov) = mov else {
		return "".to_string();
	};
	board.apply_move(mov);
	let fen = board.to_fen();
	format!(
		r#"{{
		"from": "{}",
		"to": "{}",
		"fen": "{fen}"
	}}"#,
		mov.from, mov.to,
	)
}

#[wasm_bindgen]
pub fn init_panic_hook() {
	console_error_panic_hook::set_once();
}
