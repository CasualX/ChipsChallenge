use super::*;

pub fn create(ctx: &mut GameState, data: &SpawnData) -> EntityHandle {
	let handle = ctx.ents.alloc();
	ctx.ents.insert(Entity {
		funcs: &FUNCS,
		handle,
		kind: data.kind,
		pos: data.pos,
		move_dir: None,
		move_spd: BASE_SPD,
		move_time: 0,
		face_dir: data.face_dir,
		trapped: false,
		remove: false,
	});
	return handle;
}

fn think(ent: &mut Entity, s: &mut GameState) {
	if s.time >= ent.move_time + ent.move_spd {
		ent.move_dir = None;
	}

	if s.time >= ent.move_time + ent.move_spd {
		if let Some(face_dir) = ent.face_dir {
			try_move(ent, face_dir, s);
		}
	}
}

fn try_move(ent: &mut Entity, move_dir: Dir, s: &mut GameState) -> bool {
	let flags = CanMoveFlags {
		gravel: false,
		fire: true,
	};
	if !s.field.can_move(ent.pos, move_dir, &flags) {
		return false;
	}

	let new_pos = ent.pos + move_dir.to_vec();
	for ent in s.ents.map.values() {
		if ent.pos != new_pos {
			continue;
		}
		match ent.kind {
			EntityKind::Socket => return false,
			EntityKind::Block if ent.face_dir == Some(Dir::Up) => return false,
			_ => (),
		}
	}

	ent.face_dir = Some(move_dir);
	ent.move_dir = Some(move_dir);
	ent.move_time = s.time;
	ent.pos = new_pos;
	return true;
}

fn interact(_ent: &mut Entity, _s: &mut GameState, _ictx: &mut InteractContext) {
}

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
