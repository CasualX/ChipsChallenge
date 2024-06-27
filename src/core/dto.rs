use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MapDto {
	pub width: i32,
	pub height: i32,
	pub data: Vec<u8>,
	pub legend: Vec<Terrain>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LevelDto {
	pub name: String,
	pub hint: String,
	pub password: String,
	pub time: i32,
	pub chips: i32,
	pub map: MapDto,
	pub entities: Vec<SpawnData>,
	pub connections: Vec<Connection>,
}

impl GameState {
	pub fn load(&mut self, json: &str) {
		let ld: dto::LevelDto = serde_json::from_str(json).unwrap();
		self.field.name = ld.name;
		self.field.hint = ld.hint;
		self.field.password = ld.password;
		self.field.time = ld.time;
		self.field.chips = ld.chips;
		self.field.width = ld.map.width;
		self.field.height = ld.map.height;
		self.field.terrain.clear();
		self.field.conns = ld.connections;

		for data in &ld.entities {
			entities::create(self, data);
		}

		assert!(ld.map.width > 0, "Invalid map width");
		assert!(ld.map.height > 0, "Invalid map height");
		let size = ld.map.width as usize * ld.map.height as usize;

		if ld.map.data.is_empty() {
			for _ in 0..size {
				self.field.terrain.push(Terrain::Floor);
			}
		}
		else {
			assert_eq!(ld.map.data.len(), size, "Invalid map data length");
			for y in 0..ld.map.height {
				for x in 0..ld.map.width {
					let index = (y * ld.map.width + x) as usize;
					let terrain = ld.map.legend[ld.map.data[index] as usize];
					self.field.terrain.push(terrain);
				}
			}
		}
	}
}
