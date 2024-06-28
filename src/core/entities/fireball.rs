use super::*;

pub fn create(s: &mut GameState, data: &SpawnData) -> EntityHandle {
	let handle = s.ents.alloc();
	s.ents.insert(Entity {
		funcs: &FUNCS,
		handle,
		kind: data.kind,
		pos: data.pos,
		face_dir: data.face_dir,
		step_dir: None,
		step_spd: BASE_SPD,
		step_time: 0,
		trapped: false,
		remove: false,
	});
	return handle;
}

fn think(s: &mut GameState, ent: &mut Entity) {
	let terrain = s.field.get_terrain(ent.pos);

	if matches!(terrain, Terrain::CloneMachine) && ent.step_dir.is_none() {
		return;
	}

	if ent.step_dir.is_some() && s.time >= ent.step_time + ent.step_spd {
		ent.step_dir = None;

		if matches!(terrain, Terrain::Water) {
			ent.remove = true;
			return;
		}
	}

	if s.time >= ent.step_time + ent.step_spd {
		if let Some(face_dir) = ent.face_dir {
			if try_move(s, ent, face_dir) { }
			else if try_move(s, ent, face_dir.turn_right()) { }
			else if try_move(s, ent, face_dir.turn_left()) { }
			else if try_move(s, ent, face_dir.turn_around()) { }
			else { }
		}
	}
}

fn try_move(s: &mut GameState, ent: &mut Entity, move_dir: Dir) -> bool {
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
			EntityKind::Block => return false,
			_ => (),
		}
	}

	ent.face_dir = Some(move_dir);
	ent.step_dir = Some(move_dir);
	ent.step_time = s.time;
	ent.pos = new_pos;
	return true;
}

fn interact(_s: &mut GameState, _ent: &mut Entity, _ictx: &mut InteractContext) {
}

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
