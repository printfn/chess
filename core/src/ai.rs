use std::ops;

use crate::{Board, Move, Player};

fn evaluate(board: &Board) -> i32 {
	let white_value = board.repr.white_pawns.count()
		+ board.repr.white_knights.count() * 3
		+ board.repr.white_bishops.count() * 3
		+ board.repr.white_rooks.count() * 5
		+ board.repr.white_queens.count() * 9;

	let black_value = board.repr.black_pawns.count()
		+ board.repr.black_knights.count() * 3
		+ board.repr.black_bishops.count() * 3
		+ board.repr.black_rooks.count() * 5
		+ board.repr.black_queens.count() * 9;

	let diff = white_value as i32 - black_value as i32;

	match board.current_player {
		Player::White => diff,
		Player::Black => -diff,
	}
}

fn quiesce(board: &Board, mut alpha: i32, beta: i32) -> i32 {
	let eval = evaluate(board);
	if eval >= beta {
		return beta;
	}
	if alpha < eval {
		alpha = eval;
	}

	board.all_moves(|m| {
		if !m.is_capture(board) {
			return ops::ControlFlow::Continue(());
		}
		let mut new_board = *board;
		new_board.apply_move(m);
		let score = -quiesce(&new_board, -beta, -alpha);
		if score >= beta {
			alpha = beta;
			return ops::ControlFlow::Break(());
		}
		if score > alpha {
			alpha = score;
		}
		ops::ControlFlow::Continue(())
	});

	alpha
}

fn zw_search(board: &Board, beta: i32, depth: usize) -> i32 {
	if depth == 0 {
		return quiesce(board, beta - 1, beta);
	}

	let mut result = beta - 1;
	board.all_moves(|m| {
		let mut new_board = *board;
		new_board.apply_move(m);
		let score = -zw_search(&new_board, 1 - beta, depth - 1);
		if score >= beta {
			result = beta;
			return ops::ControlFlow::Break(());
		}
		ops::ControlFlow::Continue(())
	});
	result
}

fn pv_search(board: &Board, mut alpha: i32, beta: i32, depth: usize) -> i32 {
	if depth == 0 {
		return quiesce(board, alpha, beta);
	}

	let mut search_pv = true;
	board.all_moves(|m| {
		let mut new_board = *board;
		new_board.apply_move(m);

		let score = if search_pv {
			-pv_search(&new_board, -beta, -alpha, depth - 1)
		} else {
			let s = -zw_search(&new_board, -alpha, depth - 1);
			if s > alpha {
				-pv_search(&new_board, -beta, -alpha, depth - 1)
			} else {
				s
			}
		};

		if score >= beta {
			alpha = beta;
			return ops::ControlFlow::Break(());
		}
		if score > alpha {
			alpha = score;
			search_pv = false;
		}
		ops::ControlFlow::Continue(())
	});
	alpha
}

pub fn search(board: &Board, depth: usize) -> Option<Move> {
	let mut alpha = -10000;
	let beta = 10000;
	let mut moves = vec![];
	board.all_moves(|m| {
		moves.push(m);
		ops::ControlFlow::Continue(())
	});
	if moves.is_empty() {
		return None;
	}
	if moves.len() == 1 {
		return Some(moves[0]);
	}
	let mut r = picorand::RNG::<picorand::WyRand, u16>::new(0xdeadbeef);
	moves.sort_unstable_by_key(|_| r.generate_range(0, usize::MAX));
	let mut best_move = moves[0];
	for m in moves.into_iter().skip(1) {
		let mut new_board = *board;
		new_board.apply_move(m);
		let score = -pv_search(&new_board, -beta, -alpha, depth - 1);
		if score > alpha {
			alpha = score;
			best_move = m;
		}
	}
	Some(best_move)
}

#[cfg(test)]
mod tests {
	use crate::{search, Board};
	use std::ops;

	#[test]
	fn only_one_move() {
		let mut board =
			Board::from_fen("rnbq1bnr/1pppk1pp/p2Pp3/4P1pQ/2B1N3/8/PPP2PPP/R3K1NR b KQ -");
		let mut moves = vec![];
		board.all_moves(|m| {
			moves.push(m);
			ops::ControlFlow::Continue(())
		});
		eprintln!("{moves:?}");
		let m = search(&board, 3).unwrap();
		assert_eq!(m.format(board, moves.as_slice()).to_string(), "cxd6");
		board.apply_move(m);
	}
}
