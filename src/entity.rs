use super::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct EntityHandle(pub u32);

#[derive(Copy, Clone, Debug)]
pub enum EntityKind {
	Player,
	Chip,
	Gate,
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
	pub frozen: bool,
	pub move_time: f32,
}

impl Entity {
	pub fn think(&mut self, ctx: &mut ThinkContext) -> Lifecycle {
		let think_fn = match self.kind {
			EntityKind::Player => entities::player::think,
			EntityKind::Gate => entities::gate::think,
			EntityKind::Chip => entities::chip::think,
			EntityKind::Block => entities::block::think,
			EntityKind::EnemyBug => entities::bug::think,
			EntityKind::BlueKey => entities::key::think,
			EntityKind::RedKey => entities::key::think,
			EntityKind::GreenKey => entities::key::think,
			EntityKind::YellowKey => entities::key::think,
			EntityKind::BlueDoor => entities::door::think,
			EntityKind::RedDoor => entities::door::think,
			EntityKind::GreenDoor => entities::door::think,
			EntityKind::YellowDoor => entities::door::think,
		};
		think_fn(self, ctx)
	}

	/// Player interacts with an entity by moving into it.
	pub fn interact(&mut self, ctx: &mut ThinkContext, ictx: &mut InteractContext) {
		let interact_fn = match self.kind {
			EntityKind::Player => entities::player::interact,
			EntityKind::Chip => entities::chip::interact,
			EntityKind::Gate => entities::gate::interact,
			EntityKind::Block => entities::block::interact,
			EntityKind::EnemyBug => entities::bug::interact,
			EntityKind::BlueKey => entities::key::interact,
			EntityKind::RedKey => entities::key::interact,
			EntityKind::GreenKey => entities::key::interact,
			EntityKind::YellowKey => entities::key::interact,
			EntityKind::BlueDoor => entities::door::interact,
			EntityKind::RedDoor => entities::door::interact,
			EntityKind::GreenDoor => entities::door::interact,
			EntityKind::YellowDoor => entities::door::interact,
		};
		interact_fn(self, ctx, ictx)
	}
}
