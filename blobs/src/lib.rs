use std::{
    fmt::Debug,
    sync::mpsc::{Receiver, Sender},
};

#[derive(Copy, Clone, Debug)]
pub struct Velocity(pub Vec2);

use glam::*;
pub use hecs::*;

use itertools::Itertools;
use thunderdome::{Arena, Index};

mod collider;
mod query_filter;
mod rigid_body;
mod spatial;

pub use collider::*;
pub use query_filter::*;
pub use rigid_body::*;
pub use spatial::*;

pub fn groups(
    memberships: impl Into<Group>,
    filter: impl Into<Group>,
) -> InteractionGroups {
    InteractionGroups::new(memberships.into(), filter.into())
}

#[derive(Clone, Debug)]
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

pub trait Shape: 'static + Debug {
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
        _rbd_set: &RigidBodySet,
        _col_set: &ColliderSet,
        _position: &Vec2,
        _shape: &dyn Shape,
        _filter: QueryFilter,
    ) -> Option<ColliderHandle> {
        None
    }
}

pub struct Physics {
    pub gravity: Vec2,

    pub rbd_set: RigidBodySet,
    pub col_set: ColliderSet,
    pub query_pipeline: QueryPipeline,

    pub collision_send: Sender<CollisionEvent>,
    pub collision_recv: Receiver<CollisionEvent>,
    // pub contact_force_recv: Receiver<ContactForceEvent>,
    pub accumulator: f64,
    pub time: f64,
}

impl Physics {
    pub fn new(gravity: Vec2) -> Self {
        let (send, recv) = std::sync::mpsc::channel();

        Self {
            gravity,

            rbd_set: RigidBodySet::new(),
            col_set: ColliderSet::new(),
            query_pipeline: QueryPipeline::new(),

            collision_send: send,
            collision_recv: recv,

            accumulator: 0.0,
            time: 0.0,
        }
    }

    pub fn step(&mut self, substeps: i32, frame_time: f64) {
        self.accumulator += frame_time;

        let delta = 1.0 / 60.0;
        let mut max_steps = 50;

        while self.accumulator >= delta && max_steps > 0 {
            self.integrate(substeps, delta as f32);

            self.accumulator -= delta;
            self.time += delta;
            max_steps -= 1;
        }
    }

    fn integrate(&mut self, substeps: i32, delta: f32) {
        let delta = delta / substeps as f32;

        for _ in 0..substeps {
            for (_, body) in self.rbd_set.arena.iter_mut() {
                if body.body_type == RigidBodyType::KinematicVelocityBased {
                    let velocity = body.position - body.position_old;

                    body.position_old = body.position;
                    body.position +=
                        velocity * delta + self.gravity * delta * delta;
                }
            }


            for (_, body) in self.rbd_set.arena.iter_mut() {
                for col_handle in body.colliders() {
                    if let Some(collider) = self.col_set.get_mut(*col_handle) {
                        collider.absolute_position =
                            body.position + collider.offset;
                    }
                }
            }

            // for (_, body) in self.rbd_set.arena.iter_mut() {
            //     let obj = Vec2::ZERO;
            //     let to_obj = body.position - obj;
            //     let dist = to_obj.length();
            //     let radius = 4.0;
            //
            //     if dist > (radius - body.radius) {
            //         let n = to_obj / dist;
            //         body.position = obj + n * (radius - body.radius);
            //     }
            // }

            let keys =
                self.col_set.arena.iter().map(|(idx, _)| idx).collect_vec();

            for (i, idx_a) in keys.iter().enumerate() {
                for idx_b in keys.iter().take(i) {
                    let (Some(col_a), Some(col_b)) = self.col_set.arena.get2_mut(*idx_a, *idx_b) else { continue; };

                    let Some(parent_a) = col_a.parent else { continue; };
                    let Some(parent_b) = col_b.parent else { continue; };

                    let (Some(rbd_a), Some(rbd_b)) = self.rbd_set.arena.get2_mut(parent_a.handle.0, parent_b.handle.0) else { continue; };

                    if !col_a.collision_groups.test(col_b.collision_groups) {
                        continue;
                    }

                    let axis =
                        col_a.absolute_position - col_b.absolute_position;
                    let distance = axis.length();
                    let min_dist = col_a.radius + col_b.radius;

                    if distance < min_dist {
                        if !col_a.flags.is_sensor && !col_b.flags.is_sensor {
                            let n = axis / distance;
                            let delta = min_dist - distance;

                            rbd_a.position += 0.5 * delta * n;
                            rbd_b.position -= 0.5 * delta * n;
                        }

                        // col_a.position += 0.5 * delta * n;
                        // col_b.position -= 0.5 * delta * n;

                        self.collision_send
                            .send(CollisionEvent::Started(
                                ColliderHandle(*idx_a),
                                ColliderHandle(*idx_b),
                                CollisionEventFlags::empty(),
                            ))
                            .unwrap();
                    }
                }
            }
        }

        // for (_, obj_a) in self.rbd_set.arena.iter_mut() {
        //     for (_, obj_b) in self.rbd_set.arena.iter_mut() {
        //         // let obj = Vec2::ZERO;
        //         // let to_obj = body.position - obj;
        //         // let dist = to_obj.length();
        //         // let radius = 3.0;
        //         //
        //         // if dist > (radius - 0.5) {
        //         //     let n = to_obj / dist;
        //         //     body.position = obj + n * (dist - 0.5);
        //         // }
        //     }
        // }

        // for (i, (col_a_id, col_a)) in self.col_set.arena.iter().enumerate() {
        //     for (col_b_id, col_b) in self.col_set.arena.iter().take(i) {
        //         if !col_a.collision_groups.test(col_b.collision_groups) {
        //             continue;
        //         }
        //
        //         let distance =
        //             col_a.absolute_position.distance(col_b.absolute_position);
        //
        //         if distance < col_a.size + col_b.size {
        //             self.collision_send
        //                 .send(CollisionEvent::Started(
        //                     ColliderHandle(col_a_id),
        //                     ColliderHandle(col_b_id),
        //                     CollisionEventFlags::empty(),
        //                 ))
        //                 .unwrap();
        //         }
        //     }
        // }
    }


