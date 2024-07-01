use super::*;

pub struct QuadTile {
	pub start: u16,
	pub len: u16,
}

pub struct QuadTree {
	pub ents: Vec<EntityHandle>,
	pub tiles: Vec<QuadTile>,
}
