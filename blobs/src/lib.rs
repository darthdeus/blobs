use std::sync::mpsc::Receiver;

use glam::*;
use hecs::*;

use thunderdome::{Arena, Index};

mod collider;
mod query_filter;
mod rigid_body;

pub use collider::*;
pub use query_filter::*;
pub use rigid_body::*;

pub struct Ball {
    pub radius: f32,
}

impl Shape for Ball {
    fn as_ball(&self) -> Option<&Ball> {
        Some(self)
    }

    fn as_cuboid(&self) -> Option<&Cuboid> {
        None
    }
}

impl Ball {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

pub trait Shape {
    fn as_ball(&self) -> Option<&Ball>;
    fn as_cuboid(&self) -> Option<&Cuboid>;
}

#[derive(Copy, Clone, Hash, Debug)]
/// Events occurring when two colliders start or stop colliding
pub enum CollisionEvent {
    /// Event occurring when two colliders start colliding
    Started(ColliderHandle, ColliderHandle, CollisionEventFlags),
    /// Event occurring when two colliders stop colliding.
    Stopped(ColliderHandle, ColliderHandle, CollisionEventFlags),
}

bitflags::bitflags! {
    /// Flags providing more information regarding a collision event.
    #[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
    pub struct CollisionEventFlags: u32 {
        /// Flag set if at least one of the colliders involved in the
        /// collision was a sensor when the event was fired.
        const SENSOR = 0b0001;
        /// Flag set if a `CollisionEvent::Stopped` was fired because
        /// at least one of the colliders was removed.
        const REMOVED = 0b0010;
    }
}

pub struct Rotation {}

impl Rotation {
    pub fn angle(&self) -> f32 {
        todo!()
    }
}

pub struct Aabb {
    pub mins: Vec2,
    pub maxs: Vec2,
}

impl Aabb {
    pub fn extents(&self) -> Vec2 {
        todo!()
    }
}

pub struct Cuboid {
    pub half_extents: Vec2,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct ColliderHandle(Index);

// pub struct BlobPhysics {
//     bodies: Vec<RigidBody>,
// }
//
// impl BlobPhysics {
//     pub fn step(&mut self, delta: f32) {
//         for body in self.bodies.iter_mut() {
//             body.position += body.velocity * delta;
//             body.rotation += body.angular_velocity * delta;
//         }
//     }
// }

pub struct QueryPipeline {}

impl QueryPipeline {
    pub fn new() -> Self {
        Self {}
    }

    pub fn intersection_with_shape(
        &self,
        rbd_set: &RigidBodySet,
        col_set: &ColliderSet,
        position: &Vec2,
        shape: &dyn Shape,
        filter: QueryFilter,
    ) -> Option<ColliderHandle> {
        None
    }
}

pub struct Physics {
    pub rbd_set: RigidBodySet,
    pub col_set: ColliderSet,
    pub query_pipeline: QueryPipeline,

    pub collision_recv: Receiver<CollisionEvent>,
    // pub contact_force_recv: Receiver<ContactForceEvent>,
}

impl Physics {
    pub fn new(gravity: Vec2) -> Self {
        let (send, recv) = std::sync::mpsc::channel();

        Self {
            rbd_set: RigidBodySet::new(),
            col_set: ColliderSet::new(),
            query_pipeline: QueryPipeline::new(),

            collision_recv: recv,
        }
    }

    pub fn step(&mut self, delta: f32) {}

    pub fn remove_rbd(&mut self, handle: RigidBodyHandle) {
        todo!()
    }

    pub fn spawn_kinematic_ball(
        &mut self,
        world: &World,
        commands: &mut CommandBuffer,
        size: f32,
        position: Vec2,
        velocity: Vec2,
        collision_groups: InteractionGroups,
        components: impl DynamicBundle,
    ) -> Entity {
        let entity = world.reserve_entity();
        let user_data: u128 = entity.to_bits().get().into();

        let rbd = RigidBody {
            position,
            rotation: 0.0,
            scale: Vec2::ONE,
            velocity,
            angular_velocity: 0.0,
            colilders: vec![],
            user_data,
            body_type: RigidBodyType::KinematicVelocityBased,
        };

        let rbd_handle = self.rbd_set.insert(rbd);

        let collider = Collider {
            position,
            rotation: 0.0,
            scale: Vec2::ONE,
            user_data,
            parent: Some(ColliderParent {
                handle: rbd_handle,
                pos_wrt_parent: Vec2::ZERO,
            }),
            flags: ColliderFlags::default(),
        };

        // let collider = ColliderBuilder::ball(size)
        //     .user_data(user_data)
        //     .active_events(ActiveEvents::COLLISION_EVENTS)
        //     .active_collision_types(
        //         ActiveCollisionTypes::default()
        //             | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
        //     )
        //     .collision_groups(collision_groups);

        self.col_set.insert_with_parent(
            collider,
            rbd_handle,
            &mut self.rbd_set,
        );

        commands.insert(entity, (RbdHandleComponent(rbd_handle),));
        commands.insert(entity, components);

        entity
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct InteractionGroups {
    /// Groups memberships.
    pub memberships: Group,
    /// Groups filter.
    pub filter: Group,
}

impl InteractionGroups {
    /// Initializes with the given interaction groups and interaction mask.
    pub const fn new(memberships: Group, filter: Group) -> Self {
        Self {
            memberships,
            filter,
        }
    }
}

pub struct RigidBodySet {
    arena: Arena<RigidBody>,
}

impl RigidBodySet {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
        }
    }

