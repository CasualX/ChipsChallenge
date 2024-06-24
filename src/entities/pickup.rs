use super::*;

pub fn create(ctx: &mut SpawnContext, x: i32, y: i32, item: Pickup) {
	let entity_h = ctx.entities.alloc();
	let object_h = ctx.objects.alloc();
	let kind = match item {
		Pickup::Chip => EntityKind::Chip,
		Pickup::BlueKey => EntityKind::BlueKey,
		Pickup::RedKey => EntityKind::RedKey,
		Pickup::GreenKey => EntityKind::GreenKey,
		Pickup::YellowKey => EntityKind::YellowKey,
		Pickup::Flippers => EntityKind::Flippers,
		Pickup::FireBoots => EntityKind::FireBoots,
		Pickup::IceSkates => EntityKind::IceSkates,
		Pickup::SuctionBoots => EntityKind::SuctionBoots,
	};
	let sprite = match item {
		Pickup::Chip => Sprite::Chip,
		Pickup::BlueKey => Sprite::BlueKey,
		Pickup::RedKey => Sprite::RedKey,
		Pickup::GreenKey => Sprite::GreenKey,
		Pickup::YellowKey => Sprite::YellowKey,
		Pickup::Flippers => Sprite::PowerFlippers,
		Pickup::FireBoots => Sprite::PowerFireBoots,
		Pickup::IceSkates => Sprite::PowerIceSkates,
		Pickup::SuctionBoots => Sprite::PowerSuctionBoots,
	};
	ctx.entities.insert(Entity {
		handle: entity_h,
		kind,
		pos: Vec2(x, y),
		move_dir: None,
		move_spd: 0.0,
		face_dir: None,
		frozen: false,
		spawner_kind: None,
		move_time: 0.0,
	});
	ctx.objects.insert(Object {
		handle: object_h,
		entity_handle: entity_h,
		entity_kind: kind,
		pos: Vec3(x as f32 * 32.0, y as f32 * 32.0, 0.0),
		vel: Vec3::ZERO,
		sprite,
		model: Model::Sprite,
		anim: Animation::None,
		atime: 0.0,
		alpha: 1.0,
		vis: true,
		live: true,
	});
}

pub fn think(_ent: &mut Entity, _ctx: &mut ThinkContext) -> Lifecycle {
	Lifecycle::KeepAlive
}

pub fn interact(ent: &mut Entity, ctx: &mut ThinkContext, ictx: &mut InteractContext) {
	match ent.kind {
		EntityKind::Chip => ctx.pl.inv.chips += 1,
		EntityKind::BlueKey => ctx.pl.inv.keys[0] += 1,
		EntityKind::RedKey => ctx.pl.inv.keys[1] += 1,
		EntityKind::GreenKey => ctx.pl.inv.keys[2] += 1,
		EntityKind::YellowKey => ctx.pl.inv.keys[3] += 1,
		EntityKind::Flippers => ctx.pl.inv.flippers = true,
		EntityKind::FireBoots => ctx.pl.inv.fire_boots = true,
		EntityKind::IceSkates => ctx.pl.inv.ice_skates = true,
		EntityKind::SuctionBoots => ctx.pl.inv.suction_boots = true,
		_ => (),
	}

	ictx.remove_entity = true;
	ictx.blocking = false;
}

pub fn update(obj: &mut Object, ctx: &mut ThinkContext) {
	let ent = ctx.entities.get(obj.entity_handle);

	obj.vis = true;
	if let Some(ent) = ent {
		for e in ctx.entities.map.values() {
			if e.pos == ent.pos && e.kind == EntityKind::Block {
				obj.vis = false;
			}
		}
	}

	if ent.is_none() {
		obj.anim = Animation::Rise;
		obj.vel = Vec3(0.0, 0.0, 200.0);
	}
}
