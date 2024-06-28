use super::*;

pub fn entity_created(ctx: &mut VisualState, handle: core::EntityHandle) {
	let Some(ent) = ctx.game.ents.get(handle) else { return };
	let handle = ctx.objects.alloc();
	let obj = Object {
		handle,
		entity_handle: ent.handle,
		entity_kind: ent.kind,
		pos: Vec3::new(ent.pos.x as f32 * 32.0, ent.pos.y as f32 * 32.0, 0.0),
		mover: MoveType::Vel(MoveVel { vel: Vec3::ZERO }),
		sprite: sprite_for_ent(ent, &ctx.game.ps),
		model: model_for_ent(ent),
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
	};
	if matches!(ent.kind, core::EntityKind::Player) {
		ctx.camera.object_h = Some(handle);
		ctx.camera.target = obj.pos;
		ctx.camera.target_fast = obj.pos;
	}
	ctx.objects.insert(obj);
	ctx.objects.lookup.insert(ent.handle, handle);
}

pub fn entity_removed(ctx: &mut VisualState, handle: core::EntityHandle) {
	let Some(obj_handle) = ctx.objects.lookup.remove(&handle) else { return };
	let Some(obj) = ctx.objects.get_mut(obj_handle) else { return };

	// Object rises, fades and is removed
	let rises = matches!(obj.entity_kind, core::EntityKind::Chip
		| core::EntityKind::BlueKey | core::EntityKind::RedKey | core::EntityKind::GreenKey | core::EntityKind::YellowKey
		| core::EntityKind::Flippers | core::EntityKind::FireBoots | core::EntityKind::IceSkates | core::EntityKind::SuctionBoots);

	// Object fades and is removed
	let faded = matches!(obj.entity_kind, core::EntityKind::Socket);

	if rises {
		obj.anim = Animation::Rise;
		obj.mover = MoveType::Vel(MoveVel { vel: Vec3::new(0.0, 0.0, 200.0) });
	}
	else if faded {
		obj.anim = Animation::Fade;
		obj.mover = MoveType::Vel(MoveVel { vel: Vec3::new(0.0, 0.0, 0.0) });
	}
	else {
		ctx.objects.remove(obj_handle);
	}
}

pub fn entity_step(ctx: &mut VisualState, handle: core::EntityHandle) {
	let Some(&obj_handle) = ctx.objects.lookup.get(&handle) else { return };
	let Some(obj) = ctx.objects.get_mut(obj_handle) else { return };
	let Some(ent) = ctx.game.ents.get(handle) else { return };

	// obj.sprite = sprite_for_ent(ent, &ctx.game.ps);
	obj.mover = MoveType::Step(MoveStep {
		src: ent.pos - ent.step_dir.unwrap().to_vec(),
		dest: ent.pos,
		move_time: ctx.time,
		move_spd: ent.step_spd as f32 / 60.0,
	});
}

pub fn entity_face_dir(ctx: &mut VisualState, handle: core::EntityHandle) {
	let Some(&obj_handle) = ctx.objects.lookup.get(&handle) else { return };
	let Some(obj) = ctx.objects.get_mut(obj_handle) else { return };
	let Some(ent) = ctx.game.ents.get(handle) else { return };

	obj.sprite = sprite_for_ent(ent, &ctx.game.ps);
}

pub fn create_fire(ctx: &mut VisualState, pos: Vec3<f32>) {
	let handle = ctx.objects.alloc();
	let obj = Object {
		handle,
		entity_handle: core::EntityHandle::default(),
		entity_kind: core::EntityKind::RedKey,
		pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 0.0),
		mover: MoveType::Vel(MoveVel { vel: Vec3(0.0, 0.0, 0.0) }),
		sprite: Sprite::Fire,
		model: Model::Sprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
	};
	ctx.objects.insert(obj);
}

fn model_for_ent(ent: &core::Entity) -> Model {
	match ent.kind {
		core::EntityKind::Block => Model::Wall,
		_ => Model::Sprite,
	}
}

fn sprite_for_ent(ent: &core::Entity, pl: &core::PlayerState) -> Sprite {
	match ent.kind {
		core::EntityKind::Player => match pl.action {
			core::PlayerAction::Walk | core::PlayerAction::Push | core::PlayerAction::Skate | core::PlayerAction::Suction | core::PlayerAction::Slide =>
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
				_ => Sprite::PlayerSwimNeutral,
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

pub fn item_pickup(ctx: &mut VisualState, ehandle: core::EntityHandle) {
	let Some(&obj_handle) = ctx.objects.lookup.get(&ehandle) else { return };
	let Some(obj) = ctx.objects.get_mut(obj_handle) else { return };

	obj.anim = Animation::Rise;
	obj.mover = MoveType::Vel(MoveVel { vel: Vec3::new(0.0, 0.0, 200.0) });
}

pub fn game_win(ctx: &mut VisualState) {
	let Some(&obj_handle) = ctx.objects.lookup.get(&ctx.game.ps.entity) else { return };
	let Some(obj) = ctx.objects.get_mut(obj_handle) else { return };

	obj.sprite = Sprite::PlayerCheer;
}

pub fn lock_removed(ctx: &mut VisualState, pos: Vec2<i32>, key: core::KeyColor) {
	let handle = ctx.objects.alloc();
	let obj = Object {
		handle,
		entity_handle: core::EntityHandle::default(),
		entity_kind: core::EntityKind::RedKey,
		pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 0.0),
		mover: MoveType::Vel(MoveVel { vel: Vec3(0.0, 0.0, -200.0) }),
		sprite: match key {
			core::KeyColor::Red => Sprite::RedLock,
			core::KeyColor::Green => Sprite::GreenLock,
			core::KeyColor::Blue => Sprite::BlueLock,
			core::KeyColor::Yellow => Sprite::YellowLock,
		},
		model: Model::Wall,
		anim: Animation::Fall,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
	};
	ctx.objects.insert(obj);
}
