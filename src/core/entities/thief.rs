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

fn interact(ent: &mut Entity, s: &mut GameState, ictx: &mut InteractContext) {
	ictx.blocking = false;
	s.ps.flippers = false;
	s.ps.fire_boots = false;
	s.ps.ice_skates = false;
	s.ps.suction_boots = false;
	s.events.push(GameEvent::AllItemsCleared { handle: ent.handle });
}

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
