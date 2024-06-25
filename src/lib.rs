use std::collections::HashMap;
use std::mem;

use cvmath::*;

mod dto;
mod sprites;
mod render;

mod game;
mod object;
mod event;
mod inventory;
mod entity;
mod camera;
mod entities;
mod editor;
mod terrain;
mod tile;

use self::sprites::*;
use self::object::*;
use self::inventory::*;
use self::entity::*;
use self::camera::*;
pub use self::editor::*;
pub use self::terrain::*;
pub use self::tile::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Lifecycle {
	KeepAlive,
	Destroy,
}

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
pub enum Dir {
	Up,
	Left,
	Down,
	Right,
}
impl Dir {
	pub fn turn_left(self) -> Dir {
		match self {
			Dir::Up => Dir::Left,
			Dir::Left => Dir::Down,
			Dir::Down => Dir::Right,
			Dir::Right => Dir::Up,
		}
	}
	pub fn turn_right(self) -> Dir {
		match self {
			Dir::Up => Dir::Right,
			Dir::Left => Dir::Up,
			Dir::Down => Dir::Left,
			Dir::Right => Dir::Down,
		}
	}
	pub fn turn_around(self) -> Dir {
		match self {
			Dir::Up => Dir::Down,
			Dir::Left => Dir::Right,
			Dir::Down => Dir::Up,
			Dir::Right => Dir::Left,
		}
	}
	pub fn to_vec(self) -> Vec2<i32> {
		match self {
			Dir::Up => Vec2::new(0, -1),
			Dir::Left => Vec2::new(-1, 0),
			Dir::Down => Vec2::new(0, 1),
			Dir::Right => Vec2::new(1, 0),
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KeyColor {
	Blue,
	Red,
	Green,
	Yellow,
}

const SOLID_WALL: u8 = 0xf;
const PANEL_N: u8 = 0x1;
const PANEL_E: u8 = 0x2;
const PANEL_S: u8 = 0x4;
const PANEL_W: u8 = 0x8;

#[derive(Default)]
pub struct Field {
	pub name: String,
	pub hint: String,
	pub password: String,
	pub time_limit: i32,
	pub chips: i32,
	pub width: i32,
	pub height: i32,
	pub map: Vec<Terrain>,
}
pub struct CanMoveFlags {
	pub gravel: bool,
	pub fire: bool,
}
impl Field {
	pub fn get_terrain(&self, pos: Vec2<i32>) -> Terrain {
		let Vec2 { x, y } = pos;
		if x < 0 || y < 0 || x >= self.width || y >= self.height {
			return Terrain::Blank;
		}
		let index = (y * self.width + x) as usize;
		self.map.get(index).cloned().unwrap_or(Terrain::Blank)
	}
	pub fn set_terrain(&mut self, pos: Vec2<i32>, terrain: Terrain) {
		let Vec2 { x, y } = pos;
		if x < 0 || y < 0 || x >= self.width || y >= self.height {
			return;
		}
		let index = (y * self.width + x) as usize;
		if let Some(ptr) = self.map.get_mut(index) {
			*ptr = terrain;
		}
	}
	pub fn can_move(&self, pos: Vec2<i32>, dir: Dir, flags: &CanMoveFlags) -> bool {
		let cur = self.get_terrain(pos);
		let cur = TILE_PROPS[cur as usize];

		// Allow movement if the tile is solid
		if cur.solid == SOLID_WALL {
			return true;
		}

		// Check for panels on the current tile
		let panel = match dir {
			Dir::Up => PANEL_N,
			Dir::Left => PANEL_W,
			Dir::Down => PANEL_S,
			Dir::Right => PANEL_E,
		};
		if cur.solid & panel != 0 {
			return false;
		}

		let next = self.get_terrain(pos + dir.to_vec());
		let next = TILE_PROPS[next as usize];

		// Check the solid flags of the next tile
		let panel = match dir {
			Dir::Up => PANEL_S,
			Dir::Left => PANEL_E,
			Dir::Down => PANEL_N,
			Dir::Right => PANEL_W,
		};
		if next.solid & panel != 0 {
			return false;
		}

		if !flags.gravel && next.terrain == Terrain::Gravel {
			return false;
		}
		if !flags.fire && next.terrain == Terrain::Fire {
			return false;
		}

		return true;
	}
}

#[derive(Default)]
pub struct Resources {
	pub tileset: shade::Texture2D,
	pub tileset_size: Vec2<i32>,
	pub shader: shade::Shader,
	pub screen_size: Vec2<i32>,
}

#[derive(Default)]
pub struct ObjectMap {
	pub map: HashMap<ObjectHandle, Object>,
	pub next: ObjectHandle,
}
impl ObjectMap {
	pub fn alloc(&mut self) -> ObjectHandle {
		self.next.0 += 1;
		return self.next;
	}
	pub fn create(&mut self, obj: Object) -> ObjectHandle {
		self.next.0 += 1;
		let handle = self.next;
		self.map.insert(handle, Object { handle, ..obj });
		return handle;
	}
	pub fn insert(&mut self, obj: Object) {
		assert_ne!(obj.handle.0, 0, "Object handle is zero, use alloc() or create() to allocate a new handle.");
		self.map.insert(obj.handle, obj);
	}
	pub fn get(&self, handle: ObjectHandle) -> Option<&Object> {
		self.map.get(&handle)
	}
	pub fn get_mut(&mut self, handle: ObjectHandle) -> Option<&mut Object> {
		self.map.get_mut(&handle)
	}
	pub fn remove(&mut self, handle: ObjectHandle) -> Option<Object> {
		self.map.remove(&handle)
	}
	pub fn with<F: FnMut(&mut Object)>(&mut self, handle: ObjectHandle, mut f: F) -> bool {
		if let Some(mut ent) = self.map.remove(&handle) {
			f(&mut ent);
			self.map.insert(ent.handle, ent);
			true
		}
		else {
			false
		}
	}
	pub fn find_handle(&self, kind: EntityKind) -> Option<ObjectHandle> {
		for ent in self.map.values() {
			if ent.entity_kind == kind {
				return Some(ent.handle);
			}
		}
		None
	}
}

#[derive(Default)]
pub struct EntityMap {
	pub map: HashMap<EntityHandle, Entity>,
	pub next: EntityHandle,
}
impl EntityMap {
	pub fn alloc(&mut self) -> EntityHandle {
		self.next.0 += 1;
		return self.next;
	}
	pub fn create(&mut self, ent: Entity) -> EntityHandle {
		self.next.0 += 1;
		let handle = self.next;
		self.map.insert(handle, Entity { handle, ..ent });
		return handle;
	}
	pub fn insert(&mut self, ent: Entity) {
		assert_ne!(ent.handle.0, 0, "Entity handle is zero, use alloc() or create() to allocate a new handle.");
		self.map.insert(ent.handle, ent);
	}
	pub fn get(&self, handle: EntityHandle) -> Option<&Entity> {
		self.map.get(&handle)
	}
	pub fn get_mut(&mut self, handle: EntityHandle) -> Option<&mut Entity> {
		self.map.get_mut(&handle)
	}
	pub fn remove(&mut self, handle: EntityHandle) -> Option<Entity> {
		self.map.remove(&handle)
	}
	pub fn with<F: FnMut(&mut Entity)>(&mut self, handle: EntityHandle, mut f: F) -> bool {
		if let Some(mut ent) = self.map.remove(&handle) {
			f(&mut ent);
			self.map.insert(ent.handle, ent);
			true
		}
		else {
			false
		}
	}
	pub fn find_handle(&self, kind: EntityKind) -> Option<EntityHandle> {
		for ent in self.map.values() {
			if ent.kind == kind {
				return Some(ent.handle);
			}
		}
		None
	}
}

#[derive(Default)]
pub struct Game {
	time: i64,
	resources: Resources,
	pl: PlayerState,
	cam: Camera,
	field: Field,
	input: Input,
	pub objects: ObjectMap,
	pub entities: EntityMap,
}

#[derive(Copy, Clone, Default)]
pub struct Input {
	pub a: bool,
	pub b: bool,
	pub left: bool,
	pub right: bool,
	pub up: bool,
	pub down: bool,
}

#[derive(Debug)]
pub struct InteractContext {
	pub remove_entity: bool,
	pub blocking: bool,
	pub push_dir: Dir,
}

pub struct SpawnContext {
	pub objects: ObjectMap,
	pub entities: EntityMap,
}

impl SpawnContext {
	pub fn begin(objects: &mut ObjectMap, entities: &mut EntityMap) -> SpawnContext {
		SpawnContext {
			objects: mem::replace(objects, Default::default()),
			entities: mem::replace(entities, Default::default()),
		}
	}
	pub fn end(self, objects: &mut ObjectMap, entities: &mut EntityMap) {
		mem::forget(mem::replace(objects, self.objects));
		mem::forget(mem::replace(entities, self.entities));
	}
}

pub struct ThinkContext {
	pub time: f32,
	pub dt: f32,
	pub input: Input,
	pub events: Vec<event::Event>,
	pub pl: PlayerState,
	pub field: Field,
	pub objects: ObjectMap,
	pub entities: EntityMap,
}

impl Game {
	pub fn init(&mut self, resx: Resources) {
		self.resources = resx;
	}
	pub fn think(&mut self, input: &Input, events: &mut Vec<event::Event>) {
		let mut ctx = ThinkContext {
			time: self.time as f32 / 60.0,
			dt: 1.0 / 60.0,
			input: input.clone(),
			events: Vec::new(),
			pl: self.pl.clone(),
			field: mem::replace(&mut self.field, Default::default()),
			objects: mem::replace(&mut self.objects, Default::default()),
			entities: mem::replace(&mut self.entities, Default::default()),
		};
		// self.pl.think(&mut ctx);
		ctx.pl = self.pl.clone();
		for handle in ctx.entities.map.keys().cloned().collect::<Vec<_>>() {
			// ctx.entities.with(handle, |ent| ent.think(&mut ctx));
			let Some(mut ent) = ctx.entities.remove(handle) else { continue };
			if matches!(ent.think(&mut ctx), Lifecycle::KeepAlive) {
				ctx.entities.insert(ent);
			}
		}
		self.pl = ctx.pl.clone();
		for handle in ctx.objects.map.keys().cloned().collect::<Vec<_>>() {
			let Some(mut obj) = ctx.objects.remove(handle) else { continue };
			obj.update(&mut ctx);
			ctx.objects.insert(obj);
		}
		self.time += 1;
		_ = mem::replace(&mut self.field, ctx.field);
		_ = mem::replace(&mut self.objects, ctx.objects);
		_ = mem::replace(&mut self.entities, ctx.entities);
		events.append(&mut ctx.events);
		self.cam.object_h = Some(self.pl.object);
		self.input = input.clone();
	}
	pub fn render(&mut self, g: &mut shade::Graphics) {
		let time = self.time as f32 / 60.0;
		let size = self.resources.screen_size;

		g.begin().unwrap();

		// Clear the screen
		g.clear(&shade::ClearArgs {
			surface: shade::Surface::BACK_BUFFER,
			color: Some(cvmath::Vec4(0.2, 0.2, 0.5, 1.0)),
			depth: Some(1.0),
			..Default::default()
		}).unwrap();

		self.set_game_camera();

		let mut cv = shade::d2::Canvas::<render::Vertex, render::Uniform>::new();
		cv.shader = self.resources.shader;
		cv.depth_test = Some(shade::DepthTest::Less);
		cv.viewport = cvmath::Rect::vec(cvmath::Vec2(size.x as i32, size.y as i32));
		// cv.cull_mode = Some(shade::CullMode::CW);
		cv.push_uniform(render::Uniform { transform: self.cam.view_proj_mat, texture: self.resources.tileset, texture_size: self.resources.tileset_size.map(|c| c as f32).into() });
		render::field(&mut cv, self, time);
		cv.draw(g, shade::Surface::BACK_BUFFER).unwrap();

		g.end().unwrap();
	}
}
