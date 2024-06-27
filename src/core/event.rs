use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Pickup {
	Chip,
	Flippers,
	FireBoots,
	IceSkates,
	SuctionBoots,
	BlueKey,
	RedKey,
	GreenKey,
	YellowKey,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KeyColor {
	Blue,
	Red,
	Green,
	Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEvent {
	EntityCreated { handle: EntityHandle },
	EntityRemoved { handle: EntityHandle },
	EntityMoved { handle: EntityHandle },
	EntityTeleported { handle: EntityHandle },
	PlayerActionChanged { handle: EntityHandle },
	ItemPickup { handle: EntityHandle, kind: Pickup },
	SocketFilled { pos: Vec2i },
	AllItemsCleared { handle: EntityHandle },
	LockRemoved { pos: Vec2i, key: KeyColor },
	BlueWallBumped { pos: Vec2i },
	BlueWallCleared { pos: Vec2i },
	HiddenWallBumped { pos: Vec2i },
}
