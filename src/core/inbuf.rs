use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct InputDir {
	dir: Dir,
	seen: bool,
}

/// Input buffering and socd handling.
#[derive(Clone, Debug)]
pub struct InputBuffer {
	moves: [InputDir; 4],
	nmoves: u8,
}

impl Default for InputBuffer {
	fn default() -> Self {
		Self {
			moves: [InputDir { dir: Dir::Up, seen: false }; 4],
			nmoves: 0,
		}
	}
}

impl InputBuffer {
	#[inline(never)]
	pub fn handle(&mut self, dir: Dir, is: bool, was: bool) {
		if !was && is {
			self.add_move(dir);
		}
		if /*was && */!is {
			self.remove_move(dir);
		}
	}
	fn add_move(&mut self, dir: Dir) {
		// Find the first seen move
		let mut i = 0;
		let nmoves = cmp::min(self.nmoves, 4);
		if i < nmoves && !self.moves[i as usize].seen {
			i += 1;
			if i < nmoves && !self.moves[i as usize].seen {
				i += 1;
				if i < nmoves && !self.moves[i as usize].seen {
					i += 1;
				}
			}
		}

		self.nmoves = cmp::min(nmoves + 1, 4);

		// Shift seen moves to the right
		if i <= 2 {
			self.moves[3] = self.moves[2];
			if i <= 1 {
				self.moves[2] = self.moves[1];
				if i <= 0 {
					self.moves[1] = self.moves[0];
				}
			}
		}

		// Write the new move
		self.moves[i as usize] = InputDir { dir, seen: false };
	}
	fn remove_move(&mut self, dir: Dir) {
		// Only remove the first move, if it has been seen
		if self.nmoves > 0 && self.moves[0] == (InputDir { dir, seen: true }) {
			self.moves[0] = self.moves[1];
			self.moves[1] = self.moves[2];
			self.moves[2] = self.moves[3];
			self.nmoves -= 1;
		}
	}
	pub fn read_move(&mut self) -> Option<Dir> {
		if self.nmoves > 0 {
			self.moves[0].seen = true;
			Some(self.moves[0].dir)
		}
		else {
			None
		}
	}
}
