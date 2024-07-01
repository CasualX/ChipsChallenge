use super::*;

pub struct Random {
	pub rng: urandom::Random<urandom::rng::Xoshiro256>,
}
impl Default for Random {
	fn default() -> Self {
		Random {
			rng: urandom::rng::Xoshiro256::new(),
		}
	}
}

#[derive(Default)]
pub struct GameState {
	pub time: Time,
	pub ps: PlayerState,
	pub field: Field,
	pub ents: EntityMap,
	pub input: Input,
	pub events: Vec<GameEvent>,
	pub rand: Random,
}

impl GameState {
	pub fn load(&mut self, json: &str) {
		self.time = 0;

		let ld: dto::LevelDto = serde_json::from_str(json).unwrap();
		self.field.name = ld.name;
		self.field.hint = ld.hint;
		self.field.password = ld.password;
		self.rand.rng = urandom::rng::Xoshiro256::from_seed(ld.seed);
		self.field.time = ld.time;
		self.field.chips = ld.chips;
		self.field.width = ld.map.width;
		self.field.height = ld.map.height;
		self.field.terrain.clear();
		self.field.conns = ld.connections;

		assert!(ld.map.width > 0, "Invalid map width");
		assert!(ld.map.height > 0, "Invalid map height");
		let size = ld.map.width as usize * ld.map.height as usize;
		self.field.terrain.reserve_exact(size);

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

		for data in &ld.entities {
			entities::create(self, data);
		}

		update_hidden_entities(self);
	}
}

impl GameState {
	pub fn tick(&mut self, input: &Input) {
		self.events.clear();
		self.time += 1;

		ps_update_moves(self, input);

		// Let entities think
		let keys = self.ents.map.keys().cloned().collect::<Vec<_>>();
		for &handle in &keys {
			if let Some(mut ent) = self.ents.remove(handle) {
				if !matches!(ent.kind, EntityKind::Player) {
					(ent.funcs.think)(self, &mut ent);
				}
				self.ents.insert(ent);
			}
		}

		// Simulate the player last
		if let Some(mut ent) = self.ents.remove(self.ps.entity) {
			(ent.funcs.think)(self, &mut ent);
			self.ents.insert(ent);
		}

		// Handle entity-terrain interactions
		for &handle in &keys {
			if let Some(mut ent) = self.ents.remove(handle) {
				interact_terrain(self, &mut ent);
				self.ents.insert(ent);
			}
		}

		// Remove entities marked for removal
		self.ents.map.retain(|_, ent| {
			if ent.remove {
				self.events.push(GameEvent::EntityRemoved { entity: ent.handle });
			}
			!ent.remove
		});

		self.input = *input;
	}
}
