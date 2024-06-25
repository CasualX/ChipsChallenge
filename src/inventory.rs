use super::*;

#[derive(Copy, Clone, Default)]
pub struct PlayerInventory {
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

#[derive(Clone, Default)]
pub struct PlayerState {
	pub entity: EntityHandle,
	pub object: ObjectHandle,

	pub inv: PlayerInventory,
}
