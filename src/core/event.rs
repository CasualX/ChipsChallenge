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
#[repr(u8)]
pub enum KeyColor {
	Blue,
	Red,
	Green,
	Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEvent {
	EntityCreated { entity: EntityHandle },
	EntityRemoved { entity: EntityHandle },
	EntityStep { entity: EntityHandle },
	EntityFaceDir { entity: EntityHandle },
	EntityTeleport { entity: EntityHandle },
	EntityHidden { entity: EntityHandle, hidden: bool },
	PlayerAction { player: EntityHandle },
	PlayerHint { player: EntityHandle, pos: Vec2i },
	ItemPickup { player: EntityHandle, kind: Pickup },
	SocketFilled { pos: Vec2i },
	ItemsThief { player: EntityHandle },
	LockRemoved { pos: Vec2i, key: KeyColor },
	BlueWallBumped { pos: Vec2i },
	BlueWallCleared { pos: Vec2i },
	HiddenWallBumped { pos: Vec2i },
	RecessedWallRaised { pos: Vec2i },
	GreenButton { entity: EntityHandle },
	RedButton { entity: EntityHandle },
	GameWin { player: EntityHandle },
	GameOver { player: EntityHandle },
}
