use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub enum Sprite {
	#[default]
	Blank,
	Floor,
	Wall,
	Chip,
	Socket,
	Exit1,
	Exit2,
	Exit3,
	Hint,
	Water,
	Block,
	Dirt,
	Gravel,
	Fire,
	Ice,
	IceUL,
	IceUR,
	IceDL,
	IceDR,
	ForceUp,
	ForceLeft,
	ForceDown,
	ForceRight,
	ForceRandom,
	CloneMachine,
	PanelNorth,
	PanelWest,
	PanelSouth,
	PanelEast,
	PanelSE,
	PowerFlippers,
	PowerFireBoots,
	PowerIceSkates,
	PowerSuctionBoots,
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
	BlueLock,
	RedLock,
	GreenLock,
	YellowLock,
	HiddenWall,
	InvisWall,
	BlueWall,
	BlueWallFake,
	GreenSwitch,
	RedSwitch,
	BrownSwitch,
	BlueSwitch,
	OnOffWall,
	OnOffFloor,
	Bomb,
	Thief,
	Teleport,
	BearTrap,
	RecessedWall,
	BugUp,
	BugLeft,
	BugDown,
	BugRight,
	TankUp,
	TankLeft,
	TankDown,
	TankRight,
	PinkBall,
	FireBall,
	GliderUp,
	GliderLeft,
	GliderDown,
	GliderRight,
}

impl Sprite {
	pub fn index(self) -> Vec2<i32> {
		match self {
			Sprite::Blank => Vec2(1, 5),
			Sprite::Floor => Vec2(0, 0),
			Sprite::Wall => Vec2(0, 1),
			Sprite::Chip => Vec2(0, 2),
			Sprite::Socket => Vec2(2, 2),
			Sprite::Exit1 => Vec2(3, 9),
			Sprite::Exit2 => Vec2(3, 10),
			Sprite::Exit3 => Vec2(3, 11),
			Sprite::Hint => Vec2(2, 15),
			Sprite::Water => Vec2(0, 3),
			Sprite::Block => Vec2(1, 0),
			Sprite::Dirt => Vec2(0, 11),
			Sprite::Gravel => Vec2(2, 13),
			Sprite::Fire => Vec2(0, 4),
			Sprite::Ice => Vec2(0, 12),
			Sprite::IceUL => Vec2(1, 10),
			Sprite::IceUR => Vec2(1, 11),
			Sprite::IceDR => Vec2(1, 12),
			Sprite::IceDL => Vec2(1, 13),
			Sprite::ForceUp => Vec2(1, 2),
			Sprite::ForceLeft => Vec2(1, 4),
			Sprite::ForceDown => Vec2(0, 13),
			Sprite::ForceRight => Vec2(1, 3),
			Sprite::ForceRandom => Vec2(3, 2),
			Sprite::CloneMachine => Vec2(3, 1),
			Sprite::PanelNorth => Vec2(0, 6),
			Sprite::PanelWest => Vec2(0, 7),
			Sprite::PanelSouth => Vec2(0, 8),
			Sprite::PanelEast => Vec2(0, 9),
			Sprite::PanelSE => Vec2(3, 0),
			Sprite::PowerFlippers => Vec2(6, 8),
			Sprite::PowerFireBoots => Vec2(6, 9),
			Sprite::PowerIceSkates => Vec2(6, 10),
			Sprite::PowerSuctionBoots => Vec2(6, 11),
			Sprite::PlayerCheer => Vec2(3, 8),
			Sprite::PlayerWalkNeutral => Vec2(3, 4),
			Sprite::PlayerWalkUp => Vec2(6, 12),
			Sprite::PlayerWalkLeft => Vec2(6, 13),
			Sprite::PlayerWalkDown => Vec2(6, 14),
			Sprite::PlayerWalkRight => Vec2(6, 15),
			Sprite::PlayerSwimNeutral => Vec2(2, 12),
			Sprite::PlayerSwimUp => Vec2(3, 12),
			Sprite::PlayerSwimLeft => Vec2(3, 13),
			Sprite::PlayerSwimDown => Vec2(3, 14),
			Sprite::PlayerSwimRight => Vec2(3, 15),
			Sprite::BlueKey => Vec2(6, 4),
			Sprite::RedKey => Vec2(6, 5),
			Sprite::GreenKey => Vec2(6, 6),
			Sprite::YellowKey => Vec2(6, 7),
			Sprite::BlueLock => Vec2(1, 6),
			Sprite::RedLock => Vec2(1, 7),
			Sprite::GreenLock => Vec2(1, 8),
			Sprite::YellowLock => Vec2(1, 9),
			Sprite::HiddenWall => Vec2(1, 1),
			Sprite::InvisWall => Vec2(2, 0),
			Sprite::BlueWall => Vec2(1, 14),
			Sprite::BlueWallFake => Vec2(1, 15),
			Sprite::GreenSwitch => Vec2(2, 3),
			Sprite::RedSwitch => Vec2(2, 4),
			Sprite::BrownSwitch => Vec2(2, 7),
			Sprite::BlueSwitch => Vec2(2, 8),
			Sprite::OnOffWall => Vec2(2, 5),
			Sprite::OnOffFloor => Vec2(2, 6),
			Sprite::Bomb => Vec2(2, 10),
			Sprite::Thief => Vec2(2, 1),
			Sprite::Teleport => Vec2(2, 9),
			Sprite::BearTrap => Vec2(2, 11),
			Sprite::RecessedWall => Vec2(2, 14),
			Sprite::BugUp => Vec2(4, 0),
			Sprite::BugLeft => Vec2(4, 1),
			Sprite::BugDown => Vec2(4, 2),
			Sprite::BugRight => Vec2(4, 3),
			Sprite::TankUp => Vec2(4, 12),
			Sprite::TankLeft => Vec2(4, 13),
			Sprite::TankDown => Vec2(4, 14),
			Sprite::TankRight => Vec2(4, 15),
			Sprite::PinkBall => Vec2(4, 5),
			Sprite::FireBall => Vec2(4, 4),
			Sprite::GliderUp => Vec2(5, 0),
			Sprite::GliderLeft => Vec2(5, 1),
			Sprite::GliderDown => Vec2(5, 2),
			Sprite::GliderRight => Vec2(5, 3),
		}
	}
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Model {
	Empty,
	Floor,
	Wall,
	ThinWall,
	WallV2,
	Sprite,
	SpriteShadow,
	FlatSprite,
	ReallyFlatSprite,
	FloorSprite,
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
