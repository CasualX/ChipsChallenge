use std::collections::HashMap;
use cvmath::*;
use crate::core;

mod camera;
mod model;
mod object;
mod objectmap;
mod sprite;
mod visualstate;
mod resources;
mod render;
mod tile;

pub use self::camera::*;
pub use self::model::*;
pub use self::object::*;
pub use self::objectmap::*;
pub use self::sprite::*;
pub use self::visualstate::*;
pub use self::resources::*;
pub use self::render::*;
pub use self::tile::*;

fn ticks_to_time(ticks: i32) -> f32 {
	ticks as f32 / 60.0
}
