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
	match ent.kind {
		EntityKind::Chip => s.ps.chips += 1,
		EntityKind::BlueKey => s.ps.keys[KeyColor::Blue as usize] += 1,
		EntityKind::RedKey => s.ps.keys[KeyColor::Red as usize] += 1,
		EntityKind::GreenKey => s.ps.keys[KeyColor::Green as usize] += 1,
		EntityKind::YellowKey => s.ps.keys[KeyColor::Yellow as usize] += 1,
		EntityKind::Flippers => s.ps.flippers = true,
		EntityKind::FireBoots => s.ps.fire_boots = true,
		EntityKind::IceSkates => s.ps.ice_skates = true,
		EntityKind::SuctionBoots => s.ps.suction_boots = true,
		_ => (),
	}

	ent.remove = true;
	ictx.blocking = false;
}

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
