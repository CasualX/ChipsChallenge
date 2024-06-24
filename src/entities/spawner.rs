use super::*;

pub fn create(ctx: &mut SpawnContext, x: i32, y: i32, face_dir: Option<Dir>, spawner_kind: Option<EntityKind>) {
	let entity_h = ctx.entities.alloc();
	let object_h = ctx.objects.alloc();
	ctx.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::Spawner,
		pos: Vec2(x, y),
		move_dir: None,
		move_spd: 0.25,
		face_dir,
		frozen: false,
		spawner_kind,
		move_time: 0.0,
	});
	let sprite = match spawner_kind {
		Some(EntityKind::Tank) => Sprite::BugUp,
		Some(EntityKind::PinkBall) => Sprite::PinkBall,
		Some(EntityKind::Bug) => match face_dir {
			Some(Dir::Up) => Sprite::BugUp,
			Some(Dir::Down) => Sprite::BugDown,
			Some(Dir::Left) => Sprite::BugLeft,
			Some(Dir::Right) => Sprite::BugRight,
			None => panic!("Spawner with no face direction"),
		},
		Some(EntityKind::FireBall) => Sprite::FireBall,
		_ => panic!("Invalid spawner kind: {:?}", spawner_kind),
	};
	ctx.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: EntityKind::Spawner,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 14.0),
		vel: Vec3::ZERO,
		sprite,
		model: Model::ReallyFlatSprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
	});
}

pub fn think(_ent: &mut Entity, _ctx: &mut ThinkContext) -> Lifecycle {
	return Lifecycle::KeepAlive;
}

pub fn spawn(ent: &mut Entity, ctx: &mut ThinkContext) {
	// let spawn_pos = ent.pos + ent.face_dir.unwrap().to_vec();
	let mut spawn_ctx = SpawnContext::begin(&mut ctx.objects, &mut ctx.entities);
	entities::create(&mut spawn_ctx, &dto::EntityDto {
		kind: ent.spawner_kind.unwrap(),
		pos: ent.pos.into(),
		face_dir: ent.face_dir,
		// spawner_kind: None,
	});
	spawn_ctx.end(&mut ctx.objects, &mut ctx.entities);
}

pub fn interact(_ent: &mut Entity, _ctx: &mut ThinkContext, _ictx: &mut InteractContext) {
}

pub fn update(obj: &mut Object, ctx: &mut ThinkContext) {
	let ent = ctx.entities.get(obj.entity_handle);

	if ent.is_none() {
		obj.live = false;
	}
}
