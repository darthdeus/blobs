use crate::*;

#[derive(Debug)]
pub struct Collider {
    pub offset: Affine2,
    pub absolute_transform: Affine2,

    pub user_data: u128,
    pub parent: Option<RigidBodyHandle>,

    pub radius: f32,

    pub flags: ColliderFlags,

    pub collision_groups: InteractionGroups,

    pub shape: Box<dyn Shape>,
}

impl Collider {
    pub fn relative_translation(&self) -> Vec2 {
        self.offset.translation
    }

    pub fn absolute_rotation(&self) -> f32 {
        let up = vec2(0.0, 1.0);

        self.absolute_transform
            .transform_vector2(up)
            .angle_between(up)
    }

    pub fn absolute_translation(&self) -> Vec2 {
        self.absolute_transform.translation
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

    pub fn remove_ignoring_parent(&mut self, handle: ColliderHandle) {
        self.arena.remove(handle.0);
    }

    pub fn remove(&mut self, handle: ColliderHandle, rbd_set: &mut RigidBodySet) {
        if let Some(collider) = self.arena.get(handle.0) {
            if let Some(parent) = collider.parent {
                if let Some(body) = rbd_set.arena.get_mut(parent.0) {
                    body.colliders.retain(|&h| h != handle);
                }
            }
        }

        self.remove_ignoring_parent(handle);
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
