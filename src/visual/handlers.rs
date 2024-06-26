use super::*;

pub fn entity_created(ctx: &mut VisualState, handle: core::EntityHandle) {
	let Some(ent) = ctx.game.ents.get(handle) else { return };
	let handle = ctx.objects.alloc();
	let obj = Object {
		handle,
		entity_handle: ent.handle,
		entity_kind: ent.kind,
		pos: Vec3::new(ent.pos.x as f32 * 32.0, ent.pos.y as f32 * 32.0, 0.0),
		lerp_pos: Vec3::new(ent.pos.x as f32 * 32.0, ent.pos.y as f32 * 32.0, 0.0),
		mover: MoveType::Vel(MoveVel { vel: Vec3::ZERO }),
		sprite: sprite_for_ent(ent, &ctx.game.ps),
		model: model_for_ent(ent),
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
		unalive_after_anim: false,
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
		obj.unalive_after_anim = true;
	}
	else if faded {
		obj.anim = Animation::FadeOut;
		obj.mover = MoveType::Vel(MoveVel { vel: Vec3::new(0.0, 0.0, 0.0) });
		obj.unalive_after_anim = true;
	}
	else {
		// ctx.objects.remove(obj_handle);
		obj.unalive_after_anim = true;
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

pub fn entity_teleport(ctx: &mut VisualState, handle: core::EntityHandle) {
	let Some(&obj_handle) = ctx.objects.lookup.get(&handle) else { return };
	let Some(obj) = ctx.objects.get_mut(obj_handle) else { return };
	let Some(ent) = ctx.game.ents.get(handle) else { return };

	obj.pos = ent.pos.map(|c| c as f32 * 32.0).vec3(0.0);
	obj.lerp_pos = obj.pos;
	obj.mover = MoveType::Vel(MoveVel { vel: Vec3::ZERO });
}

pub fn entity_face_dir(ctx: &mut VisualState, handle: core::EntityHandle) {
	let Some(&obj_handle) = ctx.objects.lookup.get(&handle) else { return };
	let Some(obj) = ctx.objects.get_mut(obj_handle) else { return };
	let Some(ent) = ctx.game.ents.get(handle) else { return };

	obj.sprite = sprite_for_ent(ent, &ctx.game.ps);
}

pub fn entity_hidden(ctx: &mut VisualState, handle: core::EntityHandle, hidden: bool) {
	let Some(&obj_handle) = ctx.objects.lookup.get(&handle) else { return };
	let Some(obj) = ctx.objects.get_mut(obj_handle) else { return };

	obj.vis = !hidden;
}

pub fn create_fire(ctx: &mut VisualState, pos: Vec2<i32>) {
	let handle = ctx.objects.alloc();
	let obj = Object {
		handle,
		entity_handle: core::EntityHandle::default(),
		entity_kind: core::EntityKind::RedKey,
		pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 0.0),
		lerp_pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 0.0),
		mover: MoveType::Vel(MoveVel { vel: Vec3(0.0, 0.0, 0.0) }),
		sprite: Sprite::Fire,
		model: Model::Sprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
		unalive_after_anim: false,
	};
	ctx.objects.insert(obj);
}

pub fn create_toggle_floor(ctx: &mut VisualState, pos: Vec2<i32>) {
	let handle = ctx.objects.alloc();
	let obj = Object {
		handle,
		entity_handle: core::EntityHandle::default(),
		entity_kind: core::EntityKind::RedKey,
		pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, -21.0),
		lerp_pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, -21.0),
		mover: MoveType::Vel(MoveVel { vel: Vec3(0.0, 0.0, 0.0) }),
		sprite: Sprite::Wall,
		model: Model::ThinWall,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
		unalive_after_anim: false,
	};
	ctx.objects.insert(obj);
}

pub fn create_toggle_wall(ctx: &mut VisualState, pos: Vec2<i32>) {
	let handle = ctx.objects.alloc();
	let obj = Object {
		handle,
		entity_handle: core::EntityHandle::default(),
		entity_kind: core::EntityKind::RedKey,
		pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 0.0),
		lerp_pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 0.0),
		mover: MoveType::Vel(MoveVel { vel: Vec3(0.0, 0.0, 0.0) }),
		sprite: Sprite::Wall,
		model: Model::ThinWall,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
		unalive_after_anim: false,
	};
	ctx.objects.insert(obj);
}

