use super::*;

#[derive(Default)]
pub struct VisualState {
	pub time: f32,
	pub dt: f32,
	pub game: core::GameState,
	pub camera: Camera,
	pub objects: ObjectMap,
	pub resources: Resources,
}

impl VisualState {
	pub fn load_level(&mut self, json: &str) {
		self.game.load(json);
		self.sync(&self.game.events.clone());
		self.camera.eye_offset = Vec3::new(0.0, 2.0 * 32.0, 400.0);

		for y in 0..self.game.field.height {
			for x in 0..self.game.field.width {
				let index = (y * self.game.field.width + x) as usize;
				let terrain = self.game.field.terrain[index];
				match terrain {
					core::Terrain::Fire => create_fire(self, Vec3::new(x as f32, y as f32, 0.0)),
					_ => {}
				}
			}
		}
	}
	pub fn update(&mut self, input: &core::Input) {
		self.game.tick(input);
		self.sync(&self.game.events.clone());
	}
	pub fn sync(&mut self, events: &Vec<core::GameEvent>) {
		for ev in events {
			println!("Event: {:?}", ev);
			match ev {
				&core::GameEvent::EntityCreated { handle } => entity_created(self, handle),
				&core::GameEvent::EntityRemoved { handle } => entity_removed(self, handle),
				&core::GameEvent::EntityStep { handle } => entity_step(self, handle),
				&core::GameEvent::EntityFaceDir { handle } => entity_face_dir(self, handle),
				&core::GameEvent::PlayerActionChanged { handle } => entity_face_dir(self, handle),
				&core::GameEvent::ItemPickup { handle, .. } => item_pickup(self, handle),
				&core::GameEvent::LockRemoved { pos, key } => lock_removed(self, pos, key),
				&core::GameEvent::BlueWallCleared { pos } => blue_wall_cleared(self, pos),
				&core::GameEvent::HiddenWallBumped { pos } => hidden_wall_bumped(self, pos),
				&core::GameEvent::RecessedWallRaised { pos } => recessed_wall_raised(self, pos),
				&core::GameEvent::GameWin { handle } => game_win(self, handle),
				_ => {}
			}
		}
	}
	pub fn draw(&mut self, g: &mut shade::Graphics) {
		let time = self.game.time as f32 / 60.0;
		self.time = time;
		self.dt = 1.0 / 60.0;
		let size = self.resources.screen_size;

		for handle in self.objects.map.keys().cloned().collect::<Vec<_>>() {
			let Some(mut obj) = self.objects.remove(handle) else { continue };
			obj.update(self);
			self.objects.insert(obj);
		}

		g.begin().unwrap();

		// Clear the screen
		g.clear(&shade::ClearArgs {
			surface: shade::Surface::BACK_BUFFER,
			color: Some(cvmath::Vec4(0.2, 0.2, 0.5, 1.0)),
			depth: Some(1.0),
			..Default::default()
		}).unwrap();

		self.set_game_camera();

		let mut cv = shade::d2::Canvas::<render::Vertex, render::Uniform>::new();
		cv.shader = self.resources.shader;
		cv.depth_test = Some(shade::DepthTest::Less);
		cv.viewport = cvmath::Rect::vec(cvmath::Vec2(size.x as i32, size.y as i32));
		// cv.cull_mode = Some(shade::CullMode::CW);
		cv.push_uniform(render::Uniform { transform: self.camera.view_proj_mat, texture: self.resources.tileset, texture_size: self.resources.tileset_size.map(|c| c as f32).into() });
		render::field(&mut cv, self, time);
		cv.draw(g, shade::Surface::BACK_BUFFER).unwrap();

		g.end().unwrap();
	}
}
