use super::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct ObjectHandle(pub u32);

#[derive(Clone, Debug)]
pub struct Object {
	pub handle: ObjectHandle,
	pub entity_handle: core::EntityHandle,
	pub entity_kind: core::EntityKind,
	pub pos: Vec3<f32>,
	pub final_pos: Vec3<f32>,
	pub vel: Vec3<f32>,
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

		self.pos += self.vel * ctx.dt;

		self.pos = self.pos.exp_decay(self.final_pos, 25.0, ctx.dt);

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

pub fn entity_created(ctx: &mut VisualState, handle: core::EntityHandle) {
	let ent = ctx.game.ents.get(handle).unwrap();
	let handle = ctx.objects.alloc();
	let obj = Object {
		handle,
		entity_handle: ent.handle,
		entity_kind: ent.kind,
		pos: Vec3::new(ent.pos.x as f32 * 32.0, ent.pos.y as f32 * 32.0, 0.0),
		final_pos: Vec3::new(ent.pos.x as f32 * 32.0, ent.pos.y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: sprite_for_ent(ent, &ctx.game.ps),
		model: Model::Sprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
	};
	if ent.kind == core::EntityKind::Player {
		ctx.camera.object_h = Some(handle);
		ctx.camera.target = obj.pos;
		ctx.camera.target_fast = obj.pos;
	}
	ctx.objects.insert(obj);
	ctx.objects.lookup.insert(ent.handle, handle);
}

pub fn entity_destroyed(ctx: &mut VisualState, handle: core::EntityHandle) {
	if let Some(handle) = ctx.objects.lookup.remove(&handle) {
		ctx.objects.remove(handle);
	}
}

pub fn entity_moved(ctx: &mut VisualState, handle: core::EntityHandle) {
	let obj = ctx.objects.lookup.get(&handle).cloned().and_then(|h| ctx.objects.get_mut(h)).unwrap();
	let ent = ctx.game.ents.get(handle).unwrap();
	obj.final_pos = Vec3::new(ent.pos.x as f32 * 32.0, ent.pos.y as f32 * 32.0, 0.0);
	obj.sprite = sprite_for_ent(ent, &ctx.game.ps);


	// if let Some(ent) = ent {
		// if ent.move_dir.is_none() {
		// 	obj.vel = Vec3::ZERO;
		// }
		// // obj.sprite = face_dir_to_sprite(ent.face_dir);
		// obj.pos = ent.pos.map(|c| c as f32 * 32.0).vec3(0.0);
		// if let Some(move_dir) = ent.move_dir {
		// 	let t = 1.0 - (ctx.time - ticks_to_time(ent.move_time)) / ticks_to_time(ent.move_spd);
		// 	obj.pos += (-move_dir.to_vec().map(|c| c as f32 * 32.0) * t).vec3(0.0);
		// }
	// }
	// else {
	// 	obj.live = false;
	// }
}

fn sprite_for_ent(ent: &core::Entity, pl: &core::PlayerState) -> Sprite {
	match ent.kind {
		core::EntityKind::Player => match pl.state {
			core::PlayerAction::Walk | core::PlayerAction::Push | core::PlayerAction::Idle | core::PlayerAction::Skate | core::PlayerAction::Suction | core::PlayerAction::Slide =>
				match ent.face_dir {
					Some(core::Dir::Up) => Sprite::PlayerWalkUp,
					Some(core::Dir::Down) => Sprite::PlayerWalkDown,
					Some(core::Dir::Left) => Sprite::PlayerWalkLeft,
					Some(core::Dir::Right) => Sprite::PlayerWalkRight,
					_ => Sprite::PlayerWalkNeutral,
				},
			core::PlayerAction::Win => Sprite::PlayerCheer,
			core::PlayerAction::Swim => match ent.face_dir {
				Some(core::Dir::Up) => Sprite::PlayerSwimUp,
				Some(core::Dir::Down) => Sprite::PlayerSwimDown,
				Some(core::Dir::Left) => Sprite::PlayerSwimLeft,
				Some(core::Dir::Right) => Sprite::PlayerSwimRight,
				_ => Sprite::PlayerSwimUp,
			},
			core::PlayerAction::Drown => Sprite::WaterSplash,
			core::PlayerAction::Burn => Sprite::PlayerBurned,
			core::PlayerAction::Death => Sprite::PlayerDead,
		},
		core::EntityKind::Chip => Sprite::Chip,
		core::EntityKind::Socket => Sprite::Socket,
		core::EntityKind::Block => Sprite::Block,
		core::EntityKind::Flippers => Sprite::PowerFlippers,
		core::EntityKind::FireBoots => Sprite::PowerFireBoots,
		core::EntityKind::IceSkates => Sprite::PowerIceSkates,
		core::EntityKind::SuctionBoots => Sprite::PowerSuctionBoots,
		core::EntityKind::BlueKey => Sprite::BlueKey,
		core::EntityKind::RedKey => Sprite::RedKey,
		core::EntityKind::GreenKey => Sprite::GreenKey,
		core::EntityKind::YellowKey => Sprite::YellowKey,
		core::EntityKind::Thief => Sprite::Thief,
		core::EntityKind::Bug => match ent.face_dir {
			Some(core::Dir::Up) => Sprite::BugUp,
			Some(core::Dir::Down) => Sprite::BugDown,
			Some(core::Dir::Left) => Sprite::BugLeft,
			Some(core::Dir::Right) => Sprite::BugRight,
			_ => Sprite::BugUp,
		},
		core::EntityKind::Tank => match ent.face_dir {
			Some(core::Dir::Up) => Sprite::TankUp,
			Some(core::Dir::Down) => Sprite::TankDown,
			Some(core::Dir::Left) => Sprite::TankLeft,
			Some(core::Dir::Right) => Sprite::TankRight,
			_ => Sprite::TankUp,
		},
		core::EntityKind::PinkBall => Sprite::PinkBall,
		core::EntityKind::FireBall => Sprite::FireBall,
		core::EntityKind::Glider => match ent.face_dir {
			Some(core::Dir::Up) => Sprite::GliderUp,
			Some(core::Dir::Down) => Sprite::GliderDown,
			Some(core::Dir::Left) => Sprite::GliderLeft,
			Some(core::Dir::Right) => Sprite::GliderRight,
			_ => Sprite::GliderUp,
		},
		core::EntityKind::Bomb => Sprite::Bomb,
	}
}

pub fn item_pickup(ctx: &mut VisualState, handle: core::EntityHandle) {
	let obj = ctx.objects.lookup.get(&handle).cloned().and_then(|h| ctx.objects.get_mut(h)).unwrap();
	obj.anim = Animation::Rise;
	obj.vel = Vec3::new(0.0, 0.0, 200.0);
}
