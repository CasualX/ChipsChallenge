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
	if ent.step_dir.is_some() && s.time >= ent.step_time + ent.step_spd {
		// Check for traps
		let terrain = s.field.get_terrain(ent.pos);
		if matches!(terrain, Terrain::BearTrap) {
			ent.trapped = true;
		}

		if bomb::check(ent, s) {
			return;
		}

		ent.step_dir = None;
	}

	if !ent.trapped && s.time >= ent.step_time + ent.step_spd {
		if let Some(face_dir) = ent.face_dir {
			// Try to move forward
			if try_move(s, ent, face_dir) { }
			// If it can turn left, turn left
			else if try_move(s, ent, face_dir.turn_left()) { }
			// If it can turn right, turn right
			else if try_move(s, ent, face_dir.turn_right()) { }
			// Try to turn around
			else if try_move(s, ent, face_dir.turn_around()) { }
			// Trapped! Wait until freed
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

	s.events.push(GameEvent::EntityFaceDir { handle: ent.handle });
	s.events.push(GameEvent::EntityStep { handle: ent.handle });
	ent.face_dir = Some(move_dir);
	ent.step_dir = Some(move_dir);
	ent.step_time = s.time;
	ent.pos = new_pos;
	return true;
}

fn interact(_s: &mut GameState, _ent: &mut Entity, _ictx: &mut InteractContext) {
}

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
