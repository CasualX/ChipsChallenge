use super::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct EntityHandle(pub u32);

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EntityKind {
	Player,
	Chip,
	Socket,
	Block,
	Flippers,
	FireBoots,
	IceSkates,
	SuctionBoots,
	BlueKey,
	RedKey,
	GreenKey,
	YellowKey,
	Thief,
	Bug,
	Tank,
	PinkBall,
	FireBall,
	Glider,
	Bomb,
}

#[derive(Debug)]
pub struct EntityFuncs {
	pub think: fn(&mut Entity, &mut GameState),
	pub interact: fn(&mut Entity, &mut GameState, &mut InteractContext),
}

#[derive(Clone, Debug)]
pub struct Entity {
	pub funcs: &'static EntityFuncs,
	pub handle: EntityHandle,
	pub kind: EntityKind,
	pub pos: Vec2i,
	pub move_dir: Option<Dir>,
	pub move_spd: i32,
	pub move_time: i32,
	pub face_dir: Option<Dir>,
	pub trapped: bool,
	pub remove: bool,
}
