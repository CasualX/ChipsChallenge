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
		step_spd: 0,
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
}

pub fn check(ent: &mut Entity, s: &mut GameState) -> bool {
	for other_ent in s.ents.map.values_mut() {
		if other_ent.pos != ent.pos {
			continue;
		}
		if matches!(other_ent.kind, EntityKind::Bomb) {
			other_ent.remove = true;
			ent.remove = true;
			return true;
		}
	}
	return false;
}

static FUNCS: EntityFuncs = EntityFuncs { think };
