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

fn interact(ent: &mut Entity, ctx: &mut GameState, ictx: &mut InteractContext) {
	match ent.kind {
		EntityKind::Chip => ctx.ps.chips += 1,
		EntityKind::BlueKey => ctx.ps.keys[0] += 1,
		EntityKind::RedKey => ctx.ps.keys[1] += 1,
		EntityKind::GreenKey => ctx.ps.keys[2] += 1,
		EntityKind::YellowKey => ctx.ps.keys[3] += 1,
		EntityKind::Flippers => ctx.ps.flippers = true,
		EntityKind::FireBoots => ctx.ps.fire_boots = true,
		EntityKind::IceSkates => ctx.ps.ice_skates = true,
		EntityKind::SuctionBoots => ctx.ps.suction_boots = true,
		_ => (),
	}

	ent.remove = true;
	ictx.blocking = false;
}

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
