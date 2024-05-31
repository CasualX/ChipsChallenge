use super::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct EntityHandle(pub u32);

#[derive(Copy, Clone, Debug)]
pub enum EntityKind {
	Player,
	Chip,
	Barrier,
	Block,
	BlueKey,
	RedKey,
	GreenKey,
	YellowKey,
	BlueDoor,
	RedDoor,
	GreenDoor,
	YellowDoor,
	EnemyBug,
}

#[derive(Copy, Clone, Debug)]
pub struct Entity {
	pub handle: EntityHandle,
	pub kind: EntityKind,
	pub pos: Vec2<i32>,
	pub move_dir: Option<Dir>,
	pub face_dir: Option<Dir>,
	pub move_time: f32,
}

const MOVE_TIME: f32 = 0.125;

impl Entity {
	pub fn think(&mut self, ctx: &mut ThinkContext) -> bool {
		let _ = ctx;
		let mut ret = true;

		let ent = self;
		'ret: {
			match ent.kind {
				EntityKind::Player => {
					let floor = ctx.field.lookup_tile(Tile::Floor).unwrap();
					if ctx.field.get_tile(ent.pos).tile == Tile::Dirt {
						ctx.field.set_tile(ent.pos, floor);
					}
				}
				EntityKind::Block => {
					if let Some(_) = ent.move_dir {
						if ctx.time >= ent.move_time + MOVE_TIME {
							ent.move_dir = None;
							ent.face_dir = None;
							if ctx.field.get_tile(ent.pos).tile == Tile::Water {
								let dirt = ctx.field.lookup_tile(Tile::Dirt).unwrap();
								ctx.field.set_tile(ent.pos, dirt);
								ret = false;
							}
						}
					}
				}
				EntityKind::EnemyBug => {
					let Some(face_dir) = ent.face_dir else { break 'ret };
					if ctx.time >= ent.move_time + MOVE_TIME {
						ent.move_dir = None;
					}
					if ctx.time >= ent.move_time + MOVE_TIME + 0.125 * 0.5 {
						// Check if wall on the left
						let up = ent.pos + face_dir.to_vec();
						let tile = ctx.field.get_tile(up);
						if tile.solid {
							ent.face_dir = Some(face_dir.turn_right());
						}
						else {
							let left = ent.pos + face_dir.turn_left().to_vec();
							let tile = ctx.field.get_tile(left);
							if !tile.solid {
								ent.face_dir = Some(face_dir.turn_left());
							}
						}
						ent.move_dir = ent.face_dir;
						ent.move_time = ctx.time;
						ent.pos += ent.move_dir.unwrap().to_vec();
					}
				}
				_ => (),
			}
		}

		return ret;
	}

	/// Player interacts with an entity by moving into it.
	pub fn interact(&mut self, ctx: &mut ThinkContext, ictx: &mut InteractContext) {
		let pl = &mut ctx.pl;
		let ent = self;
		match ent.kind {
			EntityKind::Chip => {
				pl.inv.chips += 1;
				ictx.remove_entity = true;
				ictx.blocking = false;
			},
			EntityKind::Barrier => {
				if pl.inv.chips >= ctx.field.chips {
					ictx.remove_entity = true;
					ictx.blocking = false;
				}
				else {
					ictx.blocking = true;
				}
			},
			EntityKind::Block => {
				let dirt = ctx.field.lookup_tile(Tile::Dirt);
				fn is_solid_or_dirt(pos: Vec2<i32>, field: &Field, entities: &EntityMap) -> bool {
					let tile = field.get_tile(pos);
					if tile.solid || tile.tile == Tile::Dirt {
						return true;
					}
					for ent in entities.map.values() {
						if ent.pos == pos {
							let solid = match ent.kind {
								EntityKind::Barrier => true,
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
				if dirt.is_none() || is_solid_or_dirt(ent.pos + ictx.push_dir.to_vec(), &ctx.field, &ctx.entities) {
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
			},
			EntityKind::BlueKey => {
				pl.inv.keys[0] += 1;
				ictx.remove_entity = true;
				ictx.blocking = false;
			},
			EntityKind::RedKey => {
				pl.inv.keys[1] += 1;
				ictx.remove_entity = true;
				ictx.blocking = false;
			},
			EntityKind::GreenKey => {
				pl.inv.keys[2] += 1;
				ictx.remove_entity = true;
				ictx.blocking = false;
			},
			EntityKind::YellowKey => {
				pl.inv.keys[3] += 1;
				ictx.remove_entity = true;
				ictx.blocking = false;
			},
			EntityKind::BlueDoor => {
				if pl.inv.keys[0] > 0 {
					pl.inv.keys[0] -= 1;
					ictx.remove_entity = true;
					ictx.blocking = false;
				}
				else {
					ictx.blocking = true;
				}
			},
			EntityKind::RedDoor => {
				if pl.inv.keys[1] > 0 {
					pl.inv.keys[1] -= 1;
					ictx.remove_entity = true;
					ictx.blocking = false;
				}
				else {
					ictx.blocking = true;
				}
			},
			EntityKind::GreenDoor => {
				if pl.inv.keys[2] > 0 {
					// pl.keys[2] -= 1;
					ictx.remove_entity = true;
					ictx.blocking = false;
				}
				else {
					ictx.blocking = true;
				}
			},
			EntityKind::YellowDoor => {
				if pl.inv.keys[3] > 0 {
					pl.inv.keys[3] -= 1;
					ictx.remove_entity = true;
					ictx.blocking = false;
				}
				else {
					ictx.blocking = true;
				}
			},
			_ => (),
		}
	}
}
