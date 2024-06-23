use super::*;

pub fn create(game: &mut Game, x: i32, y: i32) {
	let entity_h = game.entities.alloc();
	let object_h = game.objects.alloc();
	game.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::Block,
		pos: Vec2(x, y),
		move_dir: None,
		move_spd: 0.125,
		face_dir: None,
		frozen: false,
		spawner_kind: None,
		move_time: 0.0,
	});
	game.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: EntityKind::Block,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: Sprite::Block,
		model: Model::Wall,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
	});
}

pub fn think(ent: &mut Entity, ctx: &mut ThinkContext) -> Lifecycle {
	if let Some(_) = ent.move_dir {
		if ctx.time >= ent.move_time + ent.move_spd {
			ent.move_dir = None;
			ent.face_dir = None;
			if ctx.field.get_tile(ent.pos).tile == Tile::Water {
				let dirt = ctx.field.lookup_tile(Tile::Dirt).unwrap();
				ctx.field.set_tile(ent.pos, dirt);
				return Lifecycle::Destroy;
			}
		}
	}
	return Lifecycle::KeepAlive;
}

fn is_solid_or_dirt(pos: Vec2<i32>, move_dir: Dir, field: &Field, entities: &EntityMap) -> bool {
	if !field.can_move(pos, move_dir) {
		return false;
	}

	let new_pos = pos + move_dir.to_vec();
	for ent in entities.map.values() {
		if ent.pos == new_pos {
			let solid = match ent.kind {
				EntityKind::Gate => true,
				EntityKind::Block => true,
				EntityKind::BlueDoor => true,
				EntityKind::RedDoor => true,
				EntityKind::GreenDoor => true,
				EntityKind::YellowDoor => true,
				_ => false,
			};
			if solid {
				return true;
			}
		}
	}
	false
}

pub fn interact(ent: &mut Entity, ctx: &mut ThinkContext, ictx: &mut InteractContext) {
	if ent.frozen {
		ictx.blocking = true;
		return;
	}

	let dirt = ctx.field.lookup_tile(Tile::Dirt);
	if dirt.is_none() || ctx.field.get_tile(ent.pos).tile == Tile::Water || is_solid_or_dirt(ent.pos, ictx.push_dir, &ctx.field, &ctx.entities) {
		ictx.blocking = true;
	}
	else {
		ictx.blocking = false;
		ent.pos += ictx.push_dir.to_vec();
		ent.move_dir = Some(ictx.push_dir);
		ent.face_dir = Some(ictx.push_dir);
		ent.move_time = ctx.time;
		// if ctx.field.get_tile(ent.pos).tile == Tile::Water {
		// 	ictx.remove_entity = true;
		// 	ctx.field.set_tile(ent.pos, dirt.unwrap());
		// }
	}
}

pub fn update(obj: &mut Object, ctx: &mut ThinkContext) {
	let ent = ctx.entities.get(obj.entity_handle);

	if let Some(ent) = ent {
		obj.pos = ent.pos.map(|c| c as f32 * 32.0).vec3(0.0);
		if let Some(move_dir) = ent.move_dir {
			let t = 1.0 - (ctx.time - ent.move_time) / ent.move_spd;
			obj.pos += (-move_dir.to_vec().map(|c| c as f32 * 32.0) * t).vec3(0.0);
		}
	}
	else {
		obj.live = false;
	}
}
