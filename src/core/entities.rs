use super::*;

const BASE_SPD: Time = 12;

mod player;
mod pickup;
mod socket;
mod block;
mod bug;
mod tank;
mod bomb;
mod pinkball;
mod fireball;
mod thief;
mod glider;
mod walker;
mod teeth;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug)]
pub struct EntityArgs {
	pub kind: EntityKind,
	pub pos: Vec2i,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub face_dir: Option<Dir>,
}

pub fn create(s: &mut GameState, data: &EntityArgs) -> EntityHandle {
	let handle = match data.kind {
		EntityKind::Player => player::create(s, data),
		EntityKind::Chip => pickup::create(s, data),
		EntityKind::Socket => socket::create(s, data),
		EntityKind::Block => block::create(s, data),
		EntityKind::Flippers => pickup::create(s, data),
		EntityKind::FireBoots => pickup::create(s, data),
		EntityKind::IceSkates => pickup::create(s, data),
		EntityKind::SuctionBoots => pickup::create(s, data),
		EntityKind::BlueKey => pickup::create(s, data),
		EntityKind::RedKey => pickup::create(s, data),
		EntityKind::GreenKey => pickup::create(s, data),
		EntityKind::YellowKey => pickup::create(s, data),
		EntityKind::Thief => thief::create(s, data),
		EntityKind::Bug => bug::create(s, data),
		EntityKind::Tank => tank::create(s, data),
		EntityKind::PinkBall => pinkball::create(s, data),
		EntityKind::FireBall => fireball::create(s, data),
		EntityKind::Glider => glider::create(s, data),
		EntityKind::Walker => walker::create(s, data),
		EntityKind::Teeth => teeth::create(s, data),
		EntityKind::Bomb => bomb::create(s, data),
	};
	s.events.push(GameEvent::EntityCreated { entity: handle });
	return handle;
}

pub fn update_hidden_entities(s: &mut GameState) {
	let mut blocks = HashMap::new();
	for ent in s.ents.map.values() {
		if matches!(ent.kind, EntityKind::Block) {
			blocks.insert(ent.pos, ent.handle);
		}
	}
	for ent in s.ents.map.values_mut() {
		if matches!(ent.kind, EntityKind::Block) {
			continue;
		}
		let mut hidden = blocks.contains_key(&ent.pos);
		if s.time == 0 && matches!(s.field.get_terrain(ent.pos), Terrain::CloneMachine) {
			hidden = true;
		}
		if hidden != ent.hidden {
			ent.hidden = hidden;
			s.events.push(GameEvent::EntityHidden { entity: ent.handle, hidden });
		}
	}
}

pub fn press_green_button(s: &mut GameState, entity: EntityHandle) {
	for ptr in s.field.terrain.iter_mut() {
		let terrain = *ptr;
		if matches!(terrain, Terrain::ToggleFloor) {
			*ptr = Terrain::ToggleWall;
		}
		else if matches!(terrain, Terrain::ToggleWall) {
			*ptr = Terrain::ToggleFloor;
		}
	}
	s.events.push(GameEvent::GreenButton { entity });
}

pub fn press_red_button(s: &mut GameState, pos: Vec2i) {
	let Some(conn) = s.field.conns.iter().cloned().find(|conn| conn.src == pos) else { return };
	if let Some(template_ent) = s.ents.map.values().find(|ent| ent.pos == conn.dest) {
		let ent_dto = EntityArgs {
			kind: template_ent.kind,
			pos: template_ent.pos,
			face_dir: template_ent.face_dir,
		};
		let h = create(s, &ent_dto);
		if let Some(ent) = s.ents.get_mut(h) {
			ent.step_dir = ent_dto.face_dir;
		}
	}
}

// pub fn press_brown_button(s: &mut GameState, pos: Vec2i) {
// 	let Some(conn) = s.field.conns.iter().find(|conn| conn.src == pos) else { return };
// 	for ent in s.ents.map.values_mut() {
// 		if ent.pos == conn.dest {
// 			ent.trapped = false;
// 		}
// 	}
// }

pub fn press_blue_button(s: &mut GameState) {
	for other in s.ents.map.values_mut() {
		if matches!(other.kind, EntityKind::Tank) {
			if let Some(face_dir) = other.face_dir {
				other.face_dir = Some(face_dir.turn_around());
			}
		}
	}
}

pub fn interact_terrain(s: &mut GameState, ent: &mut Entity) {
	let terrain = s.field.get_terrain(ent.pos);

	match terrain {
		Terrain::GreenButton => {
			for ptr in s.field.terrain.iter_mut() {
				let terrain = *ptr;
				if matches!(terrain, Terrain::ToggleFloor) {
					*ptr = Terrain::ToggleWall;
				}
				else if matches!(terrain, Terrain::ToggleWall) {
					*ptr = Terrain::ToggleFloor;
				}
			}
			s.events.push(GameEvent::GreenButton { entity: ent.handle });
		}
		Terrain::RedButton => {
			// Find the template entity connected to the red button
			let Some(conn) = s.field.conns.iter().cloned().find(|conn| conn.src == ent.pos) else { return };
			let Some(template_ent) = s.ents.map.values().find(|ent| ent.pos == conn.dest) else { return };
			// Spawn a new entity at the template entity's position
			let args = EntityArgs {
				kind: template_ent.kind,
				pos: template_ent.pos,
				face_dir: template_ent.face_dir,
			};
			let h = create(s, &args);
			// Force the new entity to move
			if let Some(ent) = s.ents.get_mut(h) {
				ent.step_dir = args.face_dir;
			}
			s.events.push(GameEvent::RedButton { entity: ent.handle });
		}
		Terrain::BlueButton => {
			for other in s.ents.map.values_mut() {
				if matches!(other.kind, EntityKind::Tank) {
					if let Some(face_dir) = other.face_dir {
						other.face_dir = Some(face_dir.turn_around());
					}
				}
			}
		}
		Terrain::Teleport => {

		}
		_ => {}
	}
}

pub fn is_brown_button_pressed(s: &GameState, pos: Vec2i) -> bool {
	for conn in &s.field.conns {
		if pos == conn.dest {
			for other in s.ents.map.values() {
				if other.pos == conn.src {
					return true;
				}
			}
		}
	}
	return false;
}
