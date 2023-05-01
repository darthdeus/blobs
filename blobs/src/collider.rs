use crate::*;

#[derive(Debug)]
pub struct Collider {
    pub offset: Vec2,
    pub rotation: f32,
    pub scale: Vec2,

    pub user_data: u128,
    pub parent: Option<RigidBodyHandle>,

    pub radius: f32,

    pub flags: ColliderFlags,

    pub absolute_position: Vec2,
    pub collision_groups: InteractionGroups,

    pub shape: Box<dyn Shape>,
}

impl Collider {
    pub fn translation(&self) -> Vec2 {
        self.absolute_position
    }

    pub fn shape(&self) -> &dyn Shape {
        &*self.shape
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct ColliderHandle(pub Index);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// A set of flags for controlling collision/intersection filtering,
/// modification, and events.
pub struct ColliderFlags {
    pub is_sensor: bool,
    // pub active_collision_types: ActiveCollisionTypes,
    // pub collision_groups: InteractionGroups,
    // pub solver_groups: InteractionGroups,
    // pub active_hooks: ActiveHooks,
    // pub active_events: ActiveEvents,
}

impl Default for ColliderFlags {
    fn default() -> Self {
        Self {
            is_sensor: false,
            // active_collision_types: ActiveCollisionTypes::default(),
            // collision_groups: InteractionGroups::all(),
            // solver_groups: InteractionGroups::all(),
            // active_hooks: ActiveHooks::empty(),
            // active_events: ActiveEvents::empty(),
        }
    }
}

pub struct ColliderSet {
    pub arena: Arena<Collider>,
}

impl ColliderSet {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
        }
    }

    pub fn get(&self, handle: ColliderHandle) -> Option<&Collider> {
        self.arena.get(handle.0)
    }

    pub fn get_mut(&mut self, handle: ColliderHandle) -> Option<&mut Collider> {
        self.arena.get_mut(handle.0)
    }

    pub fn len(&self) -> usize {
        self.arena.len()
    }

    pub fn remove(&mut self, handle: ColliderHandle) {
        self.arena.remove(handle.0);
    }

    pub fn insert_with_parent(
        &mut self,
        collider: Collider,
        rbd_handle: RigidBodyHandle,
        rbd_set: &mut RigidBodySet,
    ) -> ColliderHandle {
        let col_handle = self.arena.insert(collider);

        if let Some(rbd) = rbd_set.get_mut(rbd_handle) {
            rbd.colliders.push(ColliderHandle(col_handle));
        }
        // TODO: insert into rbd

        ColliderHandle(col_handle)
    }
}

pub struct ColliderBuilder {}
