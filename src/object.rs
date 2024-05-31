use super::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct ObjectHandle(pub u32);

#[derive(Clone, Debug)]
pub struct Object {
	pub handle: ObjectHandle,
	pub entity_handle: EntityHandle,
	pub entity_kind: EntityKind,
	pub pos: Vec3<f32>,
	pub vel: Vec3<f32>,
	pub sprite: Sprite,
	pub model: Model,
	pub anim: Animation,
	pub atime: f32,
	pub alpha: f32,
	pub live: bool,
}

impl Object {
	pub fn think(&mut self, ctx: &mut ThinkContext) {
		if !self.live {
			return;
		}

		let ent = ctx.entities.get(self.entity_handle);

		match self.entity_kind {
			EntityKind::Chip => {
				if ent.is_none() {
					self.anim = Animation::Rise;
					self.vel = Vec3(0.0, 0.0, 200.0);
				}
			}
			EntityKind::Block => {
				if let Some(ent) = ent {
					self.pos = ent.pos.map(|c| c as f32 * 32.0).vec3(0.0);
					if let Some(move_dir) = ent.move_dir {
						let t = 1.0 - (ctx.time - ent.move_time) / 0.125;
						self.pos += (-move_dir.to_vec().map(|c| c as f32 * 32.0) * t).vec3(0.0);
					}
				}
				else {
					self.live = false;
				}
			}
			EntityKind::Barrier => {
				if ent.is_none() {
					self.anim = Animation::Fade;
					// self.vel = Vec3(0.0, 0.0, 200.0);
				}
			}
			EntityKind::BlueKey | EntityKind::RedKey | EntityKind::GreenKey | EntityKind::YellowKey => {
				if ent.is_none() {
					self.anim = Animation::Rise;
					self.vel = Vec3(0.0, 0.0, 200.0);
				}
			}
			EntityKind::BlueDoor | EntityKind::RedDoor | EntityKind::GreenDoor | EntityKind::YellowDoor => {
				if ent.is_none() {
					self.anim = Animation::Fall;
					self.vel = Vec3(0.0, 0.0, -200.0);
				}
			}
			EntityKind::EnemyBug => {
				if let Some(ent) = ent {
					if ent.move_dir.is_none() {
						self.vel = Vec3::ZERO;
					}
					self.sprite = match ent.face_dir {
						Some(Dir::Up) => Sprite::BugUp,
						Some(Dir::Left) => Sprite::BugLeft,
						Some(Dir::Down) => Sprite::BugDown,
						Some(Dir::Right) => Sprite::BugRight,
						None => Sprite::BugUp,
					};
					self.pos = ent.pos.map(|c| c as f32 * 32.0).vec3(0.0);
					if let Some(move_dir) = ent.move_dir {
						let t = 1.0 - (ctx.time - ent.move_time) / 0.125;
						self.pos += (-move_dir.to_vec().map(|c| c as f32 * 32.0) * t).vec3(0.0);
					}
				}
				else {
					self.live = false;
				}
			}
			_ => (),
		}

		self.pos += self.vel * ctx.dt;

		if matches!(self.anim, Animation::Rise | Animation::Fade) {
			if self.atime == 0.0 {
				self.atime = ctx.time;
			}
			self.alpha = f32::max(0.0, 1.0 - (ctx.time - self.atime) * 5.0);
			if self.alpha == 0.0 {
				self.vel = Vec3::ZERO;
				self.live = false;
			}
		}
		if matches!(self.anim, Animation::Fall) {
			if self.atime == 0.0 {
				self.atime = ctx.time;
			}
			if ctx.time > self.atime + 0.5 {
				self.vel = Vec3::ZERO;
				self.live = false;
			}
		}
	}
}
