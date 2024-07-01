use super::*;

pub fn create(s: &mut GameState, args: &EntityArgs) -> EntityHandle {
	let handle = s.ents.alloc();
	s.ents.insert(Entity {
		funcs: &FUNCS,
		handle,
		kind: args.kind,
		pos: args.pos,
		speed: BASE_SPD,
		face_dir: args.face_dir,
		step_dir: None,
		step_spd: BASE_SPD,
		step_time: 0,
		trapped: false,
		hidden: false,
		has_moved: false,
		remove: false,
	});
	return handle;
}

use creature::try_move;

fn think(s: &mut GameState, ent: &mut Entity) {
	let terrain = s.field.get_terrain(ent.pos);

	if matches!(terrain, Terrain::Water) {
		s.field.set_terrain(ent.pos, Terrain::Dirt);
		ent.remove = true;
	}


	if ent.step_dir.is_some() && s.time >= ent.step_time + ent.step_spd {
		let step_dir = ent.step_dir.unwrap();
		if let Some((ice_dir, back_dir)) = creature::ice_dir(terrain, step_dir) {
			if try_move(s, ent, ice_dir) { }
			else if try_move(s, ent, back_dir) { }
			else {
				ent.step_dir = None;
			}
		}
		else {
			ent.step_dir = None;
		}
	}
}

static FUNCS: EntityFuncs = EntityFuncs { think };
