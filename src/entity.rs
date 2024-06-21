use super::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct EntityHandle(pub u32);

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EntityKind {
	Player,
	Chip,
	Gate,
	Block,
	Wall,
	Flippers,
	FireBoots,
	IceSkates,
	SuctionBoots,
	BlueKey,
	RedKey,
	GreenKey,
	YellowKey,
	BlueDoor,
	RedDoor,
	GreenDoor,
	YellowDoor,
	EnemyBug,
	EnemyTank,
}

#[derive(Copy, Clone, Debug)]
pub struct Entity {
	pub handle: EntityHandle,
	pub kind: EntityKind,
	pub pos: Vec2<i32>,
	pub move_dir: Option<Dir>,
	pub move_spd: f32,
	pub face_dir: Option<Dir>,
	pub frozen: bool,
	pub move_time: f32,
}

impl Entity {
	pub fn think(&mut self, ctx: &mut ThinkContext) -> Lifecycle {
		let think_fn = match self.kind {
			EntityKind::Player => entities::player::think,
			EntityKind::Chip => entities::pickup::think,
			EntityKind::Gate => entities::gate::think,
			EntityKind::Block => entities::block::think,
			EntityKind::Wall => entities::wall::think,
			EntityKind::Flippers => entities::pickup::think,
			EntityKind::FireBoots => entities::pickup::think,
			EntityKind::IceSkates => entities::pickup::think,
			EntityKind::SuctionBoots => entities::pickup::think,
			EntityKind::BlueKey => entities::pickup::think,
			EntityKind::RedKey => entities::pickup::think,
			EntityKind::GreenKey => entities::pickup::think,
			EntityKind::YellowKey => entities::pickup::think,
			EntityKind::BlueDoor => entities::door::think,
			EntityKind::RedDoor => entities::door::think,
			EntityKind::GreenDoor => entities::door::think,
			EntityKind::YellowDoor => entities::door::think,
			EntityKind::EnemyBug => entities::bug::think,
			EntityKind::EnemyTank => entities::tank::think,
		};
		think_fn(self, ctx)
	}

	/// Player interacts with an entity by moving into it.
	pub fn interact(&mut self, ctx: &mut ThinkContext, ictx: &mut InteractContext) {
		let interact_fn = match self.kind {
			EntityKind::Player => entities::player::interact,
			EntityKind::Chip => entities::pickup::interact,
			EntityKind::Gate => entities::gate::interact,
			EntityKind::Block => entities::block::interact,
			EntityKind::Wall => entities::wall::interact,
			EntityKind::Flippers => entities::pickup::interact,
			EntityKind::FireBoots => entities::pickup::interact,
			EntityKind::IceSkates => entities::pickup::interact,
			EntityKind::SuctionBoots => entities::pickup::interact,
			EntityKind::BlueKey => entities::pickup::interact,
			EntityKind::RedKey => entities::pickup::interact,
			EntityKind::GreenKey => entities::pickup::interact,
			EntityKind::YellowKey => entities::pickup::interact,
			EntityKind::BlueDoor => entities::door::interact,
			EntityKind::RedDoor => entities::door::interact,
			EntityKind::GreenDoor => entities::door::interact,
			EntityKind::YellowDoor => entities::door::interact,
			EntityKind::EnemyBug => entities::bug::interact,
			EntityKind::EnemyTank => entities::tank::interact,
		};
		interact_fn(self, ctx, ictx)
	}
}
