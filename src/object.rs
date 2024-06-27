use super::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct ObjectHandle(pub u32);

#[derive(Clone, Debug)]
pub struct Object {
	pub handle: ObjectHandle,
	pub entity_handle: EntityHandle,
	pub entity_kind: EntityKind,
	pub pos: Vec3<f32>,
	pub vel: Vec3<f32>,
	pub sprite: Sprite,
	pub model: Model,
	pub anim: Animation,
	pub atime: f32,
	pub alpha: f32,
	pub vis: bool,
	pub live: bool,
}

impl Object {
	pub fn update(&mut self, ctx: &mut ThinkContext) {
		if !self.live {
			return;
		}

		let update_fn = match self.entity_kind {
			EntityKind::Sprite => entities::sprite::update,
			EntityKind::Player => entities::player::update,
			EntityKind::Chip => entities::pickup::update,
			EntityKind::Socket => entities::socket::update,
			EntityKind::Block => entities::block::update,
			EntityKind::Wall => entities::wall::update,
			EntityKind::Flippers => entities::pickup::update,
			EntityKind::FireBoots => entities::pickup::update,
			EntityKind::IceSkates => entities::pickup::update,
			EntityKind::SuctionBoots => entities::pickup::update,
			EntityKind::BlueKey => entities::pickup::update,
			EntityKind::RedKey => entities::pickup::update,
			EntityKind::GreenKey => entities::pickup::update,
			EntityKind::YellowKey => entities::pickup::update,
			EntityKind::Fire => entities::fire::update,
			EntityKind::Thief => entities::thief::update,
			EntityKind::Bug => entities::bug::update,
			EntityKind::Tank => entities::tank::update,
			EntityKind::PinkBall => entities::pinkball::update,
			EntityKind::FireBall => entities::fireball::update,
			EntityKind::Glider => entities::glider::update,
			EntityKind::Bomb => entities::bomb::update,
		};
		update_fn(self, ctx);

		self.pos += self.vel * ctx.dt;

		if matches!(self.anim, Animation::Rise | Animation::Fade) {
			if self.atime == 0.0 {
				self.atime = ctx.time;
			}
			self.alpha = f32::max(0.0, 1.0 - (ctx.time - self.atime) * 5.0);
			if self.alpha == 0.0 {
				self.vel = Vec3::ZERO;
				self.live = false;
			}
		}
		if matches!(self.anim, Animation::Fall) {
			if self.atime == 0.0 {
				self.atime = ctx.time;
			}
			if ctx.time > self.atime + 0.5 {
				self.vel = Vec3::ZERO;
				self.live = false;
			}
		}
	}
}
