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
		step_spd: 0,
		step_time: 0,
		trapped: false,
		remove: false,
	});
	return handle;
}

fn think(_s: &mut GameState, _ent: &mut Entity) {
}

fn interact(s: &mut GameState, ent: &mut Entity, ictx: &mut InteractContext) {
	ictx.blocking = false;
	s.ps.flippers = false;
	s.ps.fire_boots = false;
	s.ps.ice_skates = false;
	s.ps.suction_boots = false;
	s.events.push(GameEvent::AllItemsCleared { handle: ent.handle });
}

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
