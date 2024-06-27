use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Connection {
	pub src: Vec2i,
	pub dest: Vec2i,
}

impl Connection {
	#[inline]
	pub fn reverse(&self) -> Connection {
		Connection { src: self.dest, dest: self.src }
	}
}