    pub fn get(&self, handle: RigidBodyHandle) -> Option<&RigidBody> {
        self.arena.get(handle.0)
    }

    pub fn get_mut(
        &mut self,
        handle: RigidBodyHandle,
    ) -> Option<&mut RigidBody> {
        self.arena.get_mut(handle.0)
    }

    pub fn remove_rbd(&mut self, handle: RigidBodyHandle) {
        if self.arena.remove(handle.0).is_none() {
            eprintln!(
                "Trying to remove a rbd that doesn't exit anymore, id: {:?}",
                handle.0
            );
        }
    }

    pub fn len(&self) -> usize {
        self.arena.len()
    }

    fn insert(&mut self, body: RigidBody) -> RigidBodyHandle {
        RigidBodyHandle(self.arena.insert(body))
    }
}

pub struct ColliderSet {
    arena: Arena<Collider>,
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

    pub fn len(&self) -> usize {
        self.arena.len()
    }

    fn insert_with_parent(
        &mut self,
        collider: Collider,
        rbd_handle: RigidBodyHandle,
        rbd_set: &mut RigidBodySet,
    ) {
        self.arena.insert(collider);
        // TODO: insert into rbd
    }
}

pub struct ColliderBuilder {}

pub struct RigidBodyBuilder {}

use bitflags::bitflags;

bitflags! {
    /// A bit mask identifying groups for interaction.
    #[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
    pub struct Group: u32 {
        /// The group n°1.
        const GROUP_1 = 1 << 0;
        /// The group n°2.
        const GROUP_2 = 1 << 1;
        /// The group n°3.
        const GROUP_3 = 1 << 2;
        /// The group n°4.
        const GROUP_4 = 1 << 3;
        /// The group n°5.
        const GROUP_5 = 1 << 4;
        /// The group n°6.
        const GROUP_6 = 1 << 5;
        /// The group n°7.
        const GROUP_7 = 1 << 6;
        /// The group n°8.
        const GROUP_8 = 1 << 7;
        /// The group n°9.
        const GROUP_9 = 1 << 8;
        /// The group n°10.
        const GROUP_10 = 1 << 9;
        /// The group n°11.
        const GROUP_11 = 1 << 10;
        /// The group n°12.
        const GROUP_12 = 1 << 11;
        /// The group n°13.
        const GROUP_13 = 1 << 12;
        /// The group n°14.
        const GROUP_14 = 1 << 13;
        /// The group n°15.
        const GROUP_15 = 1 << 14;
        /// The group n°16.
        const GROUP_16 = 1 << 15;
        /// The group n°17.
        const GROUP_17 = 1 << 16;
        /// The group n°18.
        const GROUP_18 = 1 << 17;
        /// The group n°19.
        const GROUP_19 = 1 << 18;
        /// The group n°20.
        const GROUP_20 = 1 << 19;
        /// The group n°21.
        const GROUP_21 = 1 << 20;
        /// The group n°22.
        const GROUP_22 = 1 << 21;
        /// The group n°23.
        const GROUP_23 = 1 << 22;
        /// The group n°24.
        const GROUP_24 = 1 << 23;
        /// The group n°25.
        const GROUP_25 = 1 << 24;
        /// The group n°26.
        const GROUP_26 = 1 << 25;
        /// The group n°27.
        const GROUP_27 = 1 << 26;
        /// The group n°28.
        const GROUP_28 = 1 << 27;
        /// The group n°29.
        const GROUP_29 = 1 << 28;
        /// The group n°30.
        const GROUP_30 = 1 << 29;
        /// The group n°31.
        const GROUP_31 = 1 << 30;
        /// The group n°32.
        const GROUP_32 = 1 << 31;

        /// All of the groups.
        const ALL = u32::MAX;
        /// None of the groups.
        const NONE = 0;
    }
}

impl From<u32> for Group {
    #[inline]
    fn from(val: u32) -> Self {
        unsafe { Self::from_bits_unchecked(val) }
    }
}

impl From<Group> for u32 {
    #[inline]
    fn from(val: Group) -> Self {
        val.bits()
    }
}
