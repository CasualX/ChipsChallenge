use super::*;

pub fn create(ctx: &mut SpawnContext, x: i32, y: i32, face_dir: Option<Dir>) -> EntityHandle {
	let entity_h = ctx.entities.alloc();
	let object_h = ctx.objects.alloc();
	ctx.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::PinkBall,
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
		entity_kind: EntityKind::PinkBall,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: Sprite::PinkBall,
		model: Model::Sprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
	});
	entity_h
}

pub fn think(ent: &mut Entity, ctx: &mut ThinkContext) {
	if ent.move_dir.is_some() && ctx.time >= ent.move_time + ent.move_spd {
		let terrain = ctx.field.get_terrain(ent.pos);

		if matches!(terrain, Terrain::GreenButton) {
			entities::press_green_button(ctx);
		}
		if matches!(terrain, Terrain::RedButton) {
			entities::press_red_button(ctx, ent.pos);
		}
		if matches!(terrain, Terrain::BrownButton) {
			entities::press_brown_button(ctx, ent.pos);
		}
		if matches!(terrain, Terrain::BlueButton) {
			entities::press_blue_button(ctx);
		}
		if matches!(terrain, Terrain::BearTrap) {
			ent.trapped = true;
		}

		ent.move_dir = None;
	}

	if !ent.trapped && ctx.time >= ent.move_time + ent.move_spd {
		if let Some(face_dir) = ent.face_dir {
			if try_move(ent, face_dir, ctx) { }
			else if try_move(ent, face_dir.turn_around(), ctx) { }
			else { }
		}
	}
}

fn try_move(ent: &mut Entity, move_dir: Dir, ctx: &mut ThinkContext) -> bool {
	let flags = CanMoveFlags {
		gravel: false,
		fire: true,
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
			EntityKind::Socket => return false,
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
