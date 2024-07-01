use super::*;

pub fn try_move(s: &mut GameState, ent: &mut Entity, move_dir: Dir) -> bool {
	if ent.speed == 0 {
		return false;
	}

	let flags = CanMoveFlags {
		gravel: false,
		fire: matches!(ent.kind, EntityKind::FireBall),
		dirt: false,
		exit: false,
	};
	if !s.field.can_move(ent.pos, move_dir, &flags) {
		return false;
	}

	let new_pos = ent.pos + move_dir.to_vec();
	for ent in s.ents.map.values() {
		if ent.pos != new_pos {
			continue;
		}
		if ekind_is_solid(ent.kind) {
			return false;
		}
	}

	// Set the player's move speed
	let terrain = s.field.get_terrain(ent.pos);
	if matches!(terrain, Terrain::ForceW | Terrain::ForceE | Terrain::ForceN | Terrain::ForceS) {
		ent.step_spd = ent.speed / 2;
	}
	else if matches!(terrain, Terrain::Ice | Terrain::IceNE | Terrain::IceSE | Terrain::IceNW | Terrain::IceSW) {
		ent.step_spd = ent.speed / 2;
	}
	else {
		ent.step_spd = ent.speed;
	}

	ent.face_dir = Some(move_dir);
	ent.step_dir = Some(move_dir);
	ent.step_time = s.time;
	ent.pos = new_pos;
	ent.has_moved = true;
	s.events.push(GameEvent::EntityFaceDir { entity: ent.handle });
	s.events.push(GameEvent::EntityStep { entity: ent.handle });
	return true;
}

fn ekind_is_solid(kind: EntityKind) -> bool {
	match kind {
		EntityKind::Player => false,
		EntityKind::Chip => true,
		EntityKind::Socket => true,
		EntityKind::Block => true,
		EntityKind::Flippers => true,
		EntityKind::FireBoots => true,
		EntityKind::IceSkates => true,
		EntityKind::SuctionBoots => true,
		EntityKind::BlueKey => true,
		EntityKind::RedKey => true,
		EntityKind::GreenKey => true,
		EntityKind::YellowKey => true,
		EntityKind::Thief => true,
		EntityKind::Bomb => false,
		EntityKind::Bug => true,
		EntityKind::FireBall => true,
		EntityKind::PinkBall => true,
		EntityKind::Tank => true,
		EntityKind::Glider => true,
		EntityKind::Teeth => true,
		EntityKind::Walker => true,
		EntityKind::Blob => true,
		EntityKind::Paramecium => true,
	}
}

pub fn ice_dir(terrain: Terrain, dir: Dir) -> Option<(Dir, Dir)> {
	let x = match dir {
		Dir::Up => match terrain {
			Terrain::IceNW => (Dir::Right, Dir::Down),
			Terrain::IceNE => (Dir::Left, Dir::Down),
			Terrain::IceSE => (Dir::Up, Dir::Left),
			Terrain::IceSW => (Dir::Up, Dir::Right),
			Terrain::Ice => (dir, dir.turn_around()),
			_ => return None,
		},
		Dir::Left => match terrain {
			Terrain::IceNW => (Dir::Down, Dir::Right),
			Terrain::IceNE => (Dir::Left, Dir::Down),
			Terrain::IceSE => (Dir::Left, Dir::Up),
			Terrain::IceSW => (Dir::Up, Dir::Right),
			Terrain::Ice => (dir, dir.turn_around()),
			_ => return None,
		},
		Dir::Down => match terrain {
			Terrain::IceNW => (Dir::Down, Dir::Right),
			Terrain::IceNE => (Dir::Down, Dir::Left),
			Terrain::IceSE => (Dir::Left, Dir::Up),
			Terrain::IceSW => (Dir::Right, Dir::Up),
			Terrain::Ice => (dir, dir.turn_around()),
			_ => return None,
		},
		Dir::Right => match terrain {
			Terrain::IceNW => (Dir::Right, Dir::Down),
			Terrain::IceNE => (Dir::Down, Dir::Left),
			Terrain::IceSE => (Dir::Up, Dir::Left),
			Terrain::IceSW => (Dir::Right, Dir::Up),
			Terrain::Ice => (dir, dir.turn_around()),
			_ => return None,
		},
	};
	Some(x)
}