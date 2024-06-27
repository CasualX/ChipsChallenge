use super::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct EntityHandle(pub u32);

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EntityKind {
	Sprite,
	Player,
	Chip,
	Socket,
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
	Thief,
	Fire,
	Bug,
	Tank,
	PinkBall,
	FireBall,
	Glider,
	Bomb,
}

#[derive(Copy, Clone, Debug)]
pub struct Entity {
	pub handle: EntityHandle,
	pub kind: EntityKind,
	pub pos: Vec2<i32>,
	pub move_dir: Option<Dir>,
	pub move_spd: f32,
	pub move_time: f32,
	pub face_dir: Option<Dir>,
	pub trapped: bool,
	pub destroy: bool,
}

impl Entity {
	pub fn think(&mut self, ctx: &mut ThinkContext) {
		let think_fn = match self.kind {
			EntityKind::Sprite => entities::sprite::think,
			EntityKind::Player => entities::player::think,
			EntityKind::Chip => entities::pickup::think,
			EntityKind::Socket => entities::socket::think,
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
			EntityKind::Fire => entities::fire::think,
			EntityKind::Thief => entities::thief::think,
			EntityKind::Bug => entities::bug::think,
			EntityKind::Tank => entities::tank::think,
			EntityKind::PinkBall => entities::pinkball::think,
			EntityKind::FireBall => entities::fireball::think,
			EntityKind::Glider => entities::glider::think,
			EntityKind::Bomb => entities::bomb::think,
		};
		think_fn(self, ctx)
	}

	/// Player interacts with an entity by moving into it.
	pub fn interact(&mut self, ctx: &mut ThinkContext, ictx: &mut InteractContext) {
		let interact_fn = match self.kind {
			EntityKind::Sprite => entities::sprite::interact,
			EntityKind::Player => entities::player::interact,
			EntityKind::Chip => entities::pickup::interact,
			EntityKind::Socket => entities::socket::interact,
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
			EntityKind::Fire => entities::fire::interact,
			EntityKind::Thief => entities::thief::interact,
			EntityKind::Bug => entities::bug::interact,
			EntityKind::Tank => entities::tank::interact,
			EntityKind::PinkBall => entities::pinkball::interact,
			EntityKind::FireBall => entities::fireball::interact,
			EntityKind::Glider => entities::glider::interact,
			EntityKind::Bomb => entities::bomb::interact,
		};
		interact_fn(self, ctx, ictx)
	}
}
