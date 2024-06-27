use super::*;

static FUNCS: EntityFuncs = EntityFuncs { think, interact };

pub fn create(s: &mut GameState, data: &SpawnData) -> EntityHandle {
	let handle = s.ents.alloc();
	s.ents.insert(Entity {
		funcs: &FUNCS,
		handle,
		kind: data.kind,
		pos: data.pos,
		move_dir: None,
		move_spd: BASE_SPD,
		move_time: 0,
		face_dir: data.face_dir,
		trapped: false,
		destroy: false,
	});
	return handle;
}

pub fn think(_ent: &mut Entity, _s: &mut GameState) {
}

pub fn interact(ent: &mut Entity, _s: &mut GameState, ictx: &mut InteractContext) {
	ictx.blocking = ent.face_dir == Some(Dir::Up);
	return;
}
