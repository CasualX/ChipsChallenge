use super::*;

pub fn create(s: &mut GameState, data: &SpawnData) -> EntityHandle {
	let handle = s.ents.alloc();
	s.ents.insert(Entity {
		funcs: &FUNCS,
		handle,
		kind: data.kind,
		pos: data.pos,
		move_dir: None,
		move_spd: 0,
		move_time: 0,
		face_dir: data.face_dir,
		trapped: false,
		remove: false,
	});
	return handle;
}

fn think(_ent: &mut Entity, _s: &mut GameState) {
}

fn interact(_ent: &mut Entity, _s: &mut GameState, _ictx: &mut InteractContext) {
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

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
