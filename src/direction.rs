#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
	N,
	NE,
	E,
	SE,
	S,
	SW,
	W,
	NW,
}

pub const ORTHOGONAL_DIRECTIONS: [Direction; 4] =
	[Direction::N, Direction::E, Direction::S, Direction::W];

pub const DIAGONAL_DIRECTIONS: [Direction; 4] =
	[Direction::NE, Direction::SE, Direction::SW, Direction::NW];

pub const ADJACENT_DIRECTIONS: [Direction; 8] = [
	Direction::N,
	Direction::NE,
	Direction::E,
	Direction::SE,
	Direction::S,
	Direction::SW,
	Direction::W,
	Direction::NW,
];
