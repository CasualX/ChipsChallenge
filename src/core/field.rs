use super::*;

#[derive(Default)]
pub struct Field {
	pub name: String,
	pub hint: String,
	pub password: String,
	pub time: i32,
	pub chips: i32,
	pub width: i32,
	pub height: i32,
	pub terrain: Vec<Terrain>,
	pub conns: Vec<Connection>,
}

impl Field {
	pub fn get_terrain(&self, pos: Vec2i) -> Terrain {
		let Vec2i { x, y } = pos;
		if x < 0 || y < 0 || x >= self.width || y >= self.height {
			return Terrain::Blank;
		}
		let index = (y * self.width + x) as usize;
		self.terrain.get(index).cloned().unwrap_or(Terrain::Blank)
	}
	pub fn set_terrain(&mut self, pos: Vec2i, terrain: Terrain) {
		let Vec2i { x, y } = pos;
		if x < 0 || y < 0 || x >= self.width || y >= self.height {
			return;
		}
		let index = (y * self.width + x) as usize;
		if let Some(ptr) = self.terrain.get_mut(index) {
			*ptr = terrain;
		}
	}
	pub fn get_conn_dest(&self, pos: Vec2i) -> Option<Vec2i> {
		for conn in &self.conns {
			if conn.src == pos {
				return Some(conn.dest);
			}
		}
		return None;
	}
}
pub struct CanMoveFlags {
	pub gravel: bool,
	pub fire: bool,
	pub dirt: bool,
}
impl Field {
	pub fn can_move(&self, pos: Vec2i, dir: Dir, flags: &CanMoveFlags) -> bool {
		let cur_terrain = self.get_terrain(pos);
		let cur_solid = cur_terrain.solid_flags();

		// Allow movement if the terrain is solid
		if cur_solid == SOLID_WALL {
			return true;
		}

		// Check for panels on the current terrain
		let panel = match dir {
			Dir::Up => PANEL_N,
			Dir::Left => PANEL_W,
			Dir::Down => PANEL_S,
			Dir::Right => PANEL_E,
		};
		if cur_solid & panel != 0 {
			return false;
		}

		let next_terrain = self.get_terrain(pos + dir.to_vec());
		let next_solid = next_terrain.solid_flags();

		// Check the solid flags of the next terrain
		let panel = match dir {
			Dir::Up => PANEL_S,
			Dir::Left => PANEL_E,
			Dir::Down => PANEL_N,
			Dir::Right => PANEL_W,
		};
		if next_solid & panel != 0 {
			return false;
		}

		if !flags.gravel && matches!(next_terrain, Terrain::Gravel) {
			return false;
		}
		if !flags.fire && matches!(next_terrain, Terrain::Fire) {
			return false;
		}
		if !flags.dirt && matches!(next_terrain, Terrain::Dirt) {
			return false;
		}

		return true;
	}
}
