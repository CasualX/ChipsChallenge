use super::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct ObjectHandle(pub u32);

#[derive(Clone, Debug)]
pub struct MoveStep {
	pub src: Vec2<i32>,
	pub dest: Vec2<i32>,
	pub move_time: f32,
	pub move_spd: f32,
}

#[derive(Clone, Debug)]
pub struct MoveVel {
	pub vel: Vec3<f32>,
}

#[derive(Clone, Debug)]
pub enum MoveType {
	Step(MoveStep),
	Vel(MoveVel),
}

#[derive(Clone, Debug)]
pub struct Object {
	pub handle: ObjectHandle,
	pub entity_handle: core::EntityHandle,
	pub entity_kind: core::EntityKind,
	pub pos: Vec3<f32>,
	pub lerp_pos: Vec3<f32>,
	pub mover: MoveType,
	pub sprite: Sprite,
	pub model: Model,
	pub anim: Animation,
	pub atime: f32,
	pub alpha: f32,
	pub vis: bool,
	pub live: bool,
	pub unalive_after_anim: bool,
}

impl Object {
	pub fn update(&mut self, ctx: &mut VisualState) {
		if !self.live {
			return;
		}

		match &mut self.mover {
			MoveType::Step(step) => {
				let t = f32::min(1.0, (ctx.time - step.move_time) / step.move_spd);
				let src = step.src.map(|c| c as f32 * 32.0).vec3(0.0);
				let dest = step.dest.map(|c| c as f32 * 32.0).vec3(0.0);
				self.lerp_pos = src.lerp(dest, t);
				self.pos = self.pos.exp_decay(dest, 100.0 * step.move_spd, ctx.dt);
				if t > 0.75 {
					self.pos = dest;
				}
				if t >= 0.75 && self.unalive_after_anim {
					self.live = false;
					println!("Object {:?} reached destination", self.handle);
				}
				return;
			},
			MoveType::Vel(vel) => {
				self.pos += vel.vel * ctx.dt;
			},
		}

		match self.anim {
			Animation::Rise | Animation::FadeOut => {
				if self.atime == 0.0 {
					self.atime = ctx.time;
				}
				self.alpha = f32::max(0.0, 1.0 - (ctx.time - self.atime) * 5.0);
				if self.alpha == 0.0 {
					self.mover = MoveType::Vel(MoveVel { vel: Vec3::ZERO });
					if self.unalive_after_anim {
						self.live = false;
					}
				}
			}
			Animation::FadeIn => {
				if self.atime == 0.0 {
					self.atime = ctx.time;
				}
				self.alpha = f32::min(1.0, (ctx.time - self.atime) * 10.0);
				if self.alpha == 1.0 {
					self.mover = MoveType::Vel(MoveVel { vel: Vec3::ZERO });
				}
			}
			Animation::Fall => {
				if self.atime == 0.0 {
					self.atime = ctx.time;
				}
				if self.pos.z <= -20.0 {
					self.pos.z = -21.0;
					self.mover = MoveType::Vel(MoveVel { vel: Vec3::ZERO });
					if self.unalive_after_anim {
						self.live = false;
					}
				}
			}
			Animation::Raise => {
				if self.atime == 0.0 {
					self.atime = ctx.time;
					self.mover = MoveType::Vel(MoveVel { vel: Vec3(0.0, 0.0, 200.0) });
					self.pos.z = -20.0;
				}
				if self.pos.z >= 0.0 {
					self.pos.z = 0.0;
					self.mover = MoveType::Vel(MoveVel { vel: Vec3::ZERO });
					if self.unalive_after_anim {
						self.live = false;
					}
				}
			}
			Animation::None => {
				if self.unalive_after_anim {
					self.live = false;
				}
			}
		}
	}
}
