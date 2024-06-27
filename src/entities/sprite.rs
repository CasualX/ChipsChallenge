use super::*;

pub fn think(ent: &mut Entity, ctx: &mut ThinkContext) {
}

pub fn interact(_ent: &mut Entity, _ctx: &mut ThinkContext, _ictx: &mut InteractContext) {
}

pub fn update(obj: &mut Object, ctx: &mut ThinkContext) {
	// let ent = ctx.entities.get(obj.entity_handle);

	// if let Some(ent) = ent {
	// 	obj.pos = ent.pos.map(|c| c as f32 * 32.0).vec3(0.0);
	// 	if let Some(move_dir) = ent.move_dir {
	// 		let t = 1.0 - (ctx.time - ent.move_time) / ent.move_spd;
	// 		obj.pos += (-move_dir.to_vec().map(|c| c as f32 * 32.0) * t).vec3(0.0);
	// 	}
	// }
	// else {
	// 	obj.live = false;
	// }

	if obj.pos.z < -20.0 {
		obj.live = false;
	}
}
