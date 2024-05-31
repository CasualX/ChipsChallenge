use super::*;

#[derive(Copy, Clone, Default)]
pub struct PlayerInventory {
	/// Total steps taken (for high score).
	pub steps: i32,
	pub chips: i32,
	pub keys: [u8; 4],
	pub powerups: [bool; 4],
}

#[derive(Clone, Default)]
pub struct PlayerState {
	pub entity: EntityHandle,
	pub object: ObjectHandle,

	// pub sprite: Sprite,
	pub pos: Vec2<i32>,
	pub moving: bool,
	pub wallhit: bool,
	pub pos_offset: Vec2<f32>,
	pub move_dir: Vec2<i32>,
	pub move_time: f32,

	pub inv: PlayerInventory,
}

impl PlayerState {
	pub fn cam_pos(&self) -> Vec3<f32> {
		let mut pos = self.pos.map(|c| c as f32);
		if !self.wallhit {
			pos += self.pos_offset;
		}
		pos.vec3(0.0) * 32.0
	}

	pub fn think(&mut self, ctx: &mut ThinkContext) {
		let pl = self;
		let mut sprite = None;
		if pl.moving {
			pl.pos_offset += pl.move_dir.map(|c| c as f32 * 0.125);
			if pl.wallhit {
				pl.moving = false;
				pl.move_time = ctx.time;
				pl.pos_offset = Vec2::ZERO;
			}
			else {
				if pl.pos_offset.len() >= 1.0 {
					pl.pos_offset = Vec2::ZERO;
					pl.moving = false;
					pl.move_time = ctx.time;
					pl.pos += pl.move_dir;
				}
			}
		}
		else if ctx.field.get_tile(pl.pos).tile == Tile::Exit {
			sprite = Some(Sprite::PlayerCheer);
		}
		else {
			let time = ctx.time;
			if time > pl.move_time + 0.2 {
				sprite = Some(Sprite::PlayerWalkNeutral);
			}
			let mut wish_dir = Vec2::zero();
			let mut move_dir = None;
			if ctx.input.left {
				wish_dir.x = -1;
				move_dir = Some(Dir::Left);
				sprite = Some(Sprite::PlayerWalkLeft);
			}
			else if ctx.input.right {
				wish_dir.x = 1;
				move_dir = Some(Dir::Right);
				sprite = Some(Sprite::PlayerWalkRight);
			}
			else if ctx.input.up {
				wish_dir.y = -1;
				move_dir = Some(Dir::Up);
				sprite = Some(Sprite::PlayerWalkUp);
			}
			else if ctx.input.down {
				wish_dir.y = 1;
				move_dir = Some(Dir::Down);
				sprite = Some(Sprite::PlayerWalkDown);
			}

			if let Some(move_dir) = move_dir {
				let new_pos = pl.pos + wish_dir;
				ctx.pl.inv = pl.inv;
				let mut solid_entity = false;
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
					solid_entity |= ictx.blocking;
				}
				pl.inv = ctx.pl.inv;
				// self.field.interact(new_pos, pl);
				let wallhit = solid_entity || ctx.field.get_tile(new_pos).solid;
				pl.wallhit = wallhit;
				pl.moving = true;
				pl.move_dir = wish_dir;
			}
		}
		let player_ent = ctx.entities.get_mut(pl.entity).unwrap();
		player_ent.pos = pl.pos;
		let player_obj = ctx.objects.get_mut(pl.object).unwrap();
		if let Some(sprite) = sprite {
			player_obj.sprite = sprite;
		}
		player_obj.pos = (pl.pos.map(|c| c as f32 * 32.0) + pl.pos_offset * 32.0).vec3(0.0);
	}
}
