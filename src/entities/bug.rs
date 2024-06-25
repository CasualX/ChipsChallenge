use super::*;

pub fn create(ctx: &mut SpawnContext, x: i32, y: i32, face_dir: Option<Dir>) {
	let entity_h = ctx.entities.alloc();
	let object_h = ctx.objects.alloc();
	ctx.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::Bug,
		pos: Vec2(x, y),
		move_dir: face_dir,
		move_spd: 0.25,
		face_dir,
		frozen: false,
		spawner_kind: None,
		move_time: 0.0,
	});
	ctx.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: EntityKind::Bug,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: Sprite::BugUp,
		model: Model::FlatSprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
	});
}

pub fn think(ent: &mut Entity, ctx: &mut ThinkContext) -> Lifecycle {
	if ctx.time >= ent.move_time + ent.move_spd {
		ent.move_dir = None;
	}

	if ctx.time >= ent.move_time + ent.move_spd {
		if let Some(face_dir) = ent.face_dir {
			// If bug can turn left, turn left
			if try_move(ent, face_dir.turn_left(), ctx) { }
			// Otherwise try to move forward
			else if try_move(ent, face_dir, ctx) { }
			// If forward is blocked, try to turn right
			else if try_move(ent, face_dir.turn_right(), ctx) { }
			// At this point, can't turn left, can't go forward, can't turn right so try to turn around
			else if try_move(ent, face_dir.turn_around(), ctx) { }
			// Trapped! Wait until freed
			else { }
		}
	}

	return Lifecycle::KeepAlive;
}

fn try_move(ent: &mut Entity, move_dir: Dir, ctx: &mut ThinkContext) -> bool {
	let flags = CanMoveFlags {
		gravel: true,
	};
	if !ctx.field.can_move(ent.pos, move_dir, &flags) {
		return false;
	}

	let new_pos = ent.pos + move_dir.to_vec();
	for ent in ctx.entities.map.values() {
		if ent.pos != new_pos {
			continue;
		}
		match ent.kind {
			EntityKind::Gate => return false,
			EntityKind::Block => return false,
			EntityKind::Wall if ent.face_dir == Some(Dir::Up) => return false,
			_ => (),
		}
	}

	ent.face_dir = Some(move_dir);
	ent.move_dir = Some(move_dir);
	ent.move_time = ctx.time;
	ent.pos = new_pos;
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
			let t = 1.0 - (ctx.time - ent.move_time) / ent.move_spd;
			obj.pos += (-move_dir.to_vec().map(|c| c as f32 * 32.0) * t).vec3(0.0);
		}
	}
	else {
		obj.live = false;
	}
}
