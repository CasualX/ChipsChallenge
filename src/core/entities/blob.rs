use super::*;

pub fn create(s: &mut GameState, args: &EntityArgs) -> EntityHandle {
	let handle = s.ents.alloc();
	s.ents.insert(Entity {
		funcs: &FUNCS,
		handle,
		kind: args.kind,
		pos: args.pos,
		speed: BASE_SPD * 2,
		face_dir: args.face_dir,
		step_dir: None,
		step_spd: BASE_SPD * 2,
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
		if let Some(&move_dir) = s.rand.rng.choose(&[Dir::Up, Dir::Down, Dir::Left, Dir::Right]) {
			// The direction of the blob means nothing, it is completely random
			try_move(s, ent, move_dir);
		}
	}
}

static FUNCS: EntityFuncs = EntityFuncs { think };
