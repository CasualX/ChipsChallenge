use super::*;

pub fn create(ctx: &mut SpawnContext, x: i32, y: i32, face_dir: Option<Dir>) -> EntityHandle {
	let entity_h = ctx.entities.alloc();
	let object_h = ctx.objects.alloc();
	ctx.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::Wall,
		pos: Vec2(x, y),
		move_dir: None,
		move_spd: BASE_SPD,
		move_time: 0.0,
		face_dir,
		trapped: false,
		destroy: false,
	});
	ctx.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: EntityKind::Wall,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: Sprite::Wall,
		model: Model::ThinWall,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
	});
	entity_h
}

pub fn think(_ent: &mut Entity, _ctx: &mut ThinkContext) {
}

pub fn interact(ent: &mut Entity, _ctx: &mut ThinkContext, ictx: &mut InteractContext) {
	ictx.blocking = ent.face_dir == Some(Dir::Up);
	return;
}

pub fn update(obj: &mut Object, ctx: &mut ThinkContext) {
	let ent = ctx.entities.get(obj.entity_handle);

	if let Some(ent) = ent {
		let t = (ctx.time - ent.move_time) * 120.0;
		let z = if ent.face_dir == Some(Dir::Up) { f32::min(t - 21.0, 0.0) } else { f32::max(-t, -21.0) };
		obj.pos = ent.pos.map(|c| c as f32 * 32.0).vec3(z);
	}
	else {
		obj.live = false;
	}
}
