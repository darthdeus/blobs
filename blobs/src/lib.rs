use std::{
    fmt::Debug,
    sync::mpsc::{Receiver, Sender},
};

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
mod groups;
mod joints;
mod physics;
mod query_filter;
mod rigid_body;
mod spatial;
mod springs;
mod tests;

pub use crate::collider::*;
pub use crate::groups::*;
pub use crate::joints::*;
pub use crate::physics::*;
pub use crate::query_filter::*;
pub use crate::rigid_body::*;
pub use crate::spatial::*;
pub use crate::springs::*;

#[derive(Copy, Clone, Debug)]
pub struct Velocity(pub Vec2);

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

pub struct Cuboid {
    pub half_extents: Vec2,
}

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

pub fn get_collider_parent(
    physics: &Physics,
    col_handle: ColliderHandle,
) -> Option<(&Collider, RigidBodyHandle, &RigidBody, Entity)> {
    let col = physics.col_set.get(col_handle)?;
    let rbd_handle = col.parent?;
    let rbd = physics.get_rbd(rbd_handle)?;
    let entity = Entity::from_bits(rbd.user_data as u64)?;

    Some((col, rbd_handle, rbd, entity))
}

#[cfg(feature = "tracy")]
#[macro_export]
macro_rules! tracy_span {
    ($name: expr) => {
        Some(tracy_client::span!($name, 0))
    };
}

#[cfg(not(feature = "tracy"))]
#[macro_export]
macro_rules! tracy_span {
    ($name: expr) => {
        None::<()>
    };
}

pub trait ZipTuple<A, B> {
    fn zip(self) -> Option<(A, B)>;
    fn zip_unwrap(self) -> (A, B);
}

impl<A, B> ZipTuple<A, B> for (Option<A>, Option<B>) {
    fn zip(self) -> Option<(A, B)> {
        match self {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }

    fn zip_unwrap(self) -> (A, B) {
        self.zip().unwrap()
    }
}
