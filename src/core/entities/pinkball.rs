use super::*;

pub fn create(s: &mut GameState, data: &SpawnData) -> EntityHandle {
	let handle = s.ents.alloc();
	s.ents.insert(Entity {
		funcs: &FUNCS,
		handle,
		kind: EntityKind::PinkBall,
		pos: data.pos,
		move_dir: None,
		move_spd: BASE_SPD,
		move_time: 0,
		face_dir: data.face_dir,
		trapped: false,
		remove: false,
	});
	return handle;
}

fn think(ent: &mut Entity, s: &mut GameState) {
	if ent.move_dir.is_some() && s.time >= ent.move_time + ent.move_spd {
		let terrain = s.field.get_terrain(ent.pos);

		if matches!(terrain, Terrain::GreenButton) {
			entities::press_green_button(s);
		}
		if matches!(terrain, Terrain::RedButton) {
			entities::press_red_button(s, ent.pos);
		}
		if matches!(terrain, Terrain::BrownButton) {
			entities::press_brown_button(s, ent.pos);
		}
		if matches!(terrain, Terrain::BlueButton) {
			entities::press_blue_button(s);
		}
		if matches!(terrain, Terrain::BearTrap) {
			ent.trapped = true;
		}

		ent.move_dir = None;
	}

	if !ent.trapped && s.time >= ent.move_time + ent.move_spd {
		if let Some(face_dir) = ent.face_dir {
			if try_move(ent, face_dir, s) { }
			else if try_move(ent, face_dir.turn_around(), s) { }
			else { }
		}
	}
}

fn try_move(ent: &mut Entity, move_dir: Dir, ctx: &mut GameState) -> bool {
	let flags = CanMoveFlags {
		gravel: false,
		fire: true,
	};
	if !ctx.field.can_move(ent.pos, move_dir, &flags) {
		return false;
	}

	let new_pos = ent.pos + move_dir.to_vec();
	for ent in ctx.ents.map.values() {
		if ent.pos != new_pos {
			continue;
		}
		match ent.kind {
			EntityKind::Socket => return false,
			EntityKind::Block => return false,
			_ => (),
		}
	}

	ent.face_dir = Some(move_dir);
	ent.move_dir = Some(move_dir);
	ent.move_time = ctx.time;
	ent.pos = new_pos;
	return true;
}

fn interact(_ent: &mut Entity, _s: &mut GameState, _ictx: &mut InteractContext) {
}

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
