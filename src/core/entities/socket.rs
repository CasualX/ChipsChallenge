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

fn think(_s: &mut GameState, _ent: &mut Entity) {
}

static FUNCS: EntityFuncs = EntityFuncs { think };
