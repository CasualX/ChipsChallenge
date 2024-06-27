use super::*;

const IDLE_TIME: f32 = 0.2;

pub fn create(ctx: &mut SpawnContext, x: i32, y: i32) -> EntityHandle {
	let entity_h = ctx.entities.alloc();
	let object_h = ctx.objects.alloc();
	ctx.entities.insert(Entity {
		handle: entity_h,
		kind: EntityKind::Player,
		pos: Vec2(x, y),
		move_dir: None,
		move_spd: BASE_SPD,
		move_time: 0.0,
		face_dir: None,
		trapped: false,
		destroy: false,
	});
	ctx.objects.insert(Object {
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
	entity_h
}

pub fn think(ent: &mut Entity, ctx: &mut ThinkContext) {
	let terrain = ctx.field.get_terrain(ent.pos);
	let orig_dir = ent.move_dir;

	// Clear movement after a delay
	if ctx.time >= ent.move_time + IDLE_TIME {
		ent.face_dir = None;
	}
	if ctx.time >= ent.move_time + ent.move_spd {
		ent.move_dir = None;
	}

	// Turn BlueFake to floor when stepping through it
	if matches!(terrain, Terrain::BlueFake) {
		ctx.field.set_terrain(ent.pos, Terrain::Floor);
	}

	// Wait until movement is cleared before accepting new input
	if ent.move_dir.is_none() {

		// Turn dirt to floor after stepping on it
		if matches!(terrain, Terrain::Dirt) {
			ctx.field.set_terrain(ent.pos, Terrain::Floor);
		}

		// Freeze the player when they reach the exit
		if matches!(terrain, Terrain::Exit) {
			ent.trapped = true;
		}

		// Set the player's move speed
		if !ctx.pl.inv.suction_boots && matches!(terrain, Terrain::ForceW | Terrain::ForceE | Terrain::ForceN | Terrain::ForceS) {
			ent.move_spd = BASE_SPD * 0.5;
		}
		else if !ctx.pl.inv.ice_skates && matches!(terrain, Terrain::Ice | Terrain::IceNE | Terrain::IceSE | Terrain::IceNW | Terrain::IceSW) {
			ent.move_spd = BASE_SPD * 0.5;
		}
		else {
			ent.move_spd = BASE_SPD;
		}

		'end_move: {
			// First tick after stepping on a new tile
			if let Some(orig_dir) = orig_dir {

				// Handle switch tiles
				if matches!(terrain, Terrain::GreenButton) {
					entities::press_green_button(ctx);
				}
				if matches!(terrain, Terrain::RedButton) {
					entities::press_red_button(ctx, ent.pos);
				}
				if matches!(terrain, Terrain::BrownButton) {
					entities::press_brown_button(ctx, ent.pos);
				}
				if matches!(terrain, Terrain::BlueButton) {
					entities::press_blue_button(ctx);
				}
				if matches!(terrain, Terrain::BearTrap) {
					ent.trapped = true;
				}
				if matches!(terrain, Terrain::Teleport) {
					ent.pos = find_teleport_dest(ent.pos, orig_dir, ctx);
					try_move(ent, orig_dir, ctx);
					break 'end_move;
				}

				// Handle ice physics
				if !ctx.pl.inv.ice_skates && matches!(terrain, Terrain::Ice | Terrain::IceNW | Terrain::IceNE | Terrain::IceSW | Terrain::IceSE) {
					let (ice_dir, back_dir) = match orig_dir {
						Dir::Up => match terrain {
							Terrain::IceNW => (Dir::Right, Dir::Down),
							Terrain::IceNE => (Dir::Left, Dir::Down),
							Terrain::IceSE => (Dir::Up, Dir::Left),
							Terrain::IceSW => (Dir::Up, Dir::Right),
							_ => (orig_dir, orig_dir.turn_around()),
						},
						Dir::Left => match terrain {
							Terrain::IceNW => (Dir::Down, Dir::Right),
							Terrain::IceNE => (Dir::Left, Dir::Down),
							Terrain::IceSE => (Dir::Left, Dir::Up),
							Terrain::IceSW => (Dir::Up, Dir::Right),
							_ => (orig_dir, orig_dir.turn_around()),
						},
						Dir::Down => match terrain {
							Terrain::IceNW => (Dir::Down, Dir::Right),
							Terrain::IceNE => (Dir::Down, Dir::Left),
							Terrain::IceSE => (Dir::Left, Dir::Up),
							Terrain::IceSW => (Dir::Right, Dir::Up),
							_ => (orig_dir, orig_dir.turn_around()),
						},
						Dir::Right => match terrain {
							Terrain::IceNW => (Dir::Right, Dir::Down),
							Terrain::IceNE => (Dir::Down, Dir::Left),
							Terrain::IceSE => (Dir::Up, Dir::Left),
							Terrain::IceSW => (Dir::Right, Dir::Up),
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

			if ent.trapped {
				break 'end_move;
			}

			// Handle force tiles
			let force_dir = match terrain {
				_ if ctx.pl.inv.suction_boots => None,
				Terrain::ForceW => Some(Dir::Left),
				Terrain::ForceE => Some(Dir::Right),
				Terrain::ForceN => Some(Dir::Up),
				Terrain::ForceS => Some(Dir::Down),
				_ => None,
			};
			let forced_move = ctx.pl.inv.forced_move;
			ctx.pl.inv.forced_move = false;
			// let first_time_force_dir = ctx.pl.inv.force_dir.is_none();
			// ctx.pl.inv.force_dir = None;
			if let Some(force_dir) = force_dir {

				let override_dir = match force_dir {
					_ if !forced_move || ent.trapped => None,
					Dir::Left | Dir::Right => if ctx.input.up { Some(Dir::Up) } else if ctx.input.down { Some(Dir::Down) } else { None },
					Dir::Up | Dir::Down => if ctx.input.left { Some(Dir::Left) } else if ctx.input.right { Some(Dir::Right) } else { None },
				};

				if override_dir.is_none() {
					// ctx.pl.inv.force_dir = Some(force_dir);
					ctx.pl.inv.forced_move = true;
				}

				match override_dir {
					Some(override_dir) if try_move(ent, override_dir, ctx) => true,
					_ => try_move(ent, force_dir, ctx),
				};

				break 'end_move;
			}

			// Handle player input
			if ent.trapped { }
			else if ctx.input.left && try_move(ent, Dir::Left, ctx) { }
			else if ctx.input.right && try_move(ent, Dir::Right, ctx) { }
			else if ctx.input.up && try_move(ent, Dir::Up, ctx) { }
			else if ctx.input.down && try_move(ent, Dir::Down, ctx) { }
		}
	}
}

fn find_teleport_dest(orig_pos: Vec2<i32>, dir: Dir, ctx: &mut ThinkContext) -> Vec2<i32> {
	let mut pos = orig_pos;
	loop {
		let Some(dest) = ctx.field.get_conn_dest(pos) else { break };
		let flags = CanMoveFlags {
			gravel: true,
			fire: true,
		};
		pos = dest;
		if ctx.field.can_move(dest, dir, &flags) {
			break;
		}
		if pos == orig_pos {
			break;
		}
	}
	return pos;
}

fn door_anim(objects: &mut ObjectMap, pos: Vec2<i32>, color: KeyColor) {
	objects.create(Object {
		handle: Default::default(),
		entity_handle: Default::default(),
		entity_kind: EntityKind::Sprite,
		pos: pos.map(|c| c as f32 * 32.0).vec3(0.0),
		vel: Vec3(0.0, 0.0, -200.0),
		sprite: match color {
			KeyColor::Blue => Sprite::BlueLock,
			KeyColor::Red => Sprite::RedLock,
			KeyColor::Green => Sprite::GreenLock,
			KeyColor::Yellow => Sprite::YellowLock,
		},
		model: Model::Wall,
		anim: Animation::Fall,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
	});
}

fn try_move(ent: &mut Entity, move_dir: Dir, ctx: &mut ThinkContext) -> bool {
	let new_pos = ent.pos + move_dir.to_vec();

	let terrain = ctx.field.get_terrain(new_pos);
	if matches!(terrain, Terrain::BlueLock) && ctx.pl.inv.keys[0] > 0 {
		ctx.field.set_terrain(new_pos, Terrain::Floor);
		ctx.pl.inv.keys[0] -= 1;
		door_anim(&mut ctx.objects, new_pos, KeyColor::Blue);
	}
	if matches!(terrain, Terrain::RedLock) && ctx.pl.inv.keys[1] > 0 {
		ctx.field.set_terrain(new_pos, Terrain::Floor);
		ctx.pl.inv.keys[1] -= 1;
		door_anim(&mut ctx.objects, new_pos, KeyColor::Red);
	}
	if matches!(terrain, Terrain::GreenLock) && ctx.pl.inv.keys[2] > 0 {
		ctx.field.set_terrain(new_pos, Terrain::Floor);
		// ctx.pl.inv.keys[2] -= 1; // Green keys are infinite
		door_anim(&mut ctx.objects, new_pos, KeyColor::Green);
	}
	if matches!(terrain, Terrain::YellowLock) && ctx.pl.inv.keys[3] > 0 {
		ctx.field.set_terrain(new_pos, Terrain::Floor);
		ctx.pl.inv.keys[3] -= 1;
		door_anim(&mut ctx.objects, new_pos, KeyColor::Yellow);
	}
	if matches!(terrain, Terrain::BlueWall) {
		ctx.field.set_terrain(new_pos, Terrain::Wall);
	}
	if matches!(terrain, Terrain::HiddenWall) {
		ctx.field.set_terrain(new_pos, Terrain::Wall);
	}

	let flags = CanMoveFlags {
		gravel: true,
		fire: true,
	};
	let mut success = ctx.field.can_move(ent.pos, move_dir, &flags);
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
		if ctx.field.get_terrain(ent.pos) == Terrain::RecessedWall {
			ctx.field.set_terrain(ent.pos, Terrain::Wall);
		}

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

	let terrain = ctx.field.get_terrain(ent.pos);

	if ent.move_dir.is_none() && terrain == Terrain::Exit {
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

	if terrain == Terrain::Water {
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
