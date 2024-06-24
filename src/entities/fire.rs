use super::*;

pub fn create(ctx: &mut SpawnContext, x: i32, y: i32) {
	// let entity_h = ctx.entities.alloc();
	let object_h = ctx.objects.alloc();
	// ctx.entities.insert(Entity {
	// 	handle: entity_h,
	// 	kind: EntityKind::Fire,
	// 	pos: Vec2(x, y),
	// 	move_dir: None,
	// 	move_spd: 0.25,
	// 	face_dir: None,
	// 	frozen: false,
	// 	spawner_kind: None,
	// 	move_time: 0.0,
	// });
	ctx.objects.insert(Object {
		handle: object_h,
		entity_handle: EntityHandle::default(),
		entity_kind: EntityKind::Fire,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: Sprite::Fire,
		model: Model::Sprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
	});
}

pub fn think(ent: &mut Entity, ctx: &mut ThinkContext) -> Lifecycle {
	return Lifecycle::KeepAlive;
}

pub fn interact(_ent: &mut Entity, _ctx: &mut ThinkContext, _ictx: &mut InteractContext) {
}

pub fn update(obj: &mut Object, ctx: &mut ThinkContext) {
	// let ent = ctx.entities.get(obj.entity_handle);

	// if let Some(ent) = ent {
	// 	obj.pos = ent.pos.map(|c| c as f32 * 32.0).vec3(0.0);
	// 	if let Some(move_dir) = ent.move_dir {
	// 		let t = 1.0 - (ctx.time - ent.move_time) / ent.move_spd;
	// 		obj.pos += (-move_dir.to_vec().map(|c| c as f32 * 32.0) * t).vec3(0.0);
	// 	}
	// }
	// else {
	// 	obj.live = false;
	// }
}