    pub fn remove_rbd(&mut self, handle: RigidBodyHandle) {
        if let Some(rbd) = self.rbd_set.get(handle) {
            for col_handle in rbd.colliders() {
                self.col_set.remove(*col_handle);
            }
        }

        self.rbd_set.remove_rbd(handle);
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct InteractionGroups {
    pub memberships: Group,
    pub filter: Group,
}

impl InteractionGroups {
    /// Initializes with the given interaction groups and interaction mask.
    pub const fn new(memberships: Group, filter: Group) -> Self {
        Self { memberships, filter }
    }

    /// Allow interaction with everything.
    pub const fn all() -> Self {
        Self::new(Group::ALL, Group::ALL)
    }

    /// Prevent all interactions.
    pub const fn none() -> Self {
        Self::new(Group::NONE, Group::NONE)
    }

    /// Sets the group this filter is part of.
    pub const fn with_memberships(mut self, memberships: Group) -> Self {
        self.memberships = memberships;
        self
    }

    /// Sets the interaction mask of this filter.
    pub const fn with_filter(mut self, filter: Group) -> Self {
        self.filter = filter;
        self
    }

    /// Check if interactions should be allowed based on the interaction
    /// memberships and filter.
    ///
    /// An interaction is allowed iff. the memberships of `self` contain at
    /// least one bit set to 1 in common with the filter of `rhs`, and
    /// vice-versa.
    #[inline]
    pub const fn test(self, rhs: Self) -> bool {
        // NOTE: since const ops is not stable, we have to convert `Group` into
        // u32 to use & operator in const context.
        (self.memberships.bits() & rhs.filter.bits()) != 0 &&
            (rhs.memberships.bits() & self.filter.bits()) != 0
    }
}

impl Default for InteractionGroups {
    fn default() -> Self {
        Self::all()
    }
}

pub struct RigidBodySet {
    pub arena: Arena<RigidBody>,
}

impl RigidBodySet {
    pub fn new() -> Self {
        Self { arena: Arena::new() }
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

    pub fn insert(&mut self, body: RigidBody) -> RigidBodyHandle {
        RigidBodyHandle(self.arena.insert(body))
    }
}

pub struct ColliderSet {
    arena: Arena<Collider>,
}

impl ColliderSet {
    pub fn new() -> Self {
        Self { arena: Arena::new() }
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
    ) {
        let col_handle = self.arena.insert(collider);

        if let Some(rbd) = rbd_set.get_mut(rbd_handle) {
            rbd.colliders.push(ColliderHandle(col_handle));
        }
        // TODO: insert into rbd
    }
}

pub struct ColliderBuilder {}

pub struct RigidBodyBuilder {}

use bitflags::bitflags;

bitflags! {
    pub struct Group: u32 {
        const GROUP_1 = 1 << 0;
        const GROUP_2 = 1 << 1;
        const GROUP_3 = 1 << 2;
        const GROUP_4 = 1 << 3;
        const GROUP_5 = 1 << 4;
        const GROUP_6 = 1 << 5;
        const GROUP_7 = 1 << 6;
        const GROUP_8 = 1 << 7;
        const GROUP_9 = 1 << 8;
        const GROUP_10 = 1 << 9;
        const GROUP_11 = 1 << 10;
        const GROUP_12 = 1 << 11;
        const GROUP_13 = 1 << 12;
        const GROUP_14 = 1 << 13;
        const GROUP_15 = 1 << 14;
        const GROUP_16 = 1 << 15;
        const GROUP_17 = 1 << 16;
        const GROUP_18 = 1 << 17;
        const GROUP_19 = 1 << 18;
        const GROUP_20 = 1 << 19;
        const GROUP_21 = 1 << 20;
        const GROUP_22 = 1 << 21;
        const GROUP_23 = 1 << 22;
        const GROUP_24 = 1 << 23;
        const GROUP_25 = 1 << 24;
        const GROUP_26 = 1 << 25;
        const GROUP_27 = 1 << 26;
        const GROUP_28 = 1 << 27;
        const GROUP_29 = 1 << 28;
        const GROUP_30 = 1 << 29;
        const GROUP_31 = 1 << 30;
        const GROUP_32 = 1 << 31;

        const ALL = u32::MAX;
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
