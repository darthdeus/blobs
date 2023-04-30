use std::{
    fmt::Debug,
    sync::mpsc::{Receiver, Sender},
};

#[derive(Copy, Clone, Debug)]
pub struct Velocity(pub Vec2);

pub mod perf_counters;
use crate::perf_counters::*;

pub use atomic_refcell::{AtomicRef, AtomicRefCell};
pub use once_cell::sync::Lazy;
pub use std::borrow::Cow;
pub use std::collections::HashMap;

use glam::*;
pub use hecs::*;

use itertools::Itertools;
use thunderdome::{Arena, Index};

mod collider;
mod physics;
mod query_filter;
mod rigid_body;
mod spatial;
mod tests;

pub use collider::*;
pub use physics::*;
pub use query_filter::*;
pub use rigid_body::*;
pub use spatial::*;

pub fn groups(memberships: impl Into<Group>, filter: impl Into<Group>) -> InteractionGroups {
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

// Circle constraint
pub struct Constraint {
    pub position: Vec2,
    pub radius: f32,
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
        Self {
            memberships,
            filter,
        }
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
        (self.memberships.bits() & rhs.filter.bits()) != 0
            && (rhs.memberships.bits() & self.filter.bits()) != 0
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
        Self {
            arena: Arena::new(),
        }
    }

    pub fn get(&self, handle: RigidBodyHandle) -> Option<&RigidBody> {
        self.arena.get(handle.0)
    }

    pub fn get_mut(&mut self, handle: RigidBodyHandle) -> Option<&mut RigidBody> {
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

pub fn get_collider_parent(
    physics: &Physics,
    col_handle: ColliderHandle,
) -> Option<(&Collider, RigidBodyHandle, &RigidBody, Entity)> {
    let col = physics.col_set.get(col_handle)?;
    let rbd_handle = col.parent()?;
    let rbd = physics.get_rbd(rbd_handle)?;
    let entity = Entity::from_bits(rbd.user_data as u64)?;

    Some((col, rbd_handle, rbd, entity))
}

#[cfg(feature = "tracy")]
#[macro_export]
macro_rules! span {
    ($name: expr) => {
        Some(tracy_client::span!($name, 0))
    };
}

#[cfg(not(feature = "tracy"))]
#[macro_export]
macro_rules! span {
    ($name: expr) => {
        None::<()>
    };
}
