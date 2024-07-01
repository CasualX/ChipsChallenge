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

fn think(s: &mut GameState, ent: &mut Entity) {
	if s.ents.get(s.ps.entity).map(|e| e.pos) == Some(ent.pos) {
		ps_action(s, PlayerAction::Death);
	}

	let mut exploded = false;
	for other_ent in s.ents.map.values_mut() {
		if other_ent.handle == ent.handle {
			continue;
		}
		if other_ent.pos != ent.pos {
			continue;
		}
		// HACK! Delay expolosion by 1 tick to work around animation bug
		if other_ent.step_time >= s.time {
			return;
		}
		exploded = true;
		if matches!(other_ent.kind, EntityKind::Player) {
			continue;
		}
		other_ent.remove = true;
	}

	if exploded {
		ent.remove = true;
		s.events.push(GameEvent::BombExplode { entity: ent.handle });
	}
}

static FUNCS: EntityFuncs = EntityFuncs { think };
