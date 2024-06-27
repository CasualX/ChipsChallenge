use super::*;

pub const BASE_SPD: i32 = 20;

pub mod player;
pub mod pickup;
pub mod socket;
pub mod block;
pub mod bug;
pub mod tank;
pub mod bomb;
pub mod pinkball;
pub mod fireball;
pub mod thief;
pub mod glider;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug)]
pub struct SpawnData {
	pub kind: EntityKind,
	pub pos: Vec2i,
	pub face_dir: Option<Dir>,
}

pub fn create(s: &mut GameState, data: &SpawnData) -> EntityHandle {
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
		EntityKind::Bomb => bomb::create(s, data),
	};
	s.events.push(GameEvent::EntityCreated { handle });
	return handle;
}

pub fn press_green_button(s: &mut GameState) {
	for ptr in s.field.terrain.iter_mut() {
		if *ptr == Terrain::ToggleFloor {
			*ptr = Terrain::ToggleWall;
		}
		else if *ptr == Terrain::ToggleWall {
			*ptr = Terrain::ToggleFloor;
		}
	}
}

pub fn press_red_button(s: &mut GameState, pos: Vec2i) {
	let Some(conn) = s.field.conns.iter().cloned().find(|conn| conn.src == pos) else { return };
	if let Some(template_ent) = s.ents.map.values().find(|ent| ent.pos == conn.dest) {
		let ent_dto = SpawnData {
			kind: template_ent.kind,
			pos: template_ent.pos,
			face_dir: template_ent.face_dir,
		};
		let h = create(s, &ent_dto);
		if let Some(ent) = s.ents.get_mut(h) {
			ent.move_dir = ent_dto.face_dir;
		}
	}
}

pub fn press_brown_button(s: &mut GameState, pos: Vec2i) {
	let Some(conn) = s.field.conns.iter().find(|conn| conn.src == pos) else { return };
	for ent in s.ents.map.values_mut() {
		if ent.pos == conn.dest {
			ent.trapped = false;
		}
	}
}

pub fn press_blue_button(s: &mut GameState) {
	for other in s.ents.map.values_mut() {
		if other.kind == EntityKind::Tank {
			if let Some(face_dir) = other.face_dir {
				other.face_dir = Some(face_dir.turn_around());
			}
		}
	}
}
