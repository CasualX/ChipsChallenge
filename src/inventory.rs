use super::*;

#[derive(Copy, Clone, Default)]
pub struct PlayerInventory {
	/// Total steps taken (for high score).
	pub steps: i32,
	pub chips: i32,
	pub keys: [u8; 4],
	pub powerups: [bool; 4],
}

#[derive(Clone, Default)]
pub struct PlayerState {
	pub entity: EntityHandle,
	pub object: ObjectHandle,

	pub inv: PlayerInventory,
}
