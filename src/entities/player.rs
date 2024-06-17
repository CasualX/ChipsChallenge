use super::*;

const MOVE_SPEED: f32 = 0.125;
const IDLE_TIME: f32 = 0.2;

pub fn create(game: &mut Game, x: i32, y: i32) {
	game.entities.insert(Entity {
		handle: game.pl.entity,
		kind: EntityKind::Player,
		pos: Vec2(x, y),
		move_dir: None,
		face_dir: None,
		frozen: false,
		move_time: 0.0,
	});
	game.objects.insert(Object {
		handle: game.pl.object,
		entity_handle: game.pl.entity,
		entity_kind: EntityKind::Player,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: Sprite::PlayerWalkNeutral,
		model: Model::Sprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		live: true,
	});
}

pub fn think(ent: &mut Entity, ctx: &mut ThinkContext) -> Lifecycle {
	// Clear movement after a delay
	if ctx.time >= ent.move_time + IDLE_TIME {
		ent.face_dir = None;
	}
	if ctx.time >= ent.move_time + MOVE_SPEED {
		ent.move_dir = None;
	}

	let tile = ctx.field.get_tile(ent.pos).tile;

	// Turn dirt to floor after stepping on it
	let floor = ctx.field.lookup_tile(Tile::Floor).unwrap();
	if ent.move_dir.is_none() && tile == Tile::Dirt {
		ctx.field.set_tile(ent.pos, floor);
	}

	// Freeze the player when they reach the exit
	if tile == Tile::Exit {
		ent.frozen = true;
	}

	if ent.move_dir.is_none() {
		let mut move_dir = None;
		if ctx.input.left {
			move_dir = Some(Dir::Left);
		}
		else if ctx.input.right {
			move_dir = Some(Dir::Right);
		}
		else if ctx.input.up {
			move_dir = Some(Dir::Up);
		}
		else if ctx.input.down {
			move_dir = Some(Dir::Down);
		}
		if ent.frozen {
			move_dir = None;
		}

		if let Some(move_dir) = move_dir {
			let new_pos = ent.pos + move_dir.to_vec();
			let mut blocking = ctx.field.get_tile(new_pos).solid;
			if !blocking {
				for handle in ctx.entities.map.keys().cloned().collect::<Vec<_>>() {
					let Some(mut ent) = ctx.entities.remove(handle) else { continue };
					let mut ictx = InteractContext {
						remove_entity: false,
						blocking: false,
						push_dir: move_dir,
					};
					if ent.pos == new_pos {
						ent.interact(ctx, &mut ictx);
					}
					if !ictx.remove_entity {
						ctx.entities.insert(ent);
					}
					blocking |= ictx.blocking;
				}
			}

			ent.face_dir = Some(move_dir);
			ent.move_time = ctx.time;
			if !blocking {
				ent.move_dir = Some(move_dir);
				ent.pos += move_dir.to_vec();
				ctx.pl.inv.steps += 1;
			}
		}
	}

	Lifecycle::KeepAlive
}

pub fn interact(_ent: &mut Entity, _ctx: &mut ThinkContext, ictx: &mut InteractContext) {
	ictx.blocking = false;
}

pub fn update(obj: &mut Object, ctx: &mut ThinkContext) {
	let Some(ent) = ctx.entities.get(obj.entity_handle) else { return };

	let tile = ctx.field.get_tile(ent.pos).tile;

	if ent.move_dir.is_none() && tile == Tile::Exit {
		obj.sprite = Sprite::PlayerCheer;
	}
	else if ctx.time > ent.move_time + MOVE_SPEED + IDLE_TIME {
		obj.sprite = Sprite::PlayerWalkNeutral
	}
	else {
		match ent.face_dir {
			Some(Dir::Up) => obj.sprite = Sprite::PlayerWalkUp,
			Some(Dir::Left) => obj.sprite = Sprite::PlayerWalkLeft,
			Some(Dir::Down) => obj.sprite = Sprite::PlayerWalkDown,
			Some(Dir::Right) => obj.sprite = Sprite::PlayerWalkRight,
			None => (),
		}
	}

	obj.pos = ent.pos.map(|c| c as f32 * 32.0).vec3(0.0);
	if let Some(move_dir) = ent.move_dir {
		let t = 1.0 - (ctx.time - ent.move_time) / MOVE_SPEED;
		obj.pos += (-move_dir.to_vec().map(|c| c as f32 * 32.0) * t).vec3(0.0);
	}
}
