use super::*;

pub fn create(s: &mut GameState, data: &SpawnData) -> EntityHandle {
	let handle = s.ents.alloc();
	s.ents.insert(Entity {
		funcs: &FUNCS,
		handle,
		kind: data.kind,
		pos: data.pos,
		face_dir: data.face_dir,
		move_dir: None,
		move_spd: BASE_SPD,
		move_time: 0,
		trapped: false,
		destroy: false,
	});
	return handle;
}

pub fn think(_s: &mut GameState, _ent: &mut Entity) {
}

pub fn interact(_s: &mut GameState, ent: &mut Entity, ictx: &mut InteractContext) {
	ictx.blocking = ent.face_dir == Some(Dir::Up);
	return;
}

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
