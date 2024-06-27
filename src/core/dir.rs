use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Dir {
	Up,
	Left,
	Down,
	Right,
}

impl Dir {
	#[inline]
	pub fn turn_left(self) -> Dir {
		match self {
			Dir::Up => Dir::Left,
			Dir::Left => Dir::Down,
			Dir::Down => Dir::Right,
			Dir::Right => Dir::Up,
		}
	}

	#[inline]
	pub fn turn_right(self) -> Dir {
		match self {
			Dir::Up => Dir::Right,
			Dir::Left => Dir::Up,
			Dir::Down => Dir::Left,
			Dir::Right => Dir::Down,
		}
	}

	#[inline]
	pub fn turn_around(self) -> Dir {
		match self {
			Dir::Up => Dir::Down,
			Dir::Left => Dir::Right,
			Dir::Down => Dir::Up,
			Dir::Right => Dir::Left,
		}
	}

	#[inline]
	pub fn to_vec(self) -> Vec2i {
		match self {
			Dir::Up => Vec2i(0, -1),
			Dir::Left => Vec2i(-1, 0),
			Dir::Down => Vec2i(0, 1),
			Dir::Right => Vec2i(1, 0),
		}
	}
}
