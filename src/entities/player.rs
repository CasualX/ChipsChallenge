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
		spawner_kind: None,
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

	// Wait until movement is cleared before accepting new input
	if ent.move_dir.is_none() {
		let tile = ctx.field.get_tile(ent.pos).tile;

		// Turn dirt to floor after stepping on it
		let floor = ctx.field.lookup_tile(Tile::Floor).unwrap();
		if matches!(tile, Tile::Dirt) {
			ctx.field.set_tile(ent.pos, floor);
		}

		// Freeze the player when they reach the exit
		if matches!(tile, Tile::Exit) {
			ent.frozen = true;
		}

		// Set the player's move speed
		if ctx.pl.inv.suction_boots && matches!(tile, Tile::ForceLeft | Tile::ForceRight | Tile::ForceUp | Tile::ForceDown) {
			ent.move_spd = 0.125 + 0.125 * 0.5;
		}
		else {
			ent.move_spd = 0.125;
		}

		'end_move: {
			// First tick after stepping on a new tile
			if let Some(orig_dir) = orig_dir {

				// Handle switch tiles
				for other in ctx.entities.map.values_mut() {
					if matches!(tile, Tile::BlueSwitch) {
						if other.kind == EntityKind::EnemyTank {
							if let Some(face_dir) = other.face_dir {
								other.face_dir = Some(face_dir.turn_around());
							}
						}
					}
					else if matches!(tile, Tile::GreenSwitch) {
						if other.kind == EntityKind::Wall {
							if let Some(face_dir) = other.face_dir {
								other.face_dir = Some(face_dir.turn_around());
								other.move_time = ctx.time;
							}
						}
					}
				}

				// Handle ice physics
				if !ctx.pl.inv.ice_skates && matches!(tile, Tile::Ice | Tile::IceUL | Tile::IceUR | Tile::IceDL | Tile::IceDR) {
					let (ice_dir, back_dir) = match orig_dir {
						Dir::Up => match tile {
							Tile::IceUL => (Dir::Right, Dir::Down),
							Tile::IceUR => (Dir::Left, Dir::Down),
							Tile::IceDR => (Dir::Up, Dir::Left),
							Tile::IceDL => (Dir::Up, Dir::Right),
							_ => (orig_dir, orig_dir.turn_around()),
						},
						Dir::Left => match tile {
							Tile::IceUL => (Dir::Down, Dir::Right),
							Tile::IceUR => (Dir::Left, Dir::Down),
							Tile::IceDR => (Dir::Left, Dir::Up),
							Tile::IceDL => (Dir::Up, Dir::Right),
							_ => (orig_dir, orig_dir.turn_around()),
						},
						Dir::Down => match tile {
							Tile::IceUL => (Dir::Down, Dir::Right),
							Tile::IceUR => (Dir::Down, Dir::Left),
							Tile::IceDR => (Dir::Down, Dir::Up),
							Tile::IceDL => (Dir::Right, Dir::Up),
							_ => (orig_dir, orig_dir.turn_around()),
						},
						Dir::Right => match tile {
							Tile::IceUL => (Dir::Right, Dir::Down),
							Tile::IceUR => (Dir::Down, Dir::Left),
							Tile::IceDR => (Dir::Up, Dir::Left),
							Tile::IceDL => (Dir::Right, Dir::Up),
							_ => (orig_dir, orig_dir.turn_around()),
						},
					};
					// If the player is blocked, try to turn around
					if !try_move(ent, ice_dir, ctx) {
						if !try_move(ent, back_dir, ctx) {
							// Softlocked!
						}
					}
					break 'end_move;
				}
			}

			// Handle force tiles
			let force_dir = match tile {
				_ if ctx.pl.inv.suction_boots => None,
				Tile::ForceLeft => Some(Dir::Left),
				Tile::ForceRight => Some(Dir::Right),
				Tile::ForceUp => Some(Dir::Up),
				Tile::ForceDown => Some(Dir::Down),
				_ => None,
			};
			let first_time_force_dir = ctx.pl.inv.force_dir.is_none();
			ctx.pl.inv.force_dir = force_dir;
			if let Some(force_dir) = force_dir {

				let override_dir = match force_dir {
					_ if first_time_force_dir || ent.frozen => None,
					Dir::Left | Dir::Right => if ctx.input.up { Some(Dir::Up) } else if ctx.input.down { Some(Dir::Down) } else { None },
					Dir::Up | Dir::Down => if ctx.input.left { Some(Dir::Left) } else if ctx.input.right { Some(Dir::Right) } else { None },
				};

				match override_dir {
					Some(override_dir) if try_move(ent, override_dir, ctx) => true,
					_ => try_move(ent, force_dir, ctx),
				};

				break 'end_move;
			}

			// Handle player input
			if ent.frozen { }
			else if ctx.input.left && try_move(ent, Dir::Left, ctx) { }
			else if ctx.input.right && try_move(ent, Dir::Right, ctx) { }
			else if ctx.input.up && try_move(ent, Dir::Up, ctx) { }
			else if ctx.input.down && try_move(ent, Dir::Down, ctx) { }
		}
	}

	Lifecycle::KeepAlive
}

fn try_move(ent: &mut Entity, move_dir: Dir, ctx: &mut ThinkContext) -> bool {
	let new_pos = ent.pos + move_dir.to_vec();

	let mut success = ctx.field.can_move(ent.pos, move_dir);
	if success {
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
			if ictx.blocking {
				success = false;
			}
		}
	}

	ent.face_dir = Some(move_dir);
	ent.move_time = ctx.time;
	if success {
		ent.move_dir = Some(move_dir);
		ent.pos = new_pos;
		ctx.pl.inv.steps += 1;
	}

	return success;
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
