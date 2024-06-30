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
		step_spd: BASE_SPD,
		step_time: 0,
		trapped: false,
		hidden: false,
		remove: false,
	});
	return handle;
}

fn think(s: &mut GameState, ent: &mut Entity) {
	let terrain = s.field.get_terrain(ent.pos);

	if bomb::check(ent, s) {
		return;
	}

	if ent.step_dir.is_some() && s.time >= ent.step_time + ent.step_spd {
		if matches!(terrain, Terrain::Water) {
			s.field.set_terrain(ent.pos, Terrain::Dirt);
			ent.remove = true;
		}

		ent.step_dir = None;
	}
}

static FUNCS: EntityFuncs = EntityFuncs { think };
