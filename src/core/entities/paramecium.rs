use super::*;

pub fn create(s: &mut GameState, args: &EntityArgs) -> EntityHandle {
	let handle = s.ents.alloc();
	s.ents.insert(Entity {
		funcs: &FUNCS,
		handle,
		kind: args.kind,
		pos: args.pos,
		speed: BASE_SPD,
		face_dir: args.face_dir,
		step_dir: None,
		step_spd: BASE_SPD,
		step_time: 0,
		trapped: false,
		hidden: false,
		has_moved: false,
		remove: false,
	});
	return handle;
}

use creature::try_move;

fn think(s: &mut GameState, ent: &mut Entity) {
	if s.ents.get(s.ps.entity).map(|e| e.pos) == Some(ent.pos) {
		ps_action(s, PlayerAction::Death);
	}

	if ent.step_dir.is_some() && s.time >= ent.step_time + ent.step_spd {
		ent.step_dir = None;
	}

	if ent.trapped || ent.hidden {
		return;
	}
	if s.time >= ent.step_time + ent.step_spd {
		if let Some(face_dir) = ent.face_dir {
			// If paramecium can turn right, turn right
			if try_move(s, ent, face_dir.turn_right()) { }
			// Otherwise try to move forward
			else if try_move(s, ent, face_dir) { }
			// If forward is blocked, try to turn left
			else if try_move(s, ent, face_dir.turn_left()) { }
			// At this point, can't turn right, can't go forward, can't turn left so try to turn around
			else if try_move(s, ent, face_dir.turn_around()) { }
			// Trapped! Wait until freed
			else { }
		}
	}
}

static FUNCS: EntityFuncs = EntityFuncs { think };
