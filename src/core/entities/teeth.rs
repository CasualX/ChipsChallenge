use super::*;

pub fn create(s: &mut GameState, args: &EntityArgs) -> EntityHandle {
	let handle = s.ents.alloc();
	s.ents.insert(Entity {
		funcs: &FUNCS,
		handle,
		kind: args.kind,
		pos: args.pos,
		face_dir: args.face_dir,
		step_dir: None,
		step_spd: BASE_SPD * 2,
		step_time: 0,
		trapped: false,
		hidden: false,
		remove: false,
	});
	return handle;
}

fn think(s: &mut GameState, ent: &mut Entity) {
	if s.ents.get(s.ps.entity).map(|e| e.pos) == Some(ent.pos) {
		ps_action(s, PlayerAction::Death);
	}

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

	if ent.trapped || ent.hidden {
		return;
	}
	if s.time >= ent.step_time + ent.step_spd {
		if let Some((first_dir, second_dir)) = chase_dirs(s, ent) {
			if try_move(s, ent, first_dir) { }
			else if try_move(s, ent, second_dir) { }
			// If no legal move, stay put and face in the first direction
			else {
				if ent.face_dir != Some(first_dir) {
					s.events.push(GameEvent::EntityFaceDir { entity: ent.handle });
				}
				ent.face_dir = Some(first_dir);
			}
		}
	}
}

fn chase_dirs(s: &GameState, ent: &Entity) -> Option<(Dir, Dir)> {
	let pl = s.ents.get(s.ps.entity)?;
	let d = pl.pos - ent.pos;

	// Teeth moves either vertically or horizontally toward Chip one square at a time, always taking the longer path, and vertically if tied.
	// However, if this move would be illegal because of some obstacle, it will go the other way if that is a legal move, and if not,
	// it will stay put until Chip moves somewhere that allows it to make another move.

	if d.y == 0 {
		if d.x > 0 {
			Some((Dir::Right, Dir::Right))
		}
		else if d.x < 0 {
			Some((Dir::Left, Dir::Left))
		}
		else {
			None
		}
	}
	else if d.y > 0 {
		if d.x > d.y {
			Some((Dir::Right, Dir::Down))
		}
		else if d.x > 0 {
			Some((Dir::Down, Dir::Right))
		}
		else if d.x == 0 {
			Some((Dir::Down, Dir::Down))
		}
		else if d.x < d.y {
			Some((Dir::Left, Dir::Down))
		}
		else {
			Some((Dir::Down, Dir::Left))
		}
	}
	else/* if d.y < 0*/ {
		if d.x > -d.y {
			Some((Dir::Right, Dir::Up))
		}
		else if d.x > 0 {
			Some((Dir::Up, Dir::Right))
		}
		else if d.x == 0 {
			Some((Dir::Up, Dir::Up))
		}
		else if d.x < -d.y {
			Some((Dir::Left, Dir::Up))
		}
		else {
			Some((Dir::Up, Dir::Left))
		}
	}
}

fn try_move(s: &mut GameState, ent: &mut Entity, move_dir: Dir) -> bool {
	let flags = CanMoveFlags {
		gravel: false,
		fire: false,
		dirt: false,
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

	s.events.push(GameEvent::EntityFaceDir { entity: ent.handle });
	s.events.push(GameEvent::EntityStep { entity: ent.handle });
	ent.face_dir = Some(move_dir);
	ent.step_dir = Some(move_dir);
	ent.step_time = s.time;
	ent.pos = new_pos;
	return true;
}

static FUNCS: EntityFuncs = EntityFuncs { think };
