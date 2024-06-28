use std::collections::HashMap;
use cvmath::Vec2i;

mod connection;
mod dir;
mod dto;
mod entity;
mod entities;
mod entitymap;
mod event;
mod field;
mod gamestate;
mod playerstate;
mod terrain;

pub use self::connection::*;
pub use self::dir::*;
pub use self::entity::*;
pub use self::entities::*;
pub use self::entitymap::*;
pub use self::event::*;
pub use self::field::*;
pub use self::gamestate::*;
pub use self::playerstate::*;
pub use self::terrain::*;

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
	pub blocking: bool,
	pub push_dir: Dir,
}

pub const SOLID_WALL: u8 = 0xf;
pub const PANEL_N: u8 = 0x1;
pub const PANEL_E: u8 = 0x2;
pub const PANEL_S: u8 = 0x4;
pub const PANEL_W: u8 = 0x8;

type Time = i32;
