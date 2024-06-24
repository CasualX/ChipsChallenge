use super::*;

#[derive(Clone, Default)]
pub struct EditorInput {
	pub mouse: Vec2<i32>,
	pub screen_size: Vec2<i32>,
	pub up: bool,
	pub left: bool,
	pub down: bool,
	pub right: bool,
	pub left_click: bool,
	pub right_click: bool,
	pub chr: Option<char>,
}

#[derive(Copy, Clone)]
enum Tool {
	Tile(u8),
	Entity(EntityKind),
}
impl Default for Tool {
	fn default() -> Self {
		Tool::Tile(1)
	}
}

#[derive(Default)]
pub struct EditorGame {
	game: Game,
	input: EditorInput,
	tool: Tool,
}

impl EditorGame {
	pub fn init(&mut self, resources: Resources) {
		self.game.init(resources);
	}
	pub fn load_level(&mut self, json: &str) {
		self.game.load_level(json);
	}
	pub fn save_level(&self) -> String {
		let mut legend_map = HashMap::new();
		let mut legend = Vec::new();
		legend_map.insert(Terrain::Blank, 0); legend.push(Terrain::Blank);
		legend_map.insert(Terrain::Floor, 1); legend.push(Terrain::Floor);
		let mut idx = 2;
		for &x in self.game.field.map.iter() {
			let tile = &self.game.field.tiles[x as usize];
			if !legend_map.contains_key(&tile.terrain) {
				legend_map.insert(tile.terrain, idx);
				legend.push(tile.terrain);
				idx += 1;
			}
		}
		let data = self.game.field.map.iter().map(|&x| legend_map[&self.game.field.tiles[x as usize].terrain]).collect();

		let dto = dto::LevelDto {
			name: self.game.field.name.clone(),
			hint: self.game.field.hint.clone(),
			password: self.game.field.password.clone(),
			time: self.game.field.time_limit,
			chips: self.game.field.chips,
			map: dto::MapDto {
				width: self.game.field.width,
				height: self.game.field.height,
				strings: vec![],
				data,
				legend,
			},
			entities: self.game.entities.map.values().map(|ent| dto::EntityDto {
				kind: ent.kind,
				pos: ent.pos,
				face_dir: ent.face_dir,
			}).collect(),
		};
		serde_json::to_string(&dto).unwrap()
	}
	pub fn render(&mut self, g: &mut shade::Graphics, input: &EditorInput) {
		if input.left {
			self.game.cam.target.x -= 5.0;
		}
		if input.right {
			self.game.cam.target.x += 5.0;
		}
		if input.up {
			self.game.cam.target.y -= 5.0;
		}
		if input.down {
			self.game.cam.target.y += 5.0;
		}

		match input.chr {
			Some('A') => self.tool = Tool::Tile(0),
			Some('B') => self.tool = Tool::Tile(1),
			Some('C') => self.tool = Tool::Tile(2),
			Some('D') => self.tool = Tool::Tile(3),
			Some('E') => self.tool = Tool::Tile(4),
			Some('F') => self.tool = Tool::Tile(5),
			Some('G') => self.tool = Tool::Tile(6),
			Some('H') => self.tool = Tool::Tile(7),
			Some('I') => self.tool = Tool::Tile(8),
			Some('J') => self.tool = Tool::Tile(9),
			Some('K') => self.tool = Tool::Tile(10),
			Some('L') => self.tool = Tool::Tile(11),
			Some('M') => self.tool = Tool::Tile(12),
			Some('N') => self.tool = Tool::Tile(13),
			Some('O') => self.tool = Tool::Tile(14),
			Some('P') => self.tool = Tool::Tile(15),
			Some('Q') => self.tool = Tool::Tile(16),
			Some('R') => self.tool = Tool::Tile(17),
			Some('S') => self.tool = Tool::Tile(18),
			Some('T') => self.tool = Tool::Tile(19),
			Some('U') => self.tool = Tool::Tile(20),
			Some('V') => self.tool = Tool::Tile(21),
			Some('W') => self.tool = Tool::Tile(22),
			Some('X') => self.tool = Tool::Tile(23),
			Some('Y') => self.tool = Tool::Tile(24),
			Some('Z') => self.tool = Tool::Tile(25),
			_ => (),
		}

		self.game.cam.eye_offset = Vec3::<f32>(0.0, 8.0 * 32.0, 400.0) * 2.0;

		self.game.render(g);

		let x = (input.mouse.x as f32 / input.screen_size.x as f32 - 0.5) * 2.0;
		let y = (input.mouse.y as f32 / input.screen_size.y as f32 - 0.5) * -2.0;

		// let pt_ndsh = Vec4::new(x, y, -1.0, 1.0);
		// let dir_eye = self.cam.proj_matrix.inverse() * pt_ndsh * 2.0;
		// // let dir_eye = dir_eye.with_w(0.0);
		// let dir_world = (self.cam.view_matrix.inverse() * dir_eye).xyz();
		// let dir = dir_world.normalize();

		// let inv = self.cam.view_proj_matrix.inverse();
		// let near = inv * Vec4::new(x, y, 0.0, 1.0);
		// let far = inv * Vec4::new(x, y, 1.0, 1.0);
		// let dir = (far.hdiv() - near.hdiv()).normalize();

		let x = x / self.game.cam.proj_mat.a11;
		let y = y / self.game.cam.proj_mat.a22;
		let dir = (self.game.cam.view_mat.inverse() * Vec4::new(x, y, -1.0, 1.0)).xyz().normalize();

		let ray = Ray::new(self.game.cam.target + self.game.cam.eye_offset, dir);
		let plane = Plane::new(Vec3::Z, 0.0);
		let mut hits = [TraceHit::default(); 2];
		let n_hits = ray.trace(&plane, &mut hits);
		// println!("ray: {:?}", ray);
		// println!("n_hits: {}, hits: {:?}", n_hits, hits[0]);

		let p = ray.at(hits[0].distance);
		let pi = p.xy().map(|c| (c / 32.0) as i32);
		if !self.input.left_click && input.left_click {
			// dbg!(dir_eye);
			println!("p: {:?}, pi: {}", p, pi);
		}

		g.begin().unwrap();
		{
			let mut cv = shade::d2::Canvas::<render::Vertex, render::Uniform>::new();
			cv.shader = self.game.resources.shader;
			cv.depth_test = Some(shade::DepthTest::Less);
			cv.viewport = cvmath::Rect::vec(cvmath::Vec2(input.screen_size.x as i32, input.screen_size.y as i32));
			cv.push_uniform(render::Uniform { transform: self.game.cam.view_proj_mat, texture: self.game.resources.tileset, texture_size: self.game.resources.tileset_size.map(|c| c as f32).into() });
			{
				let mut x = cv.begin(shade::PrimType::Triangles, 4, 2);
				x.add_indices_quad();
				let s = 2.0;
				let z = 40.0;
				x.add_vertex(render::Vertex { pos: Vec3::new(p.x-s, p.y-s, p.z), uv: Vec2::new(0.0, 0.0), color: [255, 0, 0, 255] });
				x.add_vertex(render::Vertex { pos: Vec3::new(p.x+s, p.y-s, p.z), uv: Vec2::new(1.0, 0.0), color: [255, 0, 0, 255] });
				x.add_vertex(render::Vertex { pos: Vec3::new(p.x+s, p.y+s, p.z + z), uv: Vec2::new(1.0, 1.0), color: [255, 0, 0, 255] });
				x.add_vertex(render::Vertex { pos: Vec3::new(p.x-s, p.y+s, p.z + z), uv: Vec2::new(0.0, 1.0), color: [255, 0, 0, 255] });
			}

			match self.tool {
				Tool::Tile(index) => {
					render::draw_tile(&mut cv, index, p, &self.game.field);
				}
				_ => (),
			}
			g.clear(&shade::ClearArgs {
				surface: shade::Surface::BACK_BUFFER,
				depth: Some(1.0),
				..Default::default()
			}).unwrap();
			cv.draw(g, shade::Surface::BACK_BUFFER).unwrap();
		}

		if input.left_click {
			match self.tool {
				Tool::Tile(index) => {
					self.game.field.set_tile(pi, index);
				}
				Tool::Entity(kind) => {
					// let object = Object::new(kind, pi.map(|c| c as f32 * 32.0));
					// self.game.objects.insert(object);
				}
			}
		}
		g.end().unwrap();

		self.input = input.clone();
	}
}
