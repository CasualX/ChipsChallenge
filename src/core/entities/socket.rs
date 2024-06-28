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
	if s.ps.chips >= s.field.chips {
		ent.remove = true;
		ictx.blocking = false;
		s.events.push(GameEvent::SocketFilled { pos: ent.pos });
	}
	else {
		ictx.blocking = true;
	}
}

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
