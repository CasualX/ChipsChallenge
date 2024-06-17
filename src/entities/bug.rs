use super::*;

const MOVE_SPEED: f32 = 0.25;// * 0.5;
const MOVE_DELAY: f32 = 0.0;//0.125;

pub fn create(game: &mut Game, x: i32, y: i32) {
	let entity_h = game.entities.alloc();
	let object_h = game.objects.alloc();
	game.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::EnemyBug,
		pos: Vec2(x, y),
		move_dir: Some(Dir::Up),
		face_dir: Some(Dir::Up),
		frozen: false,
		move_time: 0.0,
	});
	game.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: EntityKind::EnemyBug,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: Sprite::BugUp,
		model: Model::FlatSprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		live: true,
	});
}

pub fn think(ent: &mut Entity, ctx: &mut ThinkContext) -> Lifecycle {
	let mut face_dir = ent.face_dir.unwrap_or(Dir::Up);

	if ctx.time >= ent.move_time + MOVE_SPEED {
		ent.move_dir = None;
	}

	if ctx.time >= ent.move_time + MOVE_SPEED + MOVE_DELAY {
		let left_pos = ent.pos + face_dir.turn_left().to_vec();
		let forward_pos = ent.pos + face_dir.to_vec();
		let right_pos = ent.pos + face_dir.turn_right().to_vec();
		let back_pos = ent.pos - face_dir.to_vec();

		// If bug can turn left, turn left
		if walkable(left_pos, &ctx.field, &ctx.entities) {
			face_dir = face_dir.turn_left();
		}
		// Otherwise try to move forward
		else if walkable(forward_pos, &ctx.field, &ctx.entities) {
		}
		// If forward is blocked, try to turn right
		else if walkable(right_pos, &ctx.field, &ctx.entities) {
			face_dir = face_dir.turn_right();
		}
		// At this point, can't turn left, can't go forward, can't turn right so try to turn around
		else if walkable(back_pos, &ctx.field, &ctx.entities) {
			face_dir = face_dir.turn_around();
		}
		else {
			// Trapped! Wait until freed
			return Lifecycle::KeepAlive;
		}

		ent.face_dir = Some(face_dir);
		ent.move_dir = Some(face_dir);
		ent.move_time = ctx.time;
		ent.pos += face_dir.to_vec();
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
			Some(Dir::Up) => Sprite::BugUp,
			Some(Dir::Left) => Sprite::BugLeft,
			Some(Dir::Down) => Sprite::BugDown,
			Some(Dir::Right) => Sprite::BugRight,
			None => Sprite::BugUp,
		};
		obj.pos = ent.pos.map(|c| c as f32 * 32.0).vec3(0.0);
		if let Some(move_dir) = ent.move_dir {
			let t = 1.0 - (ctx.time - ent.move_time) / MOVE_SPEED;
			obj.pos += (-move_dir.to_vec().map(|c| c as f32 * 32.0) * t).vec3(0.0);
		}
	}
	else {
		obj.live = false;
	}
}
