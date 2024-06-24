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
		self.objects.map.clear();

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
					self.field.map.push(Terrain::Floor);
				}
			}
			else {
				for &index in ld.map.data.iter() {
					let x = self.field.map.len() as i32 % ld.map.width;
					let y = self.field.map.len() as i32 / ld.map.width;

					let terrain = ld.map.legend[index as usize];
					self.field.map.push(terrain);
					if matches!(terrain, Terrain::Fire) {
						let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
						entities::fire::create(&mut ctx, x, y);
						ctx.end(&mut self.objects, &mut self.entities);
					}
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
					let terrain = match chr {
						b' ' => Terrain::Blank,
						b'.' => Terrain::Floor,
						b'#' => Terrain::Wall,
						b'X' => Terrain::Exit,
						b'i' => Terrain::Hint,
						b'~' => Terrain::Water,
						b'^' => Terrain::ForceN,
						b'v' => Terrain::ForceS,
						b'<' => Terrain::ForceW,
						b'>' => Terrain::ForceE,
						b's' => Terrain::Ice,
						b't' => Terrain::IceNW,
						b'w' => Terrain::IceSW,
						b'_' => Terrain::PanelS,
						b'@' => {
							n.chips += 1;
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::pickup::create(&mut ctx, x, y, Pickup::Chip);
							ctx.end(&mut self.objects, &mut self.entities);
							Terrain::Floor
						}
						b'=' => {
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::gate::create(&mut ctx, x, y);
							ctx.end(&mut self.objects, &mut self.entities);
							Terrain::Floor
						}
						b'+' => {
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::block::create(&mut ctx, x, y);
							ctx.end(&mut self.objects, &mut self.entities);
							Terrain::Floor
						}
						b'b' => {
							n.keys[0] += 1;
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::pickup::create(&mut ctx, x, y, Pickup::BlueKey);
							ctx.end(&mut self.objects, &mut self.entities);
							Terrain::Floor
						}
						b'r' => {
							n.keys[1] += 1;
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::pickup::create(&mut ctx, x, y, Pickup::RedKey);
							ctx.end(&mut self.objects, &mut self.entities);
							Terrain::Floor
						}
						b'g' => {
							n.keys[2] += 1;
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::pickup::create(&mut ctx, x, y, Pickup::GreenKey);
							ctx.end(&mut self.objects, &mut self.entities);
							Terrain::Floor
						}
						b'y' => {
							n.keys[3] += 1;
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::pickup::create(&mut ctx, x, y, Pickup::YellowKey);
							ctx.end(&mut self.objects, &mut self.entities);
							Terrain::Floor
						}
						b'B' => {
							n.doors[0] += 1;
							// let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							// entities::door::create(&mut ctx, x, y, KeyColor::Blue);
							// ctx.end(&mut self.objects, &mut self.entities);
							Terrain::BlueLock
						}
						b'R' => {
							n.doors[1] += 1;
							// let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							// entities::door::create(&mut ctx, x, y, KeyColor::Red);
							// ctx.end(&mut self.objects, &mut self.entities);
							Terrain::RedLock
						}
						b'G' => {
							n.doors[2] += 1;
							// let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							// entities::door::create(&mut ctx, x, y, KeyColor::Green);
							// ctx.end(&mut self.objects, &mut self.entities);
							Terrain::GreenLock
						}
						b'Y' => {
							n.doors[3] += 1;
							// let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							// entities::door::create(&mut ctx, x, y, KeyColor::Yellow);
							// ctx.end(&mut self.objects, &mut self.entities);
							Terrain::YellowLock
						},
						b'(' => {
							// let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							// entities::wall::create(&mut ctx, x, y, Some(Dir::Up));
							// ctx.end(&mut self.objects, &mut self.entities);
							Terrain::ToggleWall
						}
						b')' => {
							// let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							// entities::wall::create(&mut ctx, x, y, Some(Dir::Down));
							// ctx.end(&mut self.objects, &mut self.entities);
							Terrain::ToggleFloor
						}
						b'*' => {
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::fire::create(&mut ctx, x, y);
							ctx.end(&mut self.objects, &mut self.entities);
							Terrain::Fire
						}
						b'o' => Terrain::BlueButton,
						b'p' => Terrain::GreenButton,
						b'q' => Terrain::BrownButton,
						b'm' => Terrain::RedButton,
						b'O' => {
							let mut ctx = SpawnContext::begin(&mut self.objects, &mut self.entities);
							entities::bomb::create(&mut ctx, x, y);
							ctx.end(&mut self.objects, &mut self.entities);
							Terrain::Floor
						}
						chr => unimplemented!("{}", chr as char),
					};
					self.field.map.push(terrain);
				}
			}
		}

	}
}
