use super::*;

const IDLE_TIME: i32 = 30;

pub fn create(s: &mut GameState, data: &SpawnData) -> EntityHandle {
	let handle = s.ents.alloc();
	s.ents.insert(Entity {
		funcs: &FUNCS,
		handle,
		kind: data.kind,
		pos: data.pos,
		move_dir: None,
		move_spd: BASE_SPD,
		move_time: 0,
		face_dir: data.face_dir,
		trapped: false,
		remove: false,
	});
	return handle;
}

fn think(ent: &mut Entity, s: &mut GameState) {
	let terrain = s.field.get_terrain(ent.pos);
	let orig_dir = ent.move_dir;

	// Clear movement after a delay
	if s.time >= ent.move_time + IDLE_TIME {
		ent.face_dir = None;
	}
	if s.time >= ent.move_time + ent.move_spd {
		ent.move_dir = None;
	}

	// Turn BlueFake to floor when stepping through it
	if matches!(terrain, Terrain::BlueFake) {
		s.field.set_terrain(ent.pos, Terrain::Floor);
	}

	// Wait until movement is cleared before accepting new input
	if ent.move_dir.is_none() {

		// Turn dirt to floor after stepping on it
		if matches!(terrain, Terrain::Dirt) {
			s.field.set_terrain(ent.pos, Terrain::Floor);
		}

		// Freeze the player when they reach the exit
		if matches!(terrain, Terrain::Exit) {
			ent.trapped = true;
		}

		// Set the player's move speed
		if !s.ps.suction_boots && matches!(terrain, Terrain::ForceW | Terrain::ForceE | Terrain::ForceN | Terrain::ForceS) {
			ent.move_spd = BASE_SPD / 2;
		}
		else if !s.ps.ice_skates && matches!(terrain, Terrain::Ice | Terrain::IceNE | Terrain::IceSE | Terrain::IceNW | Terrain::IceSW) {
			ent.move_spd = BASE_SPD / 2;
		}
		else {
			ent.move_spd = BASE_SPD;
		}

		'end_move: {
			// First tick after stepping on a new tile
			if let Some(orig_dir) = orig_dir {

				// Handle switch tiles
				if matches!(terrain, Terrain::GreenButton) {
					entities::press_green_button(s);
				}
				if matches!(terrain, Terrain::RedButton) {
					entities::press_red_button(s, ent.pos);
				}
				if matches!(terrain, Terrain::BrownButton) {
					entities::press_brown_button(s, ent.pos);
				}
				if matches!(terrain, Terrain::BlueButton) {
					entities::press_blue_button(s);
				}
				if matches!(terrain, Terrain::BearTrap) {
					ent.trapped = true;
				}
				if matches!(terrain, Terrain::Teleport) {
					ent.pos = find_teleport_dest(ent.pos, orig_dir, s);
					try_move(ent, orig_dir, s);
					break 'end_move;
				}

				// Handle ice physics
				if !s.ps.ice_skates && matches!(terrain, Terrain::Ice | Terrain::IceNW | Terrain::IceNE | Terrain::IceSW | Terrain::IceSE) {
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
					if !try_move(ent, ice_dir, s) {
						if !try_move(ent, back_dir, s) {
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
				_ if s.ps.suction_boots => None,
				Terrain::ForceW => Some(Dir::Left),
				Terrain::ForceE => Some(Dir::Right),
				Terrain::ForceN => Some(Dir::Up),
				Terrain::ForceS => Some(Dir::Down),
				_ => None,
			};
			let forced_move = s.ps.forced_move;
			s.ps.forced_move = false;
			// let first_time_force_dir = ctx.ps.force_dir.is_none();
			// ctx.ps.force_dir = None;
			if let Some(force_dir) = force_dir {

				let override_dir = match force_dir {
					_ if !forced_move || ent.trapped => None,
					Dir::Left | Dir::Right => if s.input.up { Some(Dir::Up) } else if s.input.down { Some(Dir::Down) } else { None },
					Dir::Up | Dir::Down => if s.input.left { Some(Dir::Left) } else if s.input.right { Some(Dir::Right) } else { None },
				};

				if override_dir.is_none() {
					// ctx.ps.force_dir = Some(force_dir);
					s.ps.forced_move = true;
				}

				match override_dir {
					Some(override_dir) if try_move(ent, override_dir, s) => true,
					_ => try_move(ent, force_dir, s),
				};

				break 'end_move;
			}

			// Handle player input
			if ent.trapped { }
			else if s.input.left && try_move(ent, Dir::Left, s) { }
			else if s.input.right && try_move(ent, Dir::Right, s) { }
			else if s.input.up && try_move(ent, Dir::Up, s) { }
			else if s.input.down && try_move(ent, Dir::Down, s) { }
		}
	}
}

fn find_teleport_dest(orig_pos: Vec2i, dir: Dir, s: &mut GameState) -> Vec2i {
	let mut pos = orig_pos;
	loop {
		let Some(dest) = s.field.get_conn_dest(pos) else { break };
		let flags = CanMoveFlags {
			gravel: true,
			fire: true,
		};
		pos = dest;
		if s.field.can_move(dest, dir, &flags) {
			break;
		}
		if pos == orig_pos {
			break;
		}
	}
	return pos;
}

fn try_move(ent: &mut Entity, move_dir: Dir, s: &mut GameState) -> bool {
	let new_pos = ent.pos + move_dir.to_vec();

	let terrain = s.field.get_terrain(new_pos);
	if matches!(terrain, Terrain::BlueLock) && s.ps.keys[0] > 0 {
		s.field.set_terrain(new_pos, Terrain::Floor);
		s.ps.keys[0] -= 1;
		s.events.push(GameEvent::LockRemoved { pos: new_pos, key: KeyColor::Blue });
	}
	if matches!(terrain, Terrain::RedLock) && s.ps.keys[1] > 0 {
		s.field.set_terrain(new_pos, Terrain::Floor);
		s.ps.keys[1] -= 1;
		s.events.push(GameEvent::LockRemoved { pos: new_pos, key: KeyColor::Red });
	}
	if matches!(terrain, Terrain::GreenLock) && s.ps.keys[2] > 0 {
		s.field.set_terrain(new_pos, Terrain::Floor);
		// ctx.ps.keys[2] -= 1; // Green keys are infinite
		s.events.push(GameEvent::LockRemoved { pos: new_pos, key: KeyColor::Green });
	}
	if matches!(terrain, Terrain::YellowLock) && s.ps.keys[3] > 0 {
		s.field.set_terrain(new_pos, Terrain::Floor);
		s.ps.keys[3] -= 1;
		s.events.push(GameEvent::LockRemoved { pos: new_pos, key: KeyColor::Yellow });
	}
	if matches!(terrain, Terrain::BlueWall) {
		s.field.set_terrain(new_pos, Terrain::Wall);
		s.events.push(GameEvent::BlueWallBumped { pos: new_pos });
	}
	if matches!(terrain, Terrain::HiddenWall) {
		s.field.set_terrain(new_pos, Terrain::Wall);
		s.events.push(GameEvent::HiddenWallBumped { pos: new_pos });
	}

	let flags = CanMoveFlags {
		gravel: true,
		fire: true,
	};
	let mut success = s.field.can_move(ent.pos, move_dir, &flags);
	if success {
		for handle in s.ents.map.keys().cloned().collect::<Vec<_>>() {
			let Some(mut ent) = s.ents.remove(handle) else { continue };
			let mut ictx = InteractContext {
				blocking: false,
				push_dir: move_dir,
			};
			if ent.pos == new_pos {
				(ent.funcs.interact)(&mut ent, s, &mut ictx);
			}
			s.ents.insert(ent);
			if ictx.blocking {
				success = false;
			}
		}
	}

	ent.face_dir = Some(move_dir);
	ent.move_time = s.time;
	if success {
		if s.field.get_terrain(ent.pos) == Terrain::RecessedWall {
			s.field.set_terrain(ent.pos, Terrain::Wall);
		}

		ent.move_dir = Some(move_dir);
		ent.pos = new_pos;
		s.ps.steps += 1;
		s.events.push(GameEvent::EntityMoved { handle: ent.handle });
	}

	return success;
}

fn interact(_ent: &mut Entity, _s: &mut GameState, ictx: &mut InteractContext) {
	ictx.blocking = false;
}

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
