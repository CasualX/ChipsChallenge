use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EntityDto {
	pub kind: EntityKind,
	pub pos: [i32; 2],
	pub face_dir: Option<Dir>,
	pub spawner_kind: Option<EntityKind>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LessonDto {
	pub hint: String,
	pub password: String,
	pub time: i32,
	pub chips: i32,
	pub width: i32,
	pub height: i32,
	pub map: Vec<String>,
	pub entities: Vec<EntityDto>,
}

impl Game {
	pub fn load_level(&mut self, json: &str) {
		let ld: dto::LessonDto = serde_json::from_str(json).unwrap();
		assert_eq!(ld.map.len(), ld.height as usize);
		self.field.time_limit = ld.time;
		self.field.chips = ld.chips;
		self.field.width = ld.width;
		self.field.height = ld.height;
		self.field.map.clear();
		self.field.tiles.clear();
		self.objects.map.clear();

		let mut map = std::collections::HashMap::new();

		map.insert(' ', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::Wall,
			sprite: Sprite::Blank,
			model: Model::Empty,
			solid: SOLID_WALL,
		});
		map.insert('.', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::Floor,
			sprite: Sprite::Floor,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('#', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::Wall,
			sprite: Sprite::Wall,
			model: Model::Wall,
			solid: SOLID_WALL,
		});
		map.insert('X', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::Exit,
			sprite: Sprite::Exit1,
			model: Model::Portal,
			solid: 0,
		});
		map.insert('i', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::Hint,
			sprite: Sprite::Hint,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('~', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::Water,
			sprite: Sprite::Water,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('\'', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::Dirt,
			sprite: Sprite::Dirt,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('s', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::Ice,
			sprite: Sprite::Ice,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('t', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::IceUL,
			sprite: Sprite::IceUL,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('w', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::IceDL,
			sprite: Sprite::IceDL,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('^', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::ForceUp,
			sprite: Sprite::ForceUp,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('<', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::ForceLeft,
			sprite: Sprite::ForceLeft,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('v', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::ForceDown,
			sprite: Sprite::ForceDown,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('>', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::ForceRight,
			sprite: Sprite::ForceRight,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('p', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::GreenSwitch,
			sprite: Sprite::GreenSwitch,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('m', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::RedSwitch,
			sprite: Sprite::RedSwitch,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('q', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::BrownSwitch,
			sprite: Sprite::BrownSwitch,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('o', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::BlueSwitch,
			sprite: Sprite::BlueSwitch,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('(', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::Floor,
			sprite: Sprite::OnOffFloor,
			model: Model::Floor,
			solid: 0,
		});
		map.insert(')', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::Floor,
			sprite: Sprite::OnOffFloor,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('_', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			tile: Tile::Floor,
			sprite: Sprite::PanelSouth,
			model: Model::Floor,
			solid: PANEL_S,
		});

		for e in &ld.entities {
			entities::create(self, &e);
		}

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
					b'@' => {
						n.chips += 1;
						entities::pickup::create(self, x, y, Pickup::Chip);
						'.'
					}
					b'=' => {
						entities::gate::create(self, x, y);
						'.'
					}
					b'+' => {
						entities::block::create(self, x, y);
						'.'
					}
					b'b' => {
						n.keys[0] += 1;
						entities::pickup::create(self, x, y, Pickup::BlueKey);
						'.'
					}
					b'r' => {
						n.keys[1] += 1;
						entities::pickup::create(self, x, y, Pickup::RedKey);
						'.'
					}
					b'g' => {
						n.keys[2] += 1;
						entities::pickup::create(self, x, y, Pickup::GreenKey);
						'.'
					}
					b'y' => {
						n.keys[3] += 1;
						entities::pickup::create(self, x, y, Pickup::YellowKey);
						'.'
					}
					b'B' => {
						n.doors[0] += 1;
						entities::door::create(self, x, y, KeyColor::Blue);
						'.'
					}
					b'R' => {
						n.doors[1] += 1;
						entities::door::create(self, x, y, KeyColor::Red);
						'.'
					}
					b'G' => {
						n.doors[2] += 1;
						entities::door::create(self, x, y, KeyColor::Green);
						'.'
					}
					b'Y' => {
						n.doors[3] += 1;
						entities::door::create(self, x, y, KeyColor::Yellow);
						'.'
					},
					b'(' => {
						entities::wall::create(self, x, y, Some(Dir::Up));
						'('
					}
					b')' => {
						entities::wall::create(self, x, y, Some(Dir::Down));
						')'
					}
					b'*' => {
						entities::fire::create(self, x, y);
						'.'
					}
					b'O' => {
						entities::bomb::create(self, x, y);
						'.'
					}
					chr => chr as char,
				};
				let index = map[&tile];
				self.field.map.push(index as u8);
			}
		}

		self.pl.entity = self.entities.find_handle(EntityKind::Player).expect("Player entity not found");
		self.pl.object = self.objects.find_handle(EntityKind::Player).expect("Player object not found");

		let obj = self.objects.get_mut(self.pl.object).unwrap();

		self.cam.target = obj.pos;
		self.cam.eye_offset = Vec3(0.0, 8.0 * 32.0, 400.0);
	}
}
