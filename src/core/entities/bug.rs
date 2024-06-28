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
	if s.time >= ent.step_time + ent.step_spd {
		ent.step_dir = None;
	}

	if s.time >= ent.step_time + ent.step_spd {
		if let Some(face_dir) = ent.face_dir {
			// If bug can turn left, turn left
			if try_move(s, ent, face_dir.turn_left()) { }
			// Otherwise try to move forward
			else if try_move(s, ent, face_dir) { }
			// If forward is blocked, try to turn right
			else if try_move(s, ent, face_dir.turn_right()) { }
			// At this point, can't turn left, can't go forward, can't turn right so try to turn around
			else if try_move(s, ent, face_dir.turn_around()) { }
			// Trapped! Wait until freed
			else { }
		}
	}
}

fn try_move(s: &mut GameState, ent: &mut Entity, move_dir: Dir) -> bool {
	let flags = CanMoveFlags {
		gravel: false,
		fire: false,
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
			EntityKind::Chip => return false,
			_ => (),
		}
	}

	ent.face_dir = Some(move_dir);
	ent.step_dir = Some(move_dir);
	ent.step_time = s.time;
	ent.pos = new_pos;
	s.events.push(GameEvent::EntityStep { handle: ent.handle });
	s.events.push(GameEvent::EntityFaceDir { handle: ent.handle });
	return true;
}

fn interact(_s: &mut GameState, _ent: &mut Entity, _ictx: &mut InteractContext) {
}

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
