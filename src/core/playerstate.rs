use super::*;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub enum PlayerAction {
	#[default]
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

	/// Current player action.
	pub action: PlayerAction,
	/// True if previous movement was involuntary.
	pub forced_move: bool,
	/// Total steps taken (for high score).
	pub steps: i32,
	/// Total chips collected.
	pub chips: i32,
	/// Keys collected.
	pub keys: [u8; 4],

	pub flippers: bool,
	pub fire_boots: bool,
	pub ice_skates: bool,
	pub suction_boots: bool,
}
