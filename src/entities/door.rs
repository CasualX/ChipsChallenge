use super::*;

pub fn create(game: &mut Game, x: i32, y: i32, key: KeyColor) {
	let entity_h = game.entities.alloc();
	let object_h = game.objects.alloc();
	game.entities.insert(Entity {
		handle: entity_h,
		kind: match key {
			KeyColor::Blue => EntityKind::BlueDoor,
			KeyColor::Red => EntityKind::RedDoor,
			KeyColor::Green => EntityKind::GreenDoor,
			KeyColor::Yellow => EntityKind::YellowDoor,
		},
		pos: Vec2(x, y),
		move_dir: None,
		face_dir: None,
		frozen: false,
		move_time: 0.0,
	});
	game.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: match key {
			KeyColor::Blue => EntityKind::BlueDoor,
			KeyColor::Red => EntityKind::RedDoor,
			KeyColor::Green => EntityKind::GreenDoor,
			KeyColor::Yellow => EntityKind::YellowDoor,
		},
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: match key {
			KeyColor::Blue => Sprite::BlueDoor,
			KeyColor::Red => Sprite::RedDoor,
			KeyColor::Green => Sprite::GreenDoor,
			KeyColor::Yellow => Sprite::YellowDoor,
		},
		model: Model::Wall,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		live: true,
	});
}

pub fn think(_ent: &mut Entity, _ctx: &mut ThinkContext) -> Lifecycle {
	Lifecycle::KeepAlive
}

pub fn interact(ent: &mut Entity, ctx: &mut ThinkContext, ictx: &mut InteractContext) {
	ictx.blocking = true;
	let (index, cost) = match ent.kind {
		EntityKind::BlueDoor => (0, 1),
		EntityKind::RedDoor => (1, 1),
		EntityKind::GreenDoor => (2, 0), // Green keys have infinite uses
		EntityKind::YellowDoor => (3, 1),
		_ => return,
	};
	if ctx.pl.inv.keys[index] > 0 {
		ctx.pl.inv.keys[index] -= cost;
		ictx.remove_entity = true;
		ictx.blocking = false;
	}
}

pub fn update(obj: &mut Object, ctx: &mut ThinkContext) {
	let ent = ctx.entities.get(obj.entity_handle);

	if ent.is_none() {
		obj.anim = Animation::Fall;
		obj.vel = Vec3(0.0, 0.0, -200.0);
	}
}