fn model_for_ent(ent: &core::Entity) -> Model {
	match ent.kind {
		core::EntityKind::Block => Model::Wall,
		core::EntityKind::Tank => Model::ReallyFlatSprite,
		core::EntityKind::Bug => Model::FlatSprite,
		core::EntityKind::Blob => Model::ReallyFlatSprite,
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
		core::EntityKind::Walker => match ent.face_dir {
			Some(core::Dir::Up) | Some(core::Dir::Down) => Sprite::WalkerUpDown,
			Some(core::Dir::Left) | Some(core::Dir::Right) => Sprite::WalkerLeftRight,
			_ => Sprite::WalkerUpDown,
		},
		core::EntityKind::Teeth => match ent.face_dir {
			Some(core::Dir::Up) => Sprite::TeethUp,
			Some(core::Dir::Down) => Sprite::TeethDown,
			Some(core::Dir::Left) => Sprite::TeethLeft,
			Some(core::Dir::Right) => Sprite::TeethRight,
			_ => Sprite::TeethUp,
		},
		core::EntityKind::Blob => Sprite::Blob,
		core::EntityKind::Paramecium => match ent.face_dir {
			Some(core::Dir::Up) | Some(core::Dir::Down) => Sprite::ParameciumUpDown,
			Some(core::Dir::Left) | Some(core::Dir::Right) => Sprite::ParameciumLeftRight,
			_ => Sprite::ParameciumUpDown,
		}
		core::EntityKind::Bomb => Sprite::Bomb,
	}
}

pub fn item_pickup(ctx: &mut VisualState, ehandle: core::EntityHandle) {
	let Some(&obj_handle) = ctx.objects.lookup.get(&ehandle) else { return };
	let Some(obj) = ctx.objects.get_mut(obj_handle) else { return };

	obj.anim = Animation::Rise;
	obj.mover = MoveType::Vel(MoveVel { vel: Vec3::new(0.0, 0.0, 200.0) });
}

pub fn lock_removed(ctx: &mut VisualState, pos: Vec2<i32>, key: core::KeyColor) {
	let handle = ctx.objects.alloc();
	let obj = Object {
		handle,
		entity_handle: core::EntityHandle::default(),
		entity_kind: core::EntityKind::RedKey,
		pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 0.0),
		lerp_pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 0.0),
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
		unalive_after_anim: true,
	};
	ctx.objects.insert(obj);
}

pub fn blue_wall_cleared(ctx: &mut VisualState, pos: Vec2<i32>) {
	let handle = ctx.objects.alloc();
	let obj = Object {
		handle,
		entity_handle: core::EntityHandle::default(),
		entity_kind: core::EntityKind::RedKey,
		pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 0.0),
		lerp_pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 0.0),
		mover: MoveType::Vel(MoveVel { vel: Vec3(0.0, 0.0, 0.0) }),
		sprite: Sprite::BlueWall,
		model: Model::Wall,
		anim: Animation::FadeOut,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
		unalive_after_anim: true,
	};
	ctx.objects.insert(obj);
}

pub fn hidden_wall_bumped(ctx: &mut VisualState, pos: Vec2<i32>) {
	let handle = ctx.objects.alloc();
	let obj = Object {
		handle,
		entity_handle: core::EntityHandle::default(),
		entity_kind: core::EntityKind::RedKey,
		pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 0.0),
		lerp_pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 0.0),
		mover: MoveType::Vel(MoveVel { vel: Vec3(0.0, 0.0, 0.0) }),
		sprite: Sprite::Wall,
		model: Model::Wall,
		anim: Animation::FadeIn,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
		unalive_after_anim: false,
	};
	ctx.objects.insert(obj);
}

pub fn recessed_wall_raised(ctx: &mut VisualState, pos: Vec2<i32>) {
	let handle = ctx.objects.alloc();
	let obj = Object {
		handle,
		entity_handle: core::EntityHandle::default(),
		entity_kind: core::EntityKind::RedKey,
		pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 0.0),
		lerp_pos: Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 0.0),
		mover: MoveType::Vel(MoveVel { vel: Vec3(0.0, 0.0, 0.0) }),
		sprite: Sprite::Wall,
		model: Model::Wall,
		anim: Animation::Raise,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
		unalive_after_anim: false,
	};
	ctx.objects.insert(obj);
}

pub fn toggle_walls(ctx: &mut VisualState) {
	for obj in ctx.objects.map.values_mut() {
		if obj.model != Model::ThinWall {
			continue;
		}

		let pos = obj.pos.xy().map(|c| (c / 32.0) as i32);
		let terrain = ctx.game.field.get_terrain(pos);
		if matches!(terrain, core::Terrain::ToggleFloor) {
			obj.pos.z = 0.0;
			obj.anim = Animation::Fall;
			obj.mover = MoveType::Vel(MoveVel { vel: Vec3(0.0, 0.0, -200.0) });
		}
		else if matches!(terrain, core::Terrain::ToggleWall) {
			obj.pos.z = -21.0;
			obj.anim = Animation::Raise;
			obj.mover = MoveType::Vel(MoveVel { vel: Vec3(0.0, 0.0, 200.0) });
		}
	}
}
