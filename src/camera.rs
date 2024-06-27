use super::*;

#[derive(Default)]
pub struct Camera {
	// Object to follow with the camera
	pub object_h: Option<ObjectHandle>,

	/// Look at target
	pub target: Vec3<f32>,
	pub target_fast: Vec3<f32>,
	/// Eye offset from the target
	pub eye_offset: Vec3<f32>,

	// Camera matrices
	pub view_mat: Mat4<f32>,
	pub proj_mat: Mat4<f32>,
	pub view_proj_mat: Mat4<f32>,
}

impl Game {
	pub fn set_game_camera(&mut self) {
		let size = self.resources.screen_size;

		let ent_pos = if let Some(obj) = self.cam.object_h.and_then(|h| self.objects.get(h)) {
			obj.pos
		}
		else {
			self.cam.target
		};

		self.cam.target_fast = self.cam.target_fast.exp_decay(ent_pos, 25.0, 1.0 / 60.0);
		self.cam.target = self.cam.target.exp_decay(ent_pos, 15.0, 1.0 / 60.0).with_x(self.cam.target_fast.x);

		self.cam.proj_mat = cvmath::Mat4::perspective_fov(cvmath::Deg(45.0), size.x as f32, size.y as f32, 0.1, 2000.0, (cvmath::RH, cvmath::NO));
		self.cam.view_mat = {
			let eye = self.cam.target + self.cam.eye_offset;
			let target = self.cam.target_fast;
			let up = cvmath::Vec3(0.0, -1.0, 0.0);
			cvmath::Mat4::look_at(eye, target, up, cvmath::RH)
		};

		self.cam.view_proj_mat = self.cam.proj_mat * self.cam.view_mat;
	}
}
