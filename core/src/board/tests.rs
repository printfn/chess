extern crate alloc;
use super::*;
use alloc::string::ToString;
use rayon::prelude::*;

fn single_thread_perft(board: Board, depth: usize) -> usize {
	if depth == 0 {
		return 1;
	}
	let mut moves = vec![];
	board.all_moves(|m| {
		moves.push(m);
		ops::ControlFlow::Continue(())
	});
	if depth == 1 {
		return moves.len();
	}
	let count = moves
		.iter()
		.map(|mov| {
			let mut board = board;
			board.apply_move(*mov);
			single_thread_perft(board, depth - 1)
		})
		.sum();
	count
}

fn perft(board: Board, depth: usize) -> usize {
	if depth == 0 {
		return 1;
	}
	let mut moves = vec![];
	board.all_moves(|m| {
		moves.push(m);
		ops::ControlFlow::Continue(())
	});
	if depth == 1 {
		return moves.len();
	}
	let count = moves
		.par_iter()
		.map(|mov| {
			let mut board = board;
			board.apply_move(*mov);
			perft(board, depth - 1)
		})
		.sum();
	count
}

#[track_caller]
fn assert_move(mut board: Board, mov: &str) -> Board {
	let mut moves = vec![];
	board.all_moves(|m| {
		moves.push(m);
		ops::ControlFlow::Continue(())
	});
	for m in moves.iter() {
		if m.format(board, &moves).to_string() == mov {
			board.apply_move(*m);
			return board;
		}
	}
	panic!(
		"move not found: {mov} (found moves {:?})",
		moves
			.iter()
			.map(|m| m.format(board, &moves).to_string())
			.reduce(|a, b| format!("{a}, {b}"))
			.unwrap_or("".to_string())
	);
}

#[track_caller]
fn assert_moves(board: Board, moves: &[&str]) -> Board {
	let mut board = board;
	for mov in moves {
		board = assert_move(board, mov);
	}
	board
}

#[track_caller]
fn assert_perft(board: Board, depth: usize, count: usize) {
	let actual = perft(board, depth);
	if actual == count {
		return;
	}
	// try to find the move that caused the failure
	let mut moves = vec![];
	board.all_moves(|m| {
		moves.push(m);
		ops::ControlFlow::Continue(())
	});
	for mov in moves.iter() {
		let mut new_board = board;
		new_board.apply_move(*mov);
		let subcount = perft(new_board, depth - 1);
		eprintln!("{}{}: {}", mov.from, mov.to, subcount);
	}
	assert_eq!(actual, count, "perft failed at depth {depth}");
}

#[test]
fn initial_position() {
	let board = Board::initial_position();
	assert_eq!(
		board.getp(Pos::try_from("a1").unwrap()),
		Some((Player::White, Piece::Rook))
	);
	assert_eq!(
		board.getp(Pos::try_from("a8").unwrap()),
		Some((Player::Black, Piece::Rook))
	);
	assert_eq!(
		board.getp(Pos::try_from("e1").unwrap()),
		Some((Player::White, Piece::King))
	);
	assert_eq!(
		board.getp(Pos::try_from("h1").unwrap()),
		Some((Player::White, Piece::Rook))
	);
	assert_eq!(
		board.getp(Pos::try_from("h8").unwrap()),
		Some((Player::Black, Piece::Rook))
	);
	let actual = board.to_string();
	assert_eq!(
		actual,
		"+---+---+---+---+---+---+---+---+
| r | n | b | q | k | b | n | r |
+---+---+---+---+---+---+---+---+
| p | p | p | p | p | p | p | p |
+---+---+---+---+---+---+---+---+
| . | . | . | . | . | . | . | . |
+---+---+---+---+---+---+---+---+
| . | . | . | . | . | . | . | . |
+---+---+---+---+---+---+---+---+
| . | . | . | . | . | . | . | . |
+---+---+---+---+---+---+---+---+
| . | . | . | . | . | . | . | . |
+---+---+---+---+---+---+---+---+
| P | P | P | P | P | P | P | P |
+---+---+---+---+---+---+---+---+
| R | N | B | Q | K | B | N | R |
+---+---+---+---+---+---+---+---+\n",
		"got: \n{actual}"
	);
	assert_perft(board, 1, 20);
	assert_perft(board, 2, 400);
	assert_perft(board, 3, 8902);
	assert_perft(board, 4, 197_281);
	assert_perft(board, 5, 4_865_609);
	assert_perft(board, 6, 119_060_324);
	assert_perft(board, 7, 3_195_901_860);
}

#[test]
fn position_2() {
	let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
	let board = Board::from_fen(fen);
	assert_eq!(board.to_fen(), fen);

	assert_eq!(
		board.to_string(),
		"+---+---+---+---+---+---+---+---+
| r | . | . | . | k | . | . | r |
+---+---+---+---+---+---+---+---+
| p | . | p | p | q | p | b | . |
+---+---+---+---+---+---+---+---+
| b | n | . | . | p | n | p | . |
+---+---+---+---+---+---+---+---+
| . | . | . | P | N | . | . | . |
+---+---+---+---+---+---+---+---+
| . | p | . | . | P | . | . | . |
+---+---+---+---+---+---+---+---+
| . | . | N | . | . | Q | . | p |
+---+---+---+---+---+---+---+---+
| P | P | P | B | B | P | P | P |
+---+---+---+---+---+---+---+---+
| R | . | . | . | K | . | . | R |
+---+---+---+---+---+---+---+---+\n",
		"got: \n{board}"
	);

	assert_moves(board, &["a4", "bxa3 e.p."]);
	assert_moves(board, &["Nxd7", "0-0-0"]);
	assert_moves(board, &["d6", "Bb5", "Rb1", "Kf8"]);
	assert_moves(board, &["d6", "Bb5", "Rb1", "Kd8"]);

	assert_perft(assert_moves(board, &["d6", "Bb5", "dxe7"]), 1, 38);
	assert_perft(assert_moves(board, &["d6", "Bb5", "Rb1"]), 1, 43);
	assert_perft(assert_moves(board, &["d6", "Bb5"]), 2, 2035);
	assert_perft(assert_moves(board, &["d6"]), 3, 79551);

	assert_perft(assert_moves(board, &["Nxf7", "Bb5", "Nxh8"]), 1, 41);
	assert_perft(assert_moves(board, &["Nxf7", "Bb5"]), 2, 2084);
	assert_perft(assert_moves(board, &["Nxf7"]), 3, 88799);

	assert_perft(board, 1, 48);
	assert_perft(board, 2, 2039);
	assert_perft(board, 3, 97_862);
	assert_perft(board, 4, 4_085_603);
	assert_perft(board, 5, 193_690_690);
	assert_perft(board, 6, 8_031_647_685);
}

#[test]
fn benchmark() {
	single_thread_perft(Board::initial_position(), 6);
}

#[test]
fn fen_test() {
	let fen = "rnbq3r/ppppkppp/5n2/2b1p3/2B1P3/5N2/PPPPKPPP/RNBQ3R w - -";
	let b = Board::from_fen(fen);
	assert_eq!(b.to_fen(), fen)
}
