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
