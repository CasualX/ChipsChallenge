use super::*;

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
