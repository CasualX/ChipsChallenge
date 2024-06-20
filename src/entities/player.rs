use super::*;

const IDLE_TIME: f32 = 0.2;

pub fn create(game: &mut Game, x: i32, y: i32) {
	let entity_h = game.entities.alloc();
	let object_h = game.objects.alloc();
	game.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::Player,
		pos: Vec2(x, y),
		move_dir: None,
		move_spd: 0.125,
		face_dir: None,
		frozen: false,
		move_time: 0.0,
	});
	game.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: EntityKind::Player,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite: Sprite::PlayerWalkNeutral,
		model: Model::Sprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
	});
}

pub fn think(ent: &mut Entity, ctx: &mut ThinkContext) -> Lifecycle {
	let orig_dir = ent.move_dir;

	// Clear movement after a delay
	if ctx.time >= ent.move_time + IDLE_TIME {
		ent.face_dir = None;
	}
	if ctx.time >= ent.move_time + ent.move_spd {
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

	if orig_dir.is_some() && ent.move_dir.is_none() {
		let tile = ctx.field.get_tile(ent.pos).tile;

		if tile == Tile::BlueSwitch {
			for other in ctx.entities.map.values_mut() {
				if other.kind == EntityKind::EnemyTank {
					if let Some(face_dir) = other.face_dir {
						other.face_dir = Some(face_dir.turn_around());
					}
				}
			}
		}
	}

	if ent.move_dir.is_none() {
		let (mut allow_left, mut allow_right, mut allow_up, mut allow_down) = (true, true, true, true);

		// Force movement when stepping on a force tile
		let mut force_dir = None;
		if !ctx.pl.inv.suction_boots {
			if tile == Tile::ForceLeft {
				force_dir = Some(Dir::Left);
			}
			else if tile == Tile::ForceRight {
				force_dir = Some(Dir::Right);
			}
			else if tile == Tile::ForceUp {
				force_dir = Some(Dir::Up);
			}
			else if tile == Tile::ForceDown {
				force_dir = Some(Dir::Down);
			}
			if let Some(force_dir) = force_dir {
				match force_dir {
					Dir::Left | Dir::Right => {
						allow_left = false;
						allow_right = false;
						allow_up = !ctx.field.get_tile(ent.pos + Dir::Up.to_vec()).solid;
						allow_down = !ctx.field.get_tile(ent.pos + Dir::Down.to_vec()).solid;
					},
					Dir::Up | Dir::Down => {
						allow_left = !ctx.field.get_tile(ent.pos + Dir::Left.to_vec()).solid;
						allow_right = !ctx.field.get_tile(ent.pos + Dir::Right.to_vec()).solid;
						allow_up = false;
						allow_down = false;
					},
				}
				// If the player just stepped on a force tile, they cannot override it
				if ctx.pl.inv.force_dir.is_none() {
					allow_left = false;
					allow_right = false;
					allow_up = false;
					allow_down = false;
				}
			}
		}
		if ctx.pl.inv.suction_boots && matches!(tile, Tile::ForceLeft | Tile::ForceRight | Tile::ForceUp | Tile::ForceDown) {
			ent.move_spd = 0.125 + 0.125 * 0.5;
		}
		else {
			ent.move_spd = 0.125;
		}

		let mut ice_dir = None;
		if !ctx.pl.inv.ice_skates {
			if let Some(orig_dir) = orig_dir {
				ice_dir = match tile {
					Tile::IceUL => match orig_dir {
						Dir::Up => Some(Dir::Right),
						Dir::Left => Some(Dir::Down),
						_ => Some(orig_dir),
					},
					Tile::IceUR => match orig_dir {
						Dir::Up => Some(Dir::Left),
						Dir::Right => Some(Dir::Down),
						_ => Some(orig_dir),
					},
					Tile::IceDL => match orig_dir {
						Dir::Down => Some(Dir::Right),
						Dir::Left => Some(Dir::Up),
						_ => Some(orig_dir),
					},
					Tile::IceDR => match orig_dir {
						Dir::Down => Some(Dir::Left),
						Dir::Right => Some(Dir::Up),
						_ => Some(orig_dir),
					},
					Tile::Ice => Some(orig_dir),
					_ => None,
				};
				if ice_dir.is_some() {
					allow_left = false;
					allow_right = false;
					allow_up = false;
					allow_down = false;
				}
				force_dir = ice_dir;
			}
		}

		let mut move_dir = force_dir;
		if ctx.input.left && allow_left {
			move_dir = Some(Dir::Left);
		}
		else if ctx.input.right && allow_right {
			move_dir = Some(Dir::Right);
		}
		else if ctx.input.up && allow_up {
			move_dir = Some(Dir::Up);
		}
		else if ctx.input.down && allow_down {
			move_dir = Some(Dir::Down);
		}

		if ent.frozen {
			move_dir = None;
		}

		if let Some(mut move_dir) = move_dir {
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

			// Very questionable logic...
			if let Some(ice_dir) = ice_dir {
				if blocking {
					move_dir = ice_dir.turn_around();
					blocking = false;
				}
			}

			ent.face_dir = Some(move_dir);
			ent.move_time = ctx.time;
			if !blocking {
				ent.move_dir = Some(move_dir);
				ent.pos += move_dir.to_vec();
				ctx.pl.inv.steps += 1;
				ctx.pl.inv.force_dir = force_dir;
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
	else if ctx.time > ent.move_time + ent.move_spd + IDLE_TIME {
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

	if tile == Tile::Water {
		obj.sprite = match obj.sprite {
			Sprite::PlayerWalkNeutral => Sprite::PlayerSwimNeutral,
			Sprite::PlayerWalkUp => Sprite::PlayerSwimUp,
			Sprite::PlayerWalkLeft => Sprite::PlayerSwimLeft,
			Sprite::PlayerWalkDown => Sprite::PlayerSwimDown,
			Sprite::PlayerWalkRight => Sprite::PlayerSwimRight,
			_ => obj.sprite,
		};
	}

	obj.pos = ent.pos.map(|c| c as f32 * 32.0).vec3(0.0);
	if let Some(move_dir) = ent.move_dir {
		let t = 1.0 - (ctx.time - ent.move_time) / ent.move_spd;
		obj.pos += (-move_dir.to_vec().map(|c| c as f32 * 32.0) * t).vec3(0.0);
	}
}
