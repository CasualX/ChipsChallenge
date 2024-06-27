use super::*;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub enum PlayerAction {
	#[default]
	Idle,
	Walk,
	Push,
	Swim,
	Drown,
	Burn,
	Skate,
	Slide,
	Suction,
	Death,
	Win,
}

#[derive(Clone, Default)]
pub struct PlayerState {
	pub entity: EntityHandle,
	pub state: PlayerAction,

	/// Force floor direction from previous step.
	// pub force_dir: Option<Dir>,
	pub forced_move: bool,
	/// Total steps taken (for high score).
	pub steps: i32,
	pub chips: i32,
	pub keys: [u8; 4],
	pub flippers: bool,
	pub fire_boots: bool,
	pub ice_skates: bool,
	pub suction_boots: bool,
}
