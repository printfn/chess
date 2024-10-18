use crate::{Bitboard, File, Piece, Player, Pos, Rank};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Repr {
	pub white_pawns: Bitboard,
	pub black_pawns: Bitboard,
	pub white_knights: Bitboard,
	pub black_knights: Bitboard,
	pub white_bishops: Bitboard,
	pub black_bishops: Bitboard,
	pub white_rooks: Bitboard,
	pub black_rooks: Bitboard,
	pub white_queens: Bitboard,
	pub black_queens: Bitboard,
	pub white_king: Pos,
	pub black_king: Pos,
}

impl Repr {
	pub fn empty() -> Self {
		Self {
			white_pawns: Bitboard::empty(),
			black_pawns: Bitboard::empty(),
			white_knights: Bitboard::empty(),
			black_knights: Bitboard::empty(),
			white_bishops: Bitboard::empty(),
			black_bishops: Bitboard::empty(),
			white_rooks: Bitboard::empty(),
			black_rooks: Bitboard::empty(),
			white_queens: Bitboard::empty(),
			black_queens: Bitboard::empty(),
			white_king: Pos::new(File::E, Rank::One),
			black_king: Pos::new(File::E, Rank::Eight),
		}
	}

	pub fn get(&self, index: usize) -> Option<(Player, Piece)> {
		let pos = Pos::from_value(index as u8);
		if self.white_pawns.get(pos) {
			return Some((Player::White, Piece::Pawn));
		}
		if self.black_pawns.get(pos) {
			return Some((Player::Black, Piece::Pawn));
		}
		if self.white_knights.get(pos) {
			return Some((Player::White, Piece::Knight));
		}
		if self.black_knights.get(pos) {
			return Some((Player::Black, Piece::Knight));
		}
		if self.white_bishops.get(pos) {
			return Some((Player::White, Piece::Bishop));
		}
		if self.black_bishops.get(pos) {
			return Some((Player::Black, Piece::Bishop));
		}
		if self.white_rooks.get(pos) {
			return Some((Player::White, Piece::Rook));
		}
		if self.black_rooks.get(pos) {
			return Some((Player::Black, Piece::Rook));
		}
		if self.white_queens.get(pos) {
			return Some((Player::White, Piece::Queen));
		}
		if self.black_queens.get(pos) {
			return Some((Player::Black, Piece::Queen));
		}
		if self.white_king == pos {
			return Some((Player::White, Piece::King));
		}
		if self.black_king == pos {
			return Some((Player::Black, Piece::King));
		}
		None
	}

	pub fn set(&mut self, index: usize, piece: Option<(Player, Piece)>) {
		let pos = Pos::from_value(index as u8);
		if piece.is_some() {
			self.set(index, None);
		}
		match piece {
			None => {
				self.white_pawns.clear(pos);
				self.black_pawns.clear(pos);
				self.white_knights.clear(pos);
				self.black_knights.clear(pos);
				self.white_bishops.clear(pos);
				self.black_bishops.clear(pos);
				self.white_rooks.clear(pos);
				self.black_rooks.clear(pos);
				self.white_queens.clear(pos);
				self.black_queens.clear(pos);
			}
			Some((Player::White, Piece::Pawn)) => {
				self.white_pawns.set(pos);
			}
			Some((Player::Black, Piece::Pawn)) => {
				self.black_pawns.set(pos);
			}
			Some((Player::White, Piece::Knight)) => {
				self.white_knights.set(pos);
			}
			Some((Player::Black, Piece::Knight)) => {
				self.black_knights.set(pos);
			}
			Some((Player::White, Piece::Bishop)) => {
				self.white_bishops.set(pos);
			}
			Some((Player::Black, Piece::Bishop)) => {
				self.black_bishops.set(pos);
			}
			Some((Player::White, Piece::Rook)) => {
				self.white_rooks.set(pos);
			}
			Some((Player::Black, Piece::Rook)) => {
				self.black_rooks.set(pos);
			}
			Some((Player::White, Piece::Queen)) => {
				self.white_queens.set(pos);
			}
			Some((Player::Black, Piece::Queen)) => {
				self.black_queens.set(pos);
			}
			Some((Player::White, Piece::King)) => {
				self.white_king = pos;
			}
			Some((Player::Black, Piece::King)) => {
				self.black_king = pos;
			}
		}
	}

	/// Bitboard of all pieces belonging to the given player
	pub fn player_pieces(&self, player: Player) -> Bitboard {
		match player {
			Player::White => {
				self.white_pawns
					| self.white_knights
					| self.white_bishops
					| self.white_rooks
					| self.white_queens
					| Bitboard::single_bit(self.white_king)
			}
			Player::Black => {
				self.black_pawns
					| self.black_knights
					| self.black_bishops
					| self.black_rooks
					| self.black_queens
					| Bitboard::single_bit(self.black_king)
			}
		}
	}

	pub fn player_pieces_checks_1(&self, player: Player) -> Bitboard {
		match player {
			Player::White => self.white_bishops | self.white_queens,
			Player::Black => self.black_bishops | self.black_queens,
		}
	}

	pub fn player_pieces_checks_2(&self, player: Player) -> Bitboard {
		match player {
			Player::White => self.white_rooks | self.white_queens,
			Player::Black => self.black_rooks | self.black_queens,
		}
	}

	pub fn king_pos(&self, colour: Player) -> Pos {
		match colour {
			Player::White => self.white_king,
			Player::Black => self.black_king,
		}
	}

	pub fn mut_bitboard(&mut self, pp: (Player, Piece)) -> &mut Bitboard {
		match pp {
			(Player::White, Piece::Pawn) => &mut self.white_pawns,
			(Player::Black, Piece::Pawn) => &mut self.black_pawns,
			(Player::White, Piece::Knight) => &mut self.white_knights,
			(Player::Black, Piece::Knight) => &mut self.black_knights,
			(Player::White, Piece::Bishop) => &mut self.white_bishops,
			(Player::Black, Piece::Bishop) => &mut self.black_bishops,
			(Player::White, Piece::Rook) => &mut self.white_rooks,
			(Player::Black, Piece::Rook) => &mut self.black_rooks,
			(Player::White, Piece::Queen) => &mut self.white_queens,
			(Player::Black, Piece::Queen) => &mut self.black_queens,
			(Player::White, Piece::King) => panic!("Cannot mutate king bitboard"),
			(Player::Black, Piece::King) => panic!("Cannot mutate king bitboard"),
		}
	}
}
