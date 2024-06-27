use super::*;

pub fn create(ctx: &mut SpawnContext, x: i32, y: i32) -> EntityHandle {
	let entity_h = ctx.entities.alloc();
	let object_h = ctx.objects.alloc();
	ctx.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::Socket,
		pos: Vec2(x, y),
		move_dir: None,
		move_spd: 0.0,
		move_time: 0.0,
		face_dir: None,
		trapped: false,
		destroy: false,
	});
	ctx.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: EntityKind::Socket,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: Sprite::Socket,
		model: Model::Sprite,
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

pub fn interact(_ent: &mut Entity, ctx: &mut ThinkContext, ictx: &mut InteractContext) {
	if ctx.pl.inv.chips >= ctx.field.chips {
		ictx.remove_entity = true;
		ictx.blocking = false;
	}
	else {
		ictx.blocking = true;
	}
}

pub fn update(obj: &mut Object, ctx: &mut ThinkContext) {
	let ent = ctx.entities.get(obj.entity_handle);

	if ent.is_none() {
		obj.anim = Animation::Fade;
	}
}
