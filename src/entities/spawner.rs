use super::*;

pub fn create(game: &mut Game, x: i32, y: i32, face_dir: Option<Dir>, spawner_kind: Option<EntityKind>) {
	let entity_h = game.entities.alloc();
	let object_h = game.objects.alloc();
	game.entities.insert(Entity {
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
	game.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: EntityKind::Spawner,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 14.0),
		vel: Vec3::ZERO,
		sprite: Sprite::BugUp,
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

pub fn interact(_ent: &mut Entity, _ctx: &mut ThinkContext, _ictx: &mut InteractContext) {
}

pub fn update(obj: &mut Object, ctx: &mut ThinkContext) {
	let ent = ctx.entities.get(obj.entity_handle);

	if ent.is_none() {
		obj.live = false;
	}
}
