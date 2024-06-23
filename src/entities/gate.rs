use super::*;

pub fn create(game: &mut Game, x: i32, y: i32) {
	let entity_h = game.entities.alloc();
	let object_h = game.objects.alloc();
	game.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::Gate,
		pos: Vec2(x, y),
		move_dir: None,
		move_spd: 0.0,
		face_dir: None,
		frozen: false,
		spawner_kind: None,
		move_time: 0.0,
	});
	game.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: EntityKind::Gate,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: Sprite::Barrier,
		model: Model::Sprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
	});
}

pub fn think(_ent: &mut Entity, _ctx: &mut ThinkContext) -> Lifecycle {
	Lifecycle::KeepAlive
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
