use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LessonDto {
	pub hint: String,
	pub password: String,
	pub time: i32,
	pub chips: i32,
	pub width: i32,
	pub height: i32,
	pub map: Vec<String>,
	pub start: [i32; 2],
	pub tiles: std::collections::HashMap<char, TileProps>,
}

impl Game {
	pub fn load_level(&mut self, json: &str) {
		let ld: level::LessonDto = serde_json::from_str(json).unwrap();
		assert_eq!(ld.map.len(), ld.height as usize);
		self.field.time_limit = ld.time;
		self.field.chips = 0;
		self.field.width = ld.width;
		self.field.height = ld.height;
		self.field.map.clear();
		self.field.tiles.clear();
		self.objects.map.clear();

		let mut map = std::collections::HashMap::new();
		for (chr, tile) in ld.tiles {
			let index = self.field.tiles.len();
			self.field.tiles.push(tile);
			map.insert(chr, index);
		}

		self.pl.object = self.objects.alloc();
		self.pl.entity = self.entities.alloc();

		#[derive(Default)]
		struct Count {
			chips: i32,
			keys: [i32; 4],
			doors: [i32; 4],
		}
		let mut n = Count::default();

		for (y, line) in ld.map.iter().enumerate() {
			assert_eq!(line.len(), ld.width as usize);
			for (x, chr) in line.bytes().enumerate() {
				let x = x as i32;
				let y = y as i32;
				let tile = match chr {
					b' ' => ' ',
					b'.' => '.',
					b'#' => '#',
					b'~' => '~',
					b'\'' => '\'',
					b'@' => {
						n.chips += 1;
						entities::chip::create(self, x, y);
						'.'
					},
					b'X' => 'X',
					b'=' => {
						entities::gate::create(self, x, y);
						'.'
					},
					b'i' => 'i',
					b'+' => {
						entities::block::create(self, x, y);
						'.'
					},
					b'b' => {
						n.keys[0] += 1;
						entities::key::create(self, x, y, KeyColor::Blue);
						'.'
					},
					b'r' => {
						n.keys[1] += 1;
						entities::key::create(self, x, y, KeyColor::Red);
						'.'
					},
					b'g' => {
						n.keys[2] += 1;
						entities::key::create(self, x, y, KeyColor::Green);
						'.'
					},
					b'y' => {
						n.keys[3] += 1;
						entities::key::create(self, x, y, KeyColor::Yellow);
						'.'
					},
					b'B' => {
						n.doors[0] += 1;
						entities::door::create(self, x, y, KeyColor::Blue);
						'.'
					},
					b'R' => {
						n.doors[1] += 1;
						entities::door::create(self, x, y, KeyColor::Red);
						'.'
					},
					b'G' => {
						n.doors[2] += 1;
						entities::door::create(self, x, y, KeyColor::Green);
						'.'
					},
					b'Y' => {
						n.doors[3] += 1;
						entities::door::create(self, x, y, KeyColor::Yellow);
						'.'
					},
					b'1' => {
						entities::bug::create(self, x, y);
						'.'
					},
					_ => unimplemented!("Unknown tile: {}", chr as char),
				};
				let index = map[&tile];
				self.field.map.push(index as u8);
			}
		}
		// assert_eq!(n.keys, n.doors);
		self.field.chips = n.chips;
		self.cam.eye = Vec2(ld.start[0], ld.start[1]).map(|c| c as f32).vec3(0.0) * 32.0;
		self.cam.offset = Vec3(0.0, 8.0 * 32.0, 400.0);
		entities::player::create(self, ld.start[0], ld.start[1]);
	}
}
