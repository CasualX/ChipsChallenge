use super::*;

/// Time after which Chip returns to idle animation
const IDLE_TIME: Time = 20;

pub fn create(s: &mut GameState, data: &SpawnData) -> EntityHandle {
	let handle = s.ents.alloc();
	s.ps.entity = handle;
	s.ents.insert(Entity {
		funcs: &FUNCS,
		handle,
		kind: data.kind,
		pos: data.pos,
		face_dir: data.face_dir,
		step_dir: None,
		step_spd: BASE_SPD,
		step_time: 0,
		trapped: false,
		remove: false,
	});
	return handle;
}

fn think(s: &mut GameState, ent: &mut Entity) {
	let terrain = s.field.get_terrain(ent.pos);
	let orig_dir = ent.step_dir;

	// Freeze player if game over
	if matches!(s.ps.action, PlayerAction::Win | PlayerAction::Burn | PlayerAction::Drown | PlayerAction::Death) {
		return;
	}

	// Clear movement after a delay
	if s.time >= ent.step_time + IDLE_TIME {
		if ent.face_dir.is_some() {
			s.events.push(GameEvent::EntityFaceDir { handle: ent.handle });
		}
		ent.face_dir = None;
	}
	if s.time >= ent.step_time + ent.step_spd {
		if ent.step_dir.is_some() {
			if matches!(terrain, Terrain::Fire) && !s.ps.fire_boots {
				s.ps.action = PlayerAction::Burn;
				s.events.push(GameEvent::PlayerActionChanged { handle: ent.handle });
				return;
			}
			if matches!(terrain, Terrain::Water) && !s.ps.flippers {
				s.ps.action = PlayerAction::Drown;
				s.events.push(GameEvent::PlayerActionChanged { handle: ent.handle });
				return;
			}
		}

		ent.step_dir = None;
	}

	// Turn BlueFake to floor when stepping through it
	if matches!(terrain, Terrain::BlueFake) {
		s.field.set_terrain(ent.pos, Terrain::Floor);
	}

	let action = match terrain {
		Terrain::Water => PlayerAction::Swim,
		Terrain::Ice | Terrain::IceNE | Terrain::IceNW | Terrain::IceSE | Terrain::IceSW => if s.ps.ice_skates { PlayerAction::Skate } else { PlayerAction::Slide },
		Terrain::ForceN | Terrain::ForceW | Terrain::ForceS | Terrain::ForceE | Terrain::ForceRandom => if s.ps.suction_boots { PlayerAction::Suction } else { PlayerAction::Slide },
		_ => PlayerAction::Walk,
	};
	if action != s.ps.action {
		s.events.push(GameEvent::PlayerActionChanged { handle: ent.handle });
		s.ps.action = action;
	}

	// Wait until movement is cleared before accepting new input
	if ent.step_dir.is_none() {

		// Turn dirt to floor after stepping on it
		if matches!(terrain, Terrain::Dirt) {
			s.field.set_terrain(ent.pos, Terrain::Floor);
		}
		// Win condition
		if matches!(terrain, Terrain::Exit) && orig_dir.is_some() {
			s.ps.action = PlayerAction::Win;
			s.events.push(GameEvent::EntityFaceDir { handle: ent.handle });
			s.events.push(GameEvent::GameWin);
			return;
		}

		// Set the player's move speed
		if !s.ps.suction_boots && matches!(terrain, Terrain::ForceW | Terrain::ForceE | Terrain::ForceN | Terrain::ForceS) {
			ent.step_spd = BASE_SPD / 2;
		}
		else if !s.ps.ice_skates && matches!(terrain, Terrain::Ice | Terrain::IceNE | Terrain::IceSE | Terrain::IceNW | Terrain::IceSW) {
			ent.step_spd = BASE_SPD / 2;
		}
		else {
			ent.step_spd = BASE_SPD;
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
					teleport(s, ent, orig_dir);
					try_move(s, ent, orig_dir);
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
					if !try_move(s, ent, ice_dir) {
						if !try_move(s, ent, back_dir) {
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
			if let Some(force_dir) = force_dir {

				let override_dir = match force_dir {
					_ if !forced_move || ent.trapped => None,
					Dir::Left | Dir::Right => if s.input.up { Some(Dir::Up) } else if s.input.down { Some(Dir::Down) } else { None },
					Dir::Up | Dir::Down => if s.input.left { Some(Dir::Left) } else if s.input.right { Some(Dir::Right) } else { None },
				};

				if override_dir.is_none() {
					s.ps.forced_move = true;
				}

				match override_dir {
					Some(override_dir) if try_move(s, ent, override_dir) => true,
					_ => try_move(s, ent, force_dir),
				};

				break 'end_move;
			}

			// Handle player input
			if ent.trapped { }
			else if s.input.left && try_move(s, ent, Dir::Left) { }
			else if s.input.right && try_move(s, ent, Dir::Right) { }
			else if s.input.up && try_move(s, ent, Dir::Up) { }
			else if s.input.down && try_move(s, ent, Dir::Down) { }
		}
	}
}

fn teleport(s: &mut GameState, ent: &mut Entity, dir: Dir) {
	let mut pos = ent.pos;
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
		if pos == ent.pos {
			break;
		}
	}
	ent.pos = pos;
	s.events.push(GameEvent::EntityTeleport { handle: ent.handle });
}

fn try_move(s: &mut GameState, ent: &mut Entity, move_dir: Dir) -> bool {
	let new_pos = ent.pos + move_dir.to_vec();

	let terrain = s.field.get_terrain(new_pos);
	match terrain {
		Terrain::BlueLock => if s.ps.keys[KeyColor::Blue as usize] > 0 {
			s.field.set_terrain(new_pos, Terrain::Floor);
			s.ps.keys[KeyColor::Blue as usize] -= 1;
			s.events.push(GameEvent::LockRemoved { pos: new_pos, key: KeyColor::Blue });
		}
		Terrain::RedLock => if s.ps.keys[KeyColor::Red as usize] > 0 {
			s.field.set_terrain(new_pos, Terrain::Floor);
			s.ps.keys[KeyColor::Red as usize] -= 1;
			s.events.push(GameEvent::LockRemoved { pos: new_pos, key: KeyColor::Red });
		}
		Terrain::GreenLock => if s.ps.keys[KeyColor::Green as usize] > 0 {
			s.field.set_terrain(new_pos, Terrain::Floor);
			// s.ps.keys[KeyColor::Green as usize] -= 1; // Green keys are infinite
			s.events.push(GameEvent::LockRemoved { pos: new_pos, key: KeyColor::Green });
		}
		Terrain::YellowLock => if s.ps.keys[KeyColor::Yellow as usize] > 0 {
			s.field.set_terrain(new_pos, Terrain::Floor);
			s.ps.keys[KeyColor::Yellow as usize] -= 1;
			s.events.push(GameEvent::LockRemoved { pos: new_pos, key: KeyColor::Yellow });
		}
		Terrain::BlueWall => {
			s.field.set_terrain(new_pos, Terrain::Wall);
			s.events.push(GameEvent::BlueWallBumped { pos: new_pos });
		}
		Terrain::HiddenWall => {
			s.field.set_terrain(new_pos, Terrain::Wall);
			s.events.push(GameEvent::HiddenWallBumped { pos: new_pos });
		}
		_ => {}
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
				(ent.funcs.interact)(s, &mut ent, &mut ictx);
			}
			s.ents.insert(ent);
			if ictx.blocking {
				success = false;
				// This is tricky. Consider the following:
				// A block is on top of an item pickup (Chip, etc)
				// If we continued and interacted with all entities, the player can interact with the item pickup through the block
				// To prevent that break here BUT the block must be earlier in the entity list than the item pickup
				break;
			}
		}
	}

	s.events.push(GameEvent::EntityFaceDir { handle: ent.handle });
	ent.face_dir = Some(move_dir);
	ent.step_time = s.time;
	if success {
		let terrain = s.field.get_terrain(ent.pos);
		if matches!(terrain, Terrain::RecessedWall) {
			s.field.set_terrain(ent.pos, Terrain::Wall);
		}

		ent.step_dir = Some(move_dir);
		ent.pos = new_pos;
		s.ps.steps += 1;
		s.events.push(GameEvent::EntityStep { handle: ent.handle });
	}

	return success;
}

fn interact(_s: &mut GameState, _ent: &mut Entity, ictx: &mut InteractContext) {
	ictx.blocking = false;
}

static FUNCS: EntityFuncs = EntityFuncs { think, interact };
