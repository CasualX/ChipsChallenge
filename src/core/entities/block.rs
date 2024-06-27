use super::*;

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
		remove: false,
	});
	return handle;
}

fn think(ent: &mut Entity, s: &mut GameState) {
	let terrain = s.field.get_terrain(ent.pos);

	if ent.move_dir.is_some() && s.time >= ent.move_time + ent.move_spd {
		if bomb::check(ent, s) {
			return;
		}

		if matches!(terrain, Terrain::Water) {
			s.field.set_terrain(ent.pos, Terrain::Dirt);
			ent.remove = true;
		}
		if matches!(terrain, Terrain::BrownButton) {
			entities::press_brown_button(s, ent.pos);
		}

		ent.move_dir = None;
	}
}

fn is_solid_or_dirt(pos: Vec2i, move_dir: Dir, field: &Field, entities: &EntityMap) -> bool {
	let flags = CanMoveFlags {
		gravel: true,
		fire: true,
	};
	if !field.can_move(pos, move_dir, &flags) {
		return true;
	}

	let new_pos = pos + move_dir.to_vec();
	for ent in entities.map.values() {
		if ent.pos == new_pos {
			let solid = match ent.kind {
				EntityKind::Socket => true,
				EntityKind::Block => true,
				_ => false,
			};
			if solid {
				return true;
			}
		}
	}
	false
}

fn interact(ent: &mut Entity, s: &mut GameState, ictx: &mut InteractContext) {
	if ent.trapped {
		ictx.blocking = true;
		return;
	}

	if s.field.get_terrain(ent.pos) == Terrain::Water || is_solid_or_dirt(ent.pos, ictx.push_dir, &s.field, &s.ents) {
		ictx.blocking = true;
	}
	else {
		ictx.blocking = false;
		ent.pos += ictx.push_dir.to_vec();
		ent.move_dir = Some(ictx.push_dir);
		ent.face_dir = Some(ictx.push_dir);
		ent.move_time = s.time;
		// if ctx.field.get_tile(ent.pos).tile == Tile::Water {
		// 	ictx.remove_entity = true;
		// 	ctx.field.set_tile(ent.pos, dirt.unwrap());
		// }

		if bomb::check(ent, s) {
			return;
		}
		let terrain = s.field.get_terrain(ent.pos);
		if matches!(terrain, Terrain::BearTrap) {
			ent.trapped = true;
		}
	}
}

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
