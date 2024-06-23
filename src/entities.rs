use super::*;

pub mod player;
pub mod pickup;
pub mod door;
pub mod gate;
pub mod block;
pub mod wall;
pub mod bug;
pub mod tank;
pub mod bomb;
pub mod pink_ball;
pub mod spawner;
pub mod fire;
pub mod thief;

pub fn create(game: &mut Game, e: &dto::EntityDto) {
	match e.kind {
		EntityKind::Player => player::create(game, e.pos[0], e.pos[1]),
		EntityKind::Chip => pickup::create(game, e.pos[0], e.pos[1], Pickup::Chip),
		EntityKind::Gate => gate::create(game, e.pos[0], e.pos[1]),
		EntityKind::Block => block::create(game, e.pos[0], e.pos[1]),
		EntityKind::Wall => wall::create(game, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::Flippers => pickup::create(game, e.pos[0], e.pos[1], Pickup::Flippers),
		EntityKind::FireBoots => pickup::create(game, e.pos[0], e.pos[1], Pickup::FireBoots),
		EntityKind::IceSkates => pickup::create(game, e.pos[0], e.pos[1], Pickup::IceSkates),
		EntityKind::SuctionBoots => pickup::create(game, e.pos[0], e.pos[1], Pickup::SuctionBoots),
		EntityKind::BlueKey => pickup::create(game, e.pos[0], e.pos[1], Pickup::BlueKey),
		EntityKind::RedKey => pickup::create(game, e.pos[0], e.pos[1], Pickup::RedKey),
		EntityKind::GreenKey => pickup::create(game, e.pos[0], e.pos[1], Pickup::GreenKey),
		EntityKind::YellowKey => pickup::create(game, e.pos[0], e.pos[1], Pickup::YellowKey),
		EntityKind::BlueDoor => door::create(game, e.pos[0], e.pos[1], KeyColor::Blue),
		EntityKind::RedDoor => door::create(game, e.pos[0], e.pos[1], KeyColor::Red),
		EntityKind::GreenDoor => door::create(game, e.pos[0], e.pos[1], KeyColor::Green),
		EntityKind::YellowDoor => door::create(game, e.pos[0], e.pos[1], KeyColor::Yellow),
		EntityKind::Fire => fire::create(game, e.pos[0], e.pos[1]),
		EntityKind::Thief => thief::create(game, e.pos[0], e.pos[1]),
		EntityKind::Spawner => spawner::create(game, e.pos[0], e.pos[1], e.face_dir, e.spawner_kind),
		EntityKind::EnemyBug => bug::create(game, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::EnemyTank => tank::create(game, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::PinkBall => pink_ball::create(game, e.pos[0], e.pos[1], e.face_dir),
		EntityKind::Bomb => bomb::create(game, e.pos[0], e.pos[1]),
	}
}
