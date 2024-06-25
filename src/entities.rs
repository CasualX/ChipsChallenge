use super::*;

pub mod player;
pub mod pickup;
pub mod gate;
pub mod block;
pub mod wall;
pub mod bug;
pub mod tank;
pub mod bomb;
pub mod pinkball;
pub mod fireball;
pub mod spawner;
pub mod fire;
pub mod thief;
pub mod glider;

pub fn create(ctx: &mut SpawnContext, e: &dto::EntityDto) {
	match e.kind {
		EntityKind::Player => player::create(ctx, e.pos[0], e.pos[1]),
		EntityKind::Chip => pickup::create(ctx, e.pos[0], e.pos[1], Pickup::Chip),
		EntityKind::Gate => gate::create(ctx, e.pos[0], e.pos[1]),
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
		EntityKind::Spawner => spawner::create(ctx, e.pos[0], e.pos[1], e.face_dir, Some(EntityKind::FireBall)),
		EntityKind::Bug => bug::create(ctx, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::Tank => tank::create(ctx, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::PinkBall => pinkball::create(ctx, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::FireBall => fireball::create(ctx, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::Glider => glider::create(ctx, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::Bomb => bomb::create(ctx, e.pos[0], e.pos[1]),
	}
}
