use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MapDto {
	pub width: i32,
	pub height: i32,
	pub data: Vec<u8>,
	pub legend: Vec<Terrain>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LevelDto {
	pub name: String,
	pub hint: String,
	pub password: String,
	pub seed: u64,
	pub time: i32,
	pub chips: i32,
	pub map: MapDto,
	pub entities: Vec<EntityArgs>,
	pub connections: Vec<Connection>,
}
