use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TileProps {
	pub sprite: Sprite,
	pub model: Model,
}

pub static TILE_PROPS: [TileProps; 43] = [
	TileProps { sprite: Sprite::Blank, model: Model::Empty }, // Terrain::Blank
	TileProps { sprite: Sprite::Floor, model: Model::Floor }, // Terrain::Floor
	TileProps { sprite: Sprite::Wall, model: Model::Wall }, // Terrain::Wall
	TileProps { sprite: Sprite::Floor, model: Model::Floor }, // Terrain::Socket
	TileProps { sprite: Sprite::BlueLock, model: Model::Wall }, // Terrain::BlueLock
	TileProps { sprite: Sprite::RedLock, model: Model::Wall }, // Terrain::RedLock
	TileProps { sprite: Sprite::GreenLock, model: Model::Wall }, // Terrain::GreenLock
	TileProps { sprite: Sprite::YellowLock, model: Model::Wall }, // Terrain::YellowLock
	TileProps { sprite: Sprite::Hint, model: Model::Floor }, // Terrain::Hint
	TileProps { sprite: Sprite::Exit1, model: Model::Portal }, // Terrain::Exit
	TileProps { sprite: Sprite::Water, model: Model::Floor }, // Terrain::Water
	TileProps { sprite: Sprite::Floor, model: Model::Floor }, // Terrain::Fire
	TileProps { sprite: Sprite::Dirt, model: Model::Floor }, // Terrain::Dirt
	TileProps { sprite: Sprite::Gravel, model: Model::Floor }, // Terrain::Gravel
	TileProps { sprite: Sprite::Ice, model: Model::Floor }, // Terrain::Ice
	TileProps { sprite: Sprite::IceUL, model: Model::Floor }, // Terrain::IceNW
	TileProps { sprite: Sprite::IceUR, model: Model::Floor }, // Terrain::IceNE
	TileProps { sprite: Sprite::IceDL, model: Model::Floor }, // Terrain::IceSW
	TileProps { sprite: Sprite::IceDR, model: Model::Floor }, // Terrain::IceSE
	TileProps { sprite: Sprite::ForceUp, model: Model::Floor }, // Terrain::ForceN
	TileProps { sprite: Sprite::ForceLeft, model: Model::Floor }, // Terrain::ForceW
	TileProps { sprite: Sprite::ForceDown, model: Model::Floor }, // Terrain::ForceS
	TileProps { sprite: Sprite::ForceRight, model: Model::Floor }, // Terrain::ForceE
	TileProps { sprite: Sprite::ForceRandom, model: Model::Floor }, // Terrain::ForceRandom
	TileProps { sprite: Sprite::CloneMachine, model: Model::Wall }, // Terrain::CloneMachine
	TileProps { sprite: Sprite::OnOffFloor, model: Model::Floor }, // Terrain::ToggleFloor
	TileProps { sprite: Sprite::OnOffWall, model: Model::Wall }, // Terrain::ToggleWall
	TileProps { sprite: Sprite::PanelNorth, model: Model::Floor }, // Terrain::PanelN
	TileProps { sprite: Sprite::PanelWest, model: Model::Floor }, // Terrain::PanelW
	TileProps { sprite: Sprite::PanelSouth, model: Model::Floor }, // Terrain::PanelS
	TileProps { sprite: Sprite::PanelEast, model: Model::Floor }, // Terrain::PanelE
	TileProps { sprite: Sprite::PanelSE, model: Model::Floor }, // Terrain::PanelSE
	TileProps { sprite: Sprite::HiddenWall, model: Model::Wall }, // Terrain::HiddenWall
	TileProps { sprite: Sprite::InvisWall, model: Model::Wall }, // Terrain::InvisWall
	TileProps { sprite: Sprite::BlueWall, model: Model::Wall }, // Terrain::BlueWall
	TileProps { sprite: Sprite::BlueWallFake, model: Model::Wall }, // Terrain::BlueFake
	TileProps { sprite: Sprite::GreenSwitch, model: Model::Floor }, // Terrain::GreenButton
	TileProps { sprite: Sprite::RedSwitch, model: Model::Floor }, // Terrain::RedButton
	TileProps { sprite: Sprite::BrownSwitch, model: Model::Floor }, // Terrain::BrownButton
	TileProps { sprite: Sprite::BlueSwitch, model: Model::Floor }, // Terrain::BlueButton
	TileProps { sprite: Sprite::Teleport, model: Model::Floor }, // Terrain::Teleport
	TileProps { sprite: Sprite::BearTrap, model: Model::Floor }, // Terrain::BearTrap
	TileProps { sprite: Sprite::RecessedWall, model: Model::Floor }, // Terrain::RecessedWall
];
