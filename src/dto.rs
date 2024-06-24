use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DataDto {
	pub tiles: Vec<TileProps>,
	pub sprites: HashMap<Sprite, [i32; 2]>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EntityDto {
	pub kind: EntityKind,
	pub pos: Vec2<i32>,
	pub face_dir: Option<Dir>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MapDto {
	pub width: i32,
	pub height: i32,
	pub strings: Vec<String>,
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
	pub entities: Vec<EntityDto>,
}

impl Game {
	pub fn load_level(&mut self, json: &str) {
		let ld: dto::LevelDto = serde_json::from_str(json).unwrap();
		if ld.map.strings.len() != 0 {
			assert_eq!(ld.map.strings.len(), ld.map.height as usize);
		}
		self.field.name = ld.name;
		self.field.hint = ld.hint;
		self.field.password = ld.password;
		self.field.time_limit = ld.time;
		self.field.chips = ld.chips;
		self.field.width = ld.map.width;
		self.field.height = ld.map.height;
		self.field.map.clear();
		self.field.tiles.clear();
		self.objects.map.clear();

		let mut map = std::collections::HashMap::new();

		map.insert(' ', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::Blank,
			sprite: Sprite::Blank,
			model: Model::Empty,
			solid: SOLID_WALL,
		});
		map.insert('.', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::Floor,
			sprite: Sprite::Floor,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('#', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::Wall,
			sprite: Sprite::Wall,
			model: Model::Wall,
			solid: SOLID_WALL,
		});
		map.insert('X', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::Exit,
			sprite: Sprite::Exit1,
			model: Model::Portal,
			solid: 0,
		});
		map.insert('i', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::Hint,
			sprite: Sprite::Hint,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('~', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::Water,
			sprite: Sprite::Water,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('\'', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::Dirt,
			sprite: Sprite::Dirt,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('%', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::Gravel,
			sprite: Sprite::Gravel,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('s', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::Ice,
			sprite: Sprite::Ice,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('t', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::IceNW,
			sprite: Sprite::IceUL,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('w', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::IceSW,
			sprite: Sprite::IceDL,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('^', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::ForceN,
			sprite: Sprite::ForceUp,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('<', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::ForceW,
			sprite: Sprite::ForceLeft,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('v', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::ForceS,
			sprite: Sprite::ForceDown,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('>', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::ForceE,
			sprite: Sprite::ForceRight,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('p', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::ButtonGreen,
			sprite: Sprite::GreenSwitch,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('m', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::ButtonRed,
			sprite: Sprite::RedSwitch,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('q', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::ButtonBrown,
			sprite: Sprite::BrownSwitch,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('o', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::ButtonBlue,
			sprite: Sprite::BlueSwitch,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('(', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::Floor,
			sprite: Sprite::OnOffFloor,
			model: Model::Floor,
			solid: 0,
		});
		map.insert(')', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::Floor,
			sprite: Sprite::OnOffFloor,
			model: Model::Floor,
			solid: 0,
		});
		map.insert('_', self.field.tiles.len());
		self.field.tiles.push(TileProps {
			terrain: Terrain::PanelS,
			sprite: Sprite::PanelSouth,
			model: Model::Floor,
			solid: PANEL_S,
		});

		for e in &ld.entities {
			let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
			entities::create(&mut ctx, &e);
			ctx.end(&mut self.objects, &mut self.entities);
		}

		if let Some(player_h) = self.objects.find_handle(EntityKind::Player) {
			self.pl.object = player_h;
			if let Some(obj) = self.objects.get(player_h) {
				self.pl.entity = obj.entity_handle;
				self.cam.target = obj.pos;
			}
		}

		self.cam.eye_offset = Vec3(0.0, 8.0 * 32.0, 400.0);

		if ld.map.strings.is_empty() {
			if ld.map.data.is_empty() {
				for _ in 0..ld.map.width * ld.map.height {
					self.field.map.push(1);
				}
			}
			else {
				for &x in ld.map.data.iter() {
					let terrain = ld.map.legend[x as usize];
					let i = self.field.tiles.iter().position(|t| t.terrain == terrain).unwrap();
					self.field.map.push(i as u8);
				}
			}
		}
		else {
			#[derive(Default)]
			struct Count {
				chips: i32,
				keys: [i32; 4],
				doors: [i32; 4],
			}
			let mut n = Count::default();

			for (y, line) in ld.map.strings.iter().enumerate() {
				assert_eq!(line.len(), ld.map.width as usize);
				for (x, chr) in line.bytes().enumerate() {
					let x = x as i32;
					let y = y as i32;
					let tile = match chr {
						b'@' => {
							n.chips += 1;
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::pickup::create(&mut ctx, x, y, Pickup::Chip);
							ctx.end(&mut self.objects, &mut self.entities);
							'.'
						}
						b'=' => {
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::gate::create(&mut ctx, x, y);
							ctx.end(&mut self.objects, &mut self.entities);
							'.'
						}
						b'+' => {
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::block::create(&mut ctx, x, y);
							ctx.end(&mut self.objects, &mut self.entities);
							'.'
						}
						b'b' => {
							n.keys[0] += 1;
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::pickup::create(&mut ctx, x, y, Pickup::BlueKey);
							ctx.end(&mut self.objects, &mut self.entities);
							'.'
						}
						b'r' => {
							n.keys[1] += 1;
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::pickup::create(&mut ctx, x, y, Pickup::RedKey);
							ctx.end(&mut self.objects, &mut self.entities);
							'.'
						}
						b'g' => {
							n.keys[2] += 1;
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::pickup::create(&mut ctx, x, y, Pickup::GreenKey);
							ctx.end(&mut self.objects, &mut self.entities);
							'.'
						}
						b'y' => {
							n.keys[3] += 1;
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::pickup::create(&mut ctx, x, y, Pickup::YellowKey);
							ctx.end(&mut self.objects, &mut self.entities);
							'.'
						}
						b'B' => {
							n.doors[0] += 1;
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::door::create(&mut ctx, x, y, KeyColor::Blue);
							ctx.end(&mut self.objects, &mut self.entities);
							'.'
						}
						b'R' => {
							n.doors[1] += 1;
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::door::create(&mut ctx, x, y, KeyColor::Red);
							ctx.end(&mut self.objects, &mut self.entities);
							'.'
						}
						b'G' => {
							n.doors[2] += 1;
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::door::create(&mut ctx, x, y, KeyColor::Green);
							ctx.end(&mut self.objects, &mut self.entities);
							'.'
						}
						b'Y' => {
							n.doors[3] += 1;
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::door::create(&mut ctx, x, y, KeyColor::Yellow);
							ctx.end(&mut self.objects, &mut self.entities);
							'.'
						},
						b'(' => {
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::wall::create(&mut ctx, x, y, Some(Dir::Up));
							ctx.end(&mut self.objects, &mut self.entities);
							'('
						}
						b')' => {
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::wall::create(&mut ctx, x, y, Some(Dir::Down));
							ctx.end(&mut self.objects, &mut self.entities);
							')'
						}
						b'*' => {
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::fire::create(&mut ctx, x, y);
							ctx.end(&mut self.objects, &mut self.entities);
							'.'
						}
						b'O' => {
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::bomb::create(&mut ctx, x, y);
							ctx.end(&mut self.objects, &mut self.entities);
							'.'
						}
						chr => chr as char,
					};
					let index = map[&tile];
					self.field.map.push(index as u8);
				}
			}
		}

	}
}
