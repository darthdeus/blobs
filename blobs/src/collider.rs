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

pub struct ColliderBuilder {
    offset: Affine2,
    absolute_transform: Affine2,
    user_data: u128,
    parent: Option<RigidBodyHandle>,
    radius: f32,
    flags: ColliderFlags,
    collision_groups: InteractionGroups,
    shape: Box<dyn Shape>,
}

impl ColliderBuilder {
    pub fn new() -> Self {
        Self {
            offset: Affine2::IDENTITY,
            absolute_transform: Affine2::IDENTITY,
            user_data: 0,
            parent: None,
            radius: 0.5,
            flags: ColliderFlags::default(),
            collision_groups: InteractionGroups::default(),
            shape: Box::new(Ball::new(0.5)),
        }
    }

    pub fn offset(mut self, offset: Affine2) -> Self {
        self.offset = offset;
        self
    }

    pub fn absolute_transform(mut self, absolute_transform: Affine2) -> Self {
        self.absolute_transform = absolute_transform;
        self
    }

    pub fn user_data(mut self, user_data: u128) -> Self {
        self.user_data = user_data;
        self
    }

    pub fn parent(mut self, parent: RigidBodyHandle) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn flags(mut self, flags: ColliderFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn collision_groups(mut self, collision_groups: InteractionGroups) -> Self {
        self.collision_groups = collision_groups;
        self
    }

    pub fn shape(mut self, shape: Box<dyn Shape>) -> Self {
        self.shape = shape;
        self
    }

    pub fn build(self) -> Collider {
        Collider {
            offset: self.offset,
            absolute_transform: self.absolute_transform,
            user_data: self.user_data,
            parent: self.parent,
            radius: self.radius,
            flags: self.flags,
            collision_groups: self.collision_groups,
            shape: self.shape,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_body_transform() {
//         let body = RigidBody {
//             position: Vec2::new(2.0, 3.0),
//             rotation: std::f32::consts::PI / 2.0, // 90 degrees
//             scale: Vec2::new(1.0, 1.0),
//             // ... other fields
//         };
//
//         let expected_transform = Affine2::new(
//             Matrix2::new(0.0, -1.0, 1.0, 0.0),
//             Vector2::new(2.0, 3.0),
//         );
//
//         assert_eq!(body.transform(), expected_transform);
//     }
//
//     #[test]
//     fn test_collider_offset() {
//         let collider = Collider {
//             offset: Affine2::new(
//                 Matrix2::new(1.0, 0.0, 0.0, 1.0),
//                 Vector2::new(1.0, 1.0),
//             ),
//             // ... other fields
//         };
//
//         let body = RigidBody {
//             position: Vec2::new(2.0, 3.0),
//             rotation: std::f32::consts::PI / 2.0, // 90 degrees
//             scale: Vec2::new(1.0, 1.0),
//             // ... other fields
//         };
//
//         let expected_transform = Affine2::new(
//             Matrix2::new(0.0, -1.0, 1.0, 0.0),
//             Vector2::new(2.0, 3.0),
//         ) * collider.offset;
//
//         collider.absolute_transform = body.transform() * collider.offset;
//         assert_eq!(collider.absolute_transform, expected_transform);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_transform() {
        let body = RigidBodyBuilder::new()
            .position(Vec2::new(2.0, 3.0))
            .rotation(std::f32::consts::PI / 2.0)
            .scale(Vec2::new(1.0, 1.0))
            .build();

        let expected_transform = Affine2::new(
            Matrix2::new(0.0, -1.0, 1.0, 0.0),
            Vector2::new(2.0, 3.0),
        );

        assert_eq!(body.transform(), expected_transform);
    }

    #[test]
    fn test_collider_offset() {
        let collider = ColliderBuilder::new()
            .offset(Affine2::new(
                Matrix2::new(1.0, 0.0, 0.0, 1.0),
                Vector2::new(1.0, 1.0),
            ))
            .build();

        let body = RigidBodyBuilder::new()
            .position(Vec2::new(2.0, 3.0))
            .rotation(std::f32::consts::PI / 2.0)
            .scale(Vec2::new(1.0, 1.0))
            .build();

        let expected_transform = Affine2::new(
            Matrix2::new(0.0, -1.0, 1.0, 0.0),
            Vector2::new(2.0, 3.0),
        ) * collider.offset;

        assert_eq!(body.transform() * collider.offset, expected_transform);
    }
}
