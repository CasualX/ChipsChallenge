use super::*;

#[derive(Default)]
pub struct GameState {
	pub time: Time,
	pub ps: PlayerState,
	pub field: Field,
	pub ents: EntityMap,
	pub input: Input,
	pub events: Vec<GameEvent>,
}

impl GameState {
	pub fn tick(&mut self, input: &Input) {
		self.events.clear();
		self.time += 1;

		// Let entities think
		for handle in self.ents.map.keys().cloned().collect::<Vec<_>>() {
			if let Some(mut ent) = self.ents.remove(handle) {
				(ent.funcs.think)(self, &mut ent);
				self.ents.insert(ent);
			}
		}

		// Remove entities marked for removal
		self.ents.map.retain(|_, ent| {
			if ent.remove {
				self.events.push(GameEvent::EntityRemoved { handle: ent.handle });
			}
			!ent.remove
		});

		self.input = *input;
	}
}
