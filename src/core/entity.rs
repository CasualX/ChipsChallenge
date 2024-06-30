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
	Walker,
	Teeth,
	Bomb,
}

#[derive(Debug)]
pub struct EntityFuncs {
	pub think: fn(&mut GameState, &mut Entity),
	// pub try_move: fn(&mut GameState, &mut Entity, Dir) -> bool,
}

#[derive(Clone, Debug)]
pub struct Entity {
	pub funcs: &'static EntityFuncs,
	pub handle: EntityHandle,
	pub kind: EntityKind,
	pub pos: Vec2i,
	pub face_dir: Option<Dir>,
	pub step_dir: Option<Dir>,
	pub step_spd: Time,
	pub step_time: Time,
	/// Entity is trapped and cannot move.
	pub trapped: bool,
	/// Entity is hidden under a block.
	pub hidden: bool,
	/// Entity will be removed at the end of the current tick.
	pub remove: bool,
}
