use super::*;

pub const BASE_SPD: f32 = 0.1875;

pub mod sprite;
pub mod player;
pub mod pickup;
pub mod socket;
pub mod block;
pub mod wall;
pub mod bug;
pub mod tank;
pub mod bomb;
pub mod pinkball;
pub mod fireball;
pub mod fire;
pub mod thief;
pub mod glider;

pub fn create(ctx: &mut SpawnContext, e: &dto::EntityDto) -> EntityHandle {
	match e.kind {
		EntityKind::Sprite => unimplemented!("Sprite entity kind is not implemented"),
		EntityKind::Player => player::create(ctx, e.pos[0], e.pos[1]),
		EntityKind::Chip => pickup::create(ctx, e.pos[0], e.pos[1], Pickup::Chip),
		EntityKind::Socket => socket::create(ctx, e.pos[0], e.pos[1]),
		EntityKind::Block => block::create(ctx, e.pos[0], e.pos[1]),
		EntityKind::Wall => wall::create(ctx, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::Flippers => pickup::create(ctx, e.pos[0], e.pos[1], Pickup::Flippers),
		EntityKind::FireBoots => pickup::create(ctx, e.pos[0], e.pos[1], Pickup::FireBoots),
		EntityKind::IceSkates => pickup::create(ctx, e.pos[0], e.pos[1], Pickup::IceSkates),
		EntityKind::SuctionBoots => pickup::create(ctx, e.pos[0], e.pos[1], Pickup::SuctionBoots),
		EntityKind::BlueKey => pickup::create(ctx, e.pos[0], e.pos[1], Pickup::BlueKey),
		EntityKind::RedKey => pickup::create(ctx, e.pos[0], e.pos[1], Pickup::RedKey),
		EntityKind::GreenKey => pickup::create(ctx, e.pos[0], e.pos[1], Pickup::GreenKey),
		EntityKind::YellowKey => pickup::create(ctx, e.pos[0], e.pos[1], Pickup::YellowKey),
		EntityKind::Fire => fire::create(ctx, e.pos[0], e.pos[1]),
		EntityKind::Thief => thief::create(ctx, e.pos[0], e.pos[1]),
		EntityKind::Bug => bug::create(ctx, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::Tank => tank::create(ctx, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::PinkBall => pinkball::create(ctx, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::FireBall => fireball::create(ctx, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::Glider => glider::create(ctx, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::Bomb => bomb::create(ctx, e.pos[0], e.pos[1]),
	}
}

pub fn press_green_button(ctx: &mut ThinkContext) {
	for ptr in ctx.field.map.iter_mut() {
		if *ptr == Terrain::ToggleFloor {
			*ptr = Terrain::ToggleWall;
		}
		else if *ptr == Terrain::ToggleWall {
			*ptr = Terrain::ToggleFloor;
		}
	}
}

pub fn press_red_button(ctx: &mut ThinkContext, pos: Vec2<i32>) {
	let Some(conn) = ctx.field.conns.iter().cloned().find(|conn| conn.src == pos) else { return };
	if let Some(template_ent) = ctx.entities.map.values().find(|ent| ent.pos == conn.dest) {
		let ent_dto = dto::EntityDto {
			kind: template_ent.kind,
			pos: template_ent.pos,
			face_dir: template_ent.face_dir,
		};
		let mut spawn_ctx = SpawnContext::begin(&mut ctx.objects, &mut ctx.entities);
		let h = create(&mut spawn_ctx, &ent_dto);
		if let Some(ent) = spawn_ctx.entities.get_mut(h) {
			ent.move_dir = ent_dto.face_dir;
		}
		spawn_ctx.end(&mut ctx.objects, &mut ctx.entities);
	}
}

pub fn press_brown_button(ctx: &mut ThinkContext, pos: Vec2<i32>) {
	let Some(conn) = ctx.field.conns.iter().find(|conn| conn.src == pos) else { return };
	for ent in ctx.entities.map.values_mut() {
		if ent.pos == conn.dest {
			ent.trapped = false;
		}
	}
}

pub fn press_blue_button(ctx: &mut ThinkContext) {
	for other in ctx.entities.map.values_mut() {
		if other.kind == EntityKind::Tank {
			if let Some(face_dir) = other.face_dir {
				other.face_dir = Some(face_dir.turn_around());
			}
		}
	}
}
