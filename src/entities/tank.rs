use super::*;

pub fn create(game: &mut Game, x: i32, y: i32, face_dir: Option<Dir>) {
	let entity_h = game.entities.alloc();
	let object_h = game.objects.alloc();
	game.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::EnemyTank,
		pos: Vec2(x, y),
		move_dir: None,
		move_spd: 0.25,
		face_dir,
		frozen: false,
		move_time: 0.0,
	});
	game.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: EntityKind::EnemyTank,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
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

pub fn think(ent: &mut Entity, ctx: &mut ThinkContext) -> Lifecycle {
	let face_dir = ent.face_dir.unwrap_or(Dir::Up);

	if ctx.time >= ent.move_time + ent.move_spd {
		ent.move_dir = None;
	}

	if ctx.time >= ent.move_time + ent.move_spd {
		let forward_pos = ent.pos + face_dir.to_vec();

		if walkable(forward_pos, &ctx.field, &ctx.entities) {
			ent.move_dir = Some(face_dir);
			ent.move_time = ctx.time;
			ent.pos = forward_pos;
		}
	}

	return Lifecycle::KeepAlive;
}

fn walkable(pos: Vec2<i32>, field: &Field, entities: &EntityMap) -> bool {
	let tile = field.get_tile(pos);
	if tile.solid {
		return false;
	}
	for ent in entities.map.values() {
		if ent.pos != pos {
			continue;
		}
		match ent.kind {
			EntityKind::Gate => return false,
			EntityKind::Block => return false,
			EntityKind::BlueDoor => return false,
			EntityKind::RedDoor => return false,
			EntityKind::GreenDoor => return false,
			EntityKind::YellowDoor => return false,
			_ => (),
		}
	}
	return true;
}

pub fn interact(_ent: &mut Entity, _ctx: &mut ThinkContext, _ictx: &mut InteractContext) {
}

pub fn update(obj: &mut Object, ctx: &mut ThinkContext) {
	let ent = ctx.entities.get(obj.entity_handle);

	if let Some(ent) = ent {
		if ent.move_dir.is_none() {
			obj.vel = Vec3::ZERO;
		}
		obj.sprite = match ent.face_dir {
			Some(Dir::Up) => Sprite::TankUp,
			Some(Dir::Left) => Sprite::TankLeft,
			Some(Dir::Down) => Sprite::TankDown,
			Some(Dir::Right) => Sprite::TankRight,
			None => Sprite::TankUp,
		};
		obj.pos = ent.pos.map(|c| c as f32 * 32.0).vec3(0.0);
		if let Some(move_dir) = ent.move_dir {
			let t = 1.0 - (ctx.time - ent.move_time) / ent.move_spd;
			obj.pos += (-move_dir.to_vec().map(|c| c as f32 * 32.0) * t).vec3(0.0);
		}
	}
	else {
		obj.live = false;
	}
}
