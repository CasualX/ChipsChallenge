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
	pub mover: MoveType,
	pub sprite: Sprite,
	pub model: Model,
	pub anim: Animation,
	pub atime: f32,
	pub alpha: f32,
	pub vis: bool,
	pub live: bool,
}

impl Object {
	pub fn update(&mut self, ctx: &mut VisualState) {
		if !self.live {
			return;
		}

		// (self.funcs.update)(self, ctx);

		match &mut self.mover {
			MoveType::Step(step) => {
				let t = f32::min(1.0, (ctx.time - step.move_time) / step.move_spd);
				let src = step.src.map(|c| c as f32 * 32.0).vec3(0.0);
				let dest = step.dest.map(|c| c as f32 * 32.0).vec3(0.0);
				self.pos = src.lerp(dest, t);
			},
			MoveType::Vel(vel) => {
				self.pos += vel.vel * ctx.dt;
				// self.pos = self.pos.exp_decay(vel.pos, 25.0, ctx.dt);
			},
		}

		if matches!(self.anim, Animation::Rise | Animation::Fade) {
			if self.atime == 0.0 {
				self.atime = ctx.time;
			}
			self.alpha = f32::max(0.0, 1.0 - (ctx.time - self.atime) * 5.0);
			if self.alpha == 0.0 {
				self.mover = MoveType::Vel(MoveVel { vel: Vec3::ZERO });
				self.live = false;
			}
		}
		if matches!(self.anim, Animation::Fall) {
			if self.atime == 0.0 {
				self.atime = ctx.time;
			}
			if ctx.time > self.atime + 0.5 {
				self.mover = MoveType::Vel(MoveVel { vel: Vec3::ZERO });
				self.live = false;
			}
		}
	}
}
