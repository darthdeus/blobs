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
pub use std::rc::Rc;

use glam::*;
pub use hecs::*;

use itertools::Itertools;
use thunderdome::{Arena, Index};

mod collider;
mod debug;
mod events;
mod groups;
mod joints;
mod physics;
mod query_filter;
mod rigid_body;
mod spatial;
mod springs;
mod tests;

pub use crate::collider::*;
pub use crate::debug::*;
pub use crate::events::*;
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

    fn calculate_aabb(&self, transform: Affine2) -> AABB {
        let min = transform.translation - vec2(self.radius, self.radius);
        let max = transform.translation + vec2(self.radius, self.radius);

        AABB::new(min, max)
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
    fn calculate_aabb(&self, transform: Affine2) -> AABB;
}

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn from_two_points(a: Vec2, b: Vec2) -> Self {
        Self {
            min: a.min(b),
            max: a.max(b),
        }
    }

    pub fn from_top_left(top_left: Vec2, size: Vec2) -> Self {
        Self::from_center_size(
            vec2(top_left.x + size.x / 2.0, top_left.y - size.y / 2.0),
            size,
        )
    }

    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        let half_size = size * 0.5;
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.min.x <= point.x
            && self.min.y <= point.y
            && self.max.x >= point.x
            && self.max.y >= point.y
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    pub fn expand_to_include_point(&mut self, point: Vec2) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }

    pub fn expand_to_include_aabb(&mut self, other: &AABB) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CollisionEvent {
    pub col_handle_a: ColliderHandle,
    pub col_handle_b: ColliderHandle,

    pub impact_vel_a: Vec2,
    pub impact_vel_b: Vec2,
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

pub struct QueryPipeline {
    #[allow(dead_code)]
    time_data: Rc<TimeData>,
}

impl QueryPipeline {
    pub fn new(time_data: Rc<TimeData>) -> Self {
        Self { time_data }
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

pub trait AffineExtensions {
    fn angle(&self) -> f32;
    fn angle_dir(&self) -> Vec2;
}

impl AffineExtensions for Affine2 {
    fn angle(&self) -> f32 {
        // NOTE: This breaks if the transform has a scale or shear component
        let up = vec2(0.0, 1.0);
        // up.angle_between(self.transform_vector2(up))
        self.transform_vector2(up).angle_between(up)
    }

    fn angle_dir(&self) -> Vec2 {
        let angle = self.angle();
        Vec2::from_angle(angle)
    }
}
