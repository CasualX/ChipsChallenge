use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tile {
	Floor,
	Wall,
	Hint,
	Exit,
	Water,
	Dirt,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub enum Sprite {
	#[default]
	Blank,
	Floor,
	Wall,
	Chip,
	Barrier,
	Exit1,
	Exit2,
	Exit3,
	Hint,
	Water,
	Block,
	Dirt,
	PlayerCheer,
	PlayerWalkNeutral,
	PlayerWalkUp,
	PlayerWalkLeft,
	PlayerWalkDown,
	PlayerWalkRight,
	PlayerSwimNeutral,
	PlayerSwimUp,
	PlayerSwimLeft,
	PlayerSwimDown,
	PlayerSwimRight,
	BlueKey,
	RedKey,
	GreenKey,
	YellowKey,
	BlueDoor,
	RedDoor,
	GreenDoor,
	YellowDoor,
	BlueDoorFloor,
	RedDoorFloor,
	GreenDoorFloor,
	YellowDoorFloor,
	BugUp,
	BugLeft,
	BugDown,
	BugRight,
}

impl Sprite {
	pub fn index(self) -> Vec2<i32> {
		match self {
			Sprite::Blank => Vec2(1, 5),
			Sprite::Floor => Vec2(0, 0),
			Sprite::Wall => Vec2(0, 1),
			Sprite::Chip => Vec2(0, 2),
			Sprite::Barrier => Vec2(2, 2),
			Sprite::Exit1 => Vec2(3, 9),
			Sprite::Exit2 => Vec2(3, 10),
			Sprite::Exit3 => Vec2(3, 11),
			Sprite::Hint => Vec2(2, 15),
			Sprite::Water => Vec2(0, 3),
			Sprite::Block => Vec2(0, 10),
			Sprite::Dirt => Vec2(0, 11),
			Sprite::PlayerCheer => Vec2(3, 8),
			Sprite::PlayerWalkNeutral => Vec2(3, 4),
			Sprite::PlayerWalkUp => Vec2(6, 12),
			Sprite::PlayerWalkLeft => Vec2(6, 13),
			Sprite::PlayerWalkDown => Vec2(6, 14),
			Sprite::PlayerWalkRight => Vec2(6, 15),
			Sprite::PlayerSwimNeutral => Vec2(4, 14),
			Sprite::PlayerSwimUp => Vec2(4, 12),
			Sprite::PlayerSwimLeft => Vec2(4, 13),
			Sprite::PlayerSwimDown => Vec2(4, 14),
			Sprite::PlayerSwimRight => Vec2(4, 15),
			Sprite::BlueKey => Vec2(6, 4),
			Sprite::RedKey => Vec2(6, 5),
			Sprite::GreenKey => Vec2(6, 6),
			Sprite::YellowKey => Vec2(6, 7),
			Sprite::BlueDoor => Vec2(1, 6),
			Sprite::RedDoor => Vec2(1, 7),
			Sprite::GreenDoor => Vec2(1, 8),
			Sprite::YellowDoor => Vec2(1, 9),
			Sprite::BlueDoorFloor => Vec2(4, 6),
			Sprite::RedDoorFloor => Vec2(4, 7),
			Sprite::GreenDoorFloor => Vec2(4, 8),
			Sprite::YellowDoorFloor => Vec2(4, 9),
			Sprite::BugUp => Vec2(4, 0),
			Sprite::BugLeft => Vec2(4, 1),
			Sprite::BugDown => Vec2(4, 2),
			Sprite::BugRight => Vec2(4, 3),
		}
	}
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Model {
	Empty,
	Floor,
	Wall,
	WallV2,
	Sprite,
	SpriteShadow,
	FlatSprite,
	Portal,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Animation {
	None,
	Rise,
	Fade,
	Fall,
}
