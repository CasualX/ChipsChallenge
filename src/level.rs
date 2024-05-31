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
						create_chip(self, x, y);
						'.'
					},
					b'X' => 'X',
					b'=' => {
						create_barrier(self, x, y);
						'.'
					},
					b'i' => 'i',
					b'+' => {
						create_block(self, x, y);
						'.'
					},
					b'b' => {
						n.keys[0] += 1;
						create_key(self, x, y, KeyColor::Blue);
						'.'
					},
					b'r' => {
						n.keys[1] += 1;
						create_key(self, x, y, KeyColor::Red);
						'.'
					},
					b'g' => {
						n.keys[2] += 1;
						create_key(self, x, y, KeyColor::Green);
						'.'
					},
					b'y' => {
						n.keys[3] += 1;
						create_key(self, x, y, KeyColor::Yellow);
						'.'
					},
					b'B' => {
						n.doors[0] += 1;
						create_door(self, x, y, KeyColor::Blue);
						'.'
					},
					b'R' => {
						n.doors[1] += 1;
						create_door(self, x, y, KeyColor::Red);
						'.'
					},
					b'G' => {
						n.doors[2] += 1;
						create_door(self, x, y, KeyColor::Green);
						'.'
					},
					b'Y' => {
						n.doors[3] += 1;
						create_door(self, x, y, KeyColor::Yellow);
						'.'
					},
					b'1' => {
						create_bug(self, x, y);
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
		self.pl.pos = Vec2(ld.start[0], ld.start[1]);
		self.cam.eye = self.pl.pos.map(|c| c as f32).vec3(0.0) * 32.0;
		self.cam.offset = Vec3(0.0, 8.0 * 32.0, 400.0);
		self.entities.insert(Entity {
			handle: self.pl.entity,
			kind: EntityKind::Player,
			pos: self.pl.pos,
			move_dir: None,
			face_dir: None,
			move_time: 0.0,
		});
		self.objects.insert(Object {
			handle: self.pl.object,
			entity_handle: self.pl.entity,
			entity_kind: EntityKind::Player,
			pos: Vec3(ld.start[0] as f32, ld.start[1] as f32, 0.0),
			vel: Vec3::ZERO,
			sprite: Sprite::PlayerWalkNeutral,
			model: Model::Sprite,
			anim: Animation::None,
			atime: 0.0,
			alpha: 1.0,
			live: true,
		});
	}
}

fn create_chip(game: &mut Game, x: i32, y: i32) {
	let entity_h = game.entities.alloc();
	let object_h = game.objects.alloc();
	game.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::Chip,
		pos: Vec2(x, y),
		move_dir: None,
		face_dir: None,
		move_time: 0.0,
	});
	game.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: EntityKind::Chip,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: Sprite::Chip,
		model: Model::Sprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		live: true,
	});
}

fn create_barrier(game: &mut Game, x: i32, y: i32) {
	let entity_h = game.entities.alloc();
	let object_h = game.objects.alloc();
	game.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::Barrier,
		pos: Vec2(x, y),
		move_dir: None,
		face_dir: None,
		move_time: 0.0,
	});
	game.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: EntityKind::Barrier,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: Sprite::Barrier,
		model: Model::Sprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		live: true,
	});
}

fn create_block(game: &mut Game, x: i32, y: i32) {
	let entity_h = game.entities.alloc();
	let object_h = game.objects.alloc();
	game.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::Block,
		pos: Vec2(x, y),
		move_dir: None,
		face_dir: None,
		move_time: 0.0,
	});
	game.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: EntityKind::Block,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: Sprite::Block,
		model: Model::Wall,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		live: true,
	});
}

fn create_key(game: &mut Game, x: i32, y: i32, key: KeyColor) {
	let entity_h = game.entities.alloc();
	let object_h = game.objects.alloc();
	game.entities.insert(Entity {
		handle: entity_h,
		kind: match key {
			KeyColor::Blue => EntityKind::BlueKey,
			KeyColor::Red => EntityKind::RedKey,
			KeyColor::Green => EntityKind::GreenKey,
			KeyColor::Yellow => EntityKind::YellowKey,
		},
		pos: Vec2(x, y),
		move_dir: None,
		face_dir: None,
		move_time: 0.0,
	});
	game.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: match key {
			KeyColor::Blue => EntityKind::BlueKey,
			KeyColor::Red => EntityKind::RedKey,
			KeyColor::Green => EntityKind::GreenKey,
			KeyColor::Yellow => EntityKind::YellowKey,
		},
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: match key {
			KeyColor::Blue => Sprite::BlueKey,
			KeyColor::Red => Sprite::RedKey,
			KeyColor::Green => Sprite::GreenKey,
			KeyColor::Yellow => Sprite::YellowKey,
		},
		model: Model::Sprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		live: true,
	});
}

fn create_door(game: &mut Game, x: i32, y: i32, key: KeyColor) {
	let entity_h = game.entities.alloc();
	let object_h = game.objects.alloc();
	game.entities.insert(Entity {
		handle: entity_h,
		kind: match key {
			KeyColor::Blue => EntityKind::BlueDoor,
			KeyColor::Red => EntityKind::RedDoor,
			KeyColor::Green => EntityKind::GreenDoor,
			KeyColor::Yellow => EntityKind::YellowDoor,
		},
		pos: Vec2(x, y),
		move_dir: None,
		face_dir: None,
		move_time: 0.0,
	});
	game.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: match key {
			KeyColor::Blue => EntityKind::BlueDoor,
			KeyColor::Red => EntityKind::RedDoor,
			KeyColor::Green => EntityKind::GreenDoor,
			KeyColor::Yellow => EntityKind::YellowDoor,
		},
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: match key {
			KeyColor::Blue => Sprite::BlueDoor,
			KeyColor::Red => Sprite::RedDoor,
			KeyColor::Green => Sprite::GreenDoor,
			KeyColor::Yellow => Sprite::YellowDoor,
		},
		model: Model::Wall,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		live: true,
	});
}

fn create_bug(game: &mut Game, x: i32, y: i32) {
	let entity_h = game.entities.alloc();
	let object_h = game.objects.alloc();
	game.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::EnemyBug,
		pos: Vec2(x, y),
		move_dir: Some(Dir::Up),
		face_dir: Some(Dir::Up),
		move_time: 0.0,
	});
	game.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: EntityKind::EnemyBug,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: Sprite::BugUp,
		model: Model::FlatSprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		live: true,
	});
}
