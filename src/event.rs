use super::*;

pub enum Event {
	ChipCollected,
	KeyCollected { color: KeyColor },
	DoorOpened { color: KeyColor },
	GateOpened,
	LevelCompleted,
	PlayerStep,
	PlayerBump,
	TimeExpired,
	PlaySound { sound: &'static str },
}
