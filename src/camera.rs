use super::*;

#[derive(Default)]
pub struct Camera {
	/// Camera position before offset
	pub eye: Vec3<f32>,
	/// Camera offset from the player's position
	pub offset: Vec3<f32>,
}
