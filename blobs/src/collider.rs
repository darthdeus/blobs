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

    pub fn mass(&self) -> f32 {
        self.radius * 2.0
    }

    pub fn inertia(&self) -> f32 {
        let mass = self.mass();
        let d = self.offset.translation.length();
        let inertia = 0.5 * mass * self.radius.powi(2);

        inertia + mass * d.powi(2)
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
    time_data: Rc<TimeData>,

    arena: Arena<Collider>,

    pub group_arenas: HashMap<Group, Arena<ColliderHandle>>,
}

impl ColliderSet {
    pub fn new(time_data: Rc<TimeData>) -> Self {
        Self {
            time_data,
            arena: Arena::new(),
            group_arenas: HashMap::new(),
        }
    }

    pub fn get(&self, handle: ColliderHandle) -> Option<&Collider> {
        self.arena.get(handle.0)
    }

    pub fn get_mut(&mut self, handle: ColliderHandle) -> Option<&mut Collider> {
        self.arena.get_mut(handle.0)
    }

    pub fn get2_mut(
        &mut self,
        handle_a: ColliderHandle,
        handle_b: ColliderHandle,
    ) -> (Option<&mut Collider>, Option<&mut Collider>) {
        self.arena.get2_mut(handle_a.0, handle_b.0)
    }

    pub fn iter(&self) -> impl Iterator<Item = (ColliderHandle, &Collider)> {
        self.arena
            .iter()
            .map(|(idx, col)| (ColliderHandle(idx), col))
    }

    pub fn len(&self) -> usize {
        self.arena.len()
    }

    pub fn remove_ignoring_parent(&mut self, handle: ColliderHandle) {
        self.arena.remove(handle.0);
    }

    pub fn remove(&mut self, handle: ColliderHandle, rbd_set: &mut RigidBodySet) {
        let mut remove_rbd = false;

        if let Some(collider) = self.arena.get(handle.0) {
            if let Some(parent) = collider.parent {
                if let Some(body) = rbd_set.arena.get_mut(parent.0) {
                    body.colliders.retain(|&h| h != handle);
                    body.update_mass_and_inertia(self);

                    if body.colliders.len() == 0 {
                        remove_rbd = true;

                        push_event(Event {
                            time_data: *self.time_data,
                            position: Some(body.position),
                            message: "rbd removed because colliders.len() == 0".into(),
                            severity: Severity::Info,
                            col_handle: Some(handle),
                            rbd_handle: Some(parent),
                        });
                    }
                }

                if remove_rbd {
                    rbd_set.remove_rbd(parent);
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
        let col_group = collider.collision_groups.memberships;

        let col_handle = self.arena.insert(collider);
        let col_handle = ColliderHandle(col_handle);

        if let Some(rbd) = rbd_set.get_mut(rbd_handle) {
            rbd.colliders.push(col_handle);
        }

        for i in 0..32 {
            if col_group.intersects(Group::from_bits(1 << i).unwrap()) {
                self.group_arenas
                    .entry(col_group)
                    .or_default()
                    .insert(col_handle);
            }
        }

        // TODO: insert into collider
        // TODO: handle deletion

        col_handle
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
    use approx::assert_relative_eq;
    use std::f32::consts::PI;

    #[test]
    fn test_simple_body_transform() {
        let body = RigidBodyBuilder::new()
            .position(Vec2::new(2.0, 3.0))
            .scale(Vec2::new(1.0, 1.0))
            .build();

        let expected_transform =
            Affine2::from_mat2_translation(Mat2::IDENTITY, Vec2::new(2.0, 3.0));

        assert_relative_eq!(body.transform(), expected_transform);
    }

    #[test]
    fn test_body_transform() {
        let body = RigidBodyBuilder::new()
            .position(Vec2::new(2.0, 3.0))
            .rotation(PI / 2.0)
            .scale(Vec2::new(1.0, 1.0))
            .build();

        let expected_transform =
            Affine2::from_mat2_translation(Mat2::from_angle(PI / 2.0), Vec2::new(2.0, 3.0));

        assert_relative_eq!(body.transform().matrix2, expected_transform.matrix2);
    }

    #[test]
    fn test_collider_offset() {
        let collider = ColliderBuilder::new()
            .offset(Affine2::from_mat2_translation(
                Mat2::from_cols_array(&[1.0, 0.0, 0.0, 1.0]),
                Vec2::new(1.0, 1.0),
            ))
            .build();

        let body = RigidBodyBuilder::new()
            .position(Vec2::new(2.0, 3.0))
            .rotation(PI / 2.0)
            .scale(Vec2::new(1.0, 1.0))
            .build();

        let expected_transform = Affine2::from_mat2_translation(
            // Mat2::from_cols_array(&[0.0, -1.0, 1.0, 0.0]),
            Mat2::from_angle(PI / 2.0),
            Vec2::new(2.0, 3.0),
        ) * collider.offset;

        assert_relative_eq!(body.transform() * collider.offset, expected_transform);
    }
}
