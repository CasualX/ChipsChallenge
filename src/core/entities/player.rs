use super::*;

/// Time after which Chip returns to idle animation
const IDLE_TIME: Time = 20;

pub fn create(s: &mut GameState, args: &EntityArgs) -> EntityHandle {
	let handle = s.ents.alloc();
	s.ps.entity = handle;
	s.ents.insert(Entity {
		funcs: &FUNCS,
		handle,
		kind: args.kind,
		pos: args.pos,
		face_dir: args.face_dir,
		step_dir: None,
		step_spd: BASE_SPD,
		step_time: 0,
		trapped: false,
		hidden: false,
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
			s.events.push(GameEvent::EntityFaceDir { entity: ent.handle });
		}
		ent.face_dir = None;
	}
	if s.time >= ent.step_time + ent.step_spd {
		if ent.step_dir.is_some() {
			if matches!(terrain, Terrain::Fire) && !s.ps.fire_boots {
				ps_action(s, PlayerAction::Burn);
				return;
			}
			if matches!(terrain, Terrain::Water) && !s.ps.flippers {
				ps_action(s, PlayerAction::Drown);
				return;
			}
		}

		ent.step_dir = None;
	}

	let action = match terrain {
		Terrain::Water => PlayerAction::Swim,
		Terrain::Ice | Terrain::IceNE | Terrain::IceNW | Terrain::IceSE | Terrain::IceSW => if s.ps.ice_skates { PlayerAction::Skate } else { PlayerAction::Slide },
		Terrain::ForceN | Terrain::ForceW | Terrain::ForceS | Terrain::ForceE | Terrain::ForceRandom => if s.ps.suction_boots { PlayerAction::Suction } else { PlayerAction::Slide },
		_ => PlayerAction::Walk,
	};
	ps_action(s, action);

	// Turn dirt to floor after stepping on it
	if matches!(terrain, Terrain::Dirt) {
		s.field.set_terrain(ent.pos, Terrain::Floor);
	}

	// Wait until movement is cleared before accepting new input
	if s.time >= ent.step_time + ent.step_spd {
		let input_dir = s.ps.inbuf.read_move();

		// Win condition
		if matches!(terrain, Terrain::Exit) && orig_dir.is_some() {
			s.events.push(GameEvent::EntityFaceDir { entity: ent.handle });
			ps_action(s, PlayerAction::Win);
			return;
		}

		if s.ps.dev_ghost {
			if let Some(input_dir) = input_dir {
				try_move(s, ent, input_dir);
				return;
			}
		}

		'end_move: {
			// First tick after stepping on a new tile
			if let Some(orig_dir) = orig_dir {

				// Handle switch tiles
				// if matches!(terrain, Terrain::GreenButton) {
				// 	entities::press_green_button(s, ent.handle);
				// }
				// if matches!(terrain, Terrain::RedButton) {
				// 	entities::press_red_button(s, ent.pos);
				// }
				// if matches!(terrain, Terrain::BrownButton) {
				// 	entities::press_brown_button(s, ent.pos);
				// }
				// if matches!(terrain, Terrain::BlueButton) {
				// 	entities::press_blue_button(s);
				// }
				// if matches!(terrain, Terrain::BearTrap) {
				// 	ent.trapped = true;
				// }
				if matches!(terrain, Terrain::Teleport) {
					teleport(s, ent, orig_dir);
					try_move(s, ent, orig_dir);
					break 'end_move;
				}
				if matches!(terrain, Terrain::Hint) {
					s.events.push(GameEvent::PlayerHint { player: ent.handle, pos: ent.pos });
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
					Dir::Left | Dir::Right => if input_dir == Some(Dir::Up) { Some(Dir::Up) } else if input_dir == Some(Dir::Down) { Some(Dir::Down) } else { None },
					Dir::Up | Dir::Down => if input_dir == Some(Dir::Left) { Some(Dir::Left) } else if input_dir == Some(Dir::Right) { Some(Dir::Right) } else { None },
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
			else if let Some(dir) = input_dir {
				try_move(s, ent, dir);
			}
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
			dirt: true,
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
	s.events.push(GameEvent::EntityTeleport { entity: ent.handle });
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
		Terrain::BlueFake => {
			s.field.set_terrain(new_pos, Terrain::Floor);
			s.events.push(GameEvent::BlueWallCleared { pos: new_pos });
		}
		Terrain::HiddenWall => {
			s.field.set_terrain(new_pos, Terrain::HiddenWallRevealed);
			s.events.push(GameEvent::HiddenWallBumped { pos: new_pos });
		}
		_ => {}
	}

	let flags = CanMoveFlags {
		gravel: true,
		fire: true,
		dirt: true,
	};
	let mut success = s.ps.dev_ghost || s.field.can_move(ent.pos, move_dir, &flags);
	if success {
		for handle in s.ents.map.keys().cloned().collect::<Vec<_>>() {
			let Some(mut ent) = s.ents.remove(handle) else { continue };
			let mut ictx = InteractContext {
				blocking: false,
				push_dir: move_dir,
			};
			if ent.pos == new_pos {
				interact(s, &mut ent, &mut ictx);
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

	s.events.push(GameEvent::EntityFaceDir { entity: ent.handle });
	ent.face_dir = Some(move_dir);
	ent.step_time = s.time;
	if success {
		let terrain = s.field.get_terrain(ent.pos);
		if matches!(terrain, Terrain::RecessedWall) {
			s.events.push(GameEvent::RecessedWallRaised { pos: ent.pos });
			s.field.set_terrain(ent.pos, Terrain::RaisedWall);
		}

		ent.step_dir = Some(move_dir);
		ent.pos = new_pos;
		interact_terrain(s, ent);

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

		s.ps.steps += 1;
		s.events.push(GameEvent::EntityStep { entity: ent.handle });
	}
	else {
		ent.step_spd = BASE_SPD / 2;
	}

	return success;
}

fn interact(s: &mut GameState, ent: &mut Entity, ictx: &mut InteractContext) {
	match ent.kind {
		EntityKind::Block => {
			if ent.trapped {
				ictx.blocking = true;
				return;
			}

			let terrain = s.field.get_terrain(ent.pos);
			if matches!(terrain, Terrain::Water) || is_solid_or_dirt(ent.pos, ictx.push_dir, &s.field, &s.ents) {
				ictx.blocking = true;
			}
			else {
				ictx.blocking = false;
				ent.pos += ictx.push_dir.to_vec();
				ent.step_dir = Some(ictx.push_dir);
				ent.face_dir = Some(ictx.push_dir);
				ent.step_time = s.time;

				s.events.push(GameEvent::EntityStep { entity: ent.handle });

				update_hidden_entities(s);

				let terrain = s.field.get_terrain(ent.pos);
				if matches!(terrain, Terrain::BearTrap) {
					ent.trapped = true;
				}
			}
		}
		EntityKind::Socket => {
			if s.ps.chips >= s.field.chips {
				ent.remove = true;
				ictx.blocking = false;
				s.events.push(GameEvent::SocketFilled { pos: ent.pos });
			}
			else {
				ictx.blocking = true;
			}
		}
		EntityKind::Thief => {
			ictx.blocking = false;
			s.ps.flippers = false;
			s.ps.fire_boots = false;
			s.ps.ice_skates = false;
			s.ps.suction_boots = false;
			s.events.push(GameEvent::ItemsThief { player: s.ps.entity });
		}
		_ => {}
	}
}

fn is_solid_or_dirt(pos: Vec2i, move_dir: Dir, field: &Field, entities: &EntityMap) -> bool {
	let flags = CanMoveFlags {
		gravel: false,
		fire: true,
		dirt: false,
	};
	if !field.can_move(pos, move_dir, &flags) {
		return true;
	}

	let new_pos = pos + move_dir.to_vec();
	for ent in entities.map.values() {
		if ent.pos == new_pos {
			let solid = match ent.kind {
				EntityKind::Socket => true,
				EntityKind::Block => true,
				_ => false,
			};
			if solid {
				return true;
			}
		}
	}
	false
}

static FUNCS: EntityFuncs = EntityFuncs { think };
