use std::{
    fmt::Debug,
    sync::mpsc::{Receiver, Sender},
};

#[derive(Copy, Clone, Debug)]
pub struct Velocity(pub Vec2);

mod perf_counters;
pub use crate::perf_counters::*;

pub use atomic_refcell::{AtomicRef, AtomicRefCell};
pub use once_cell::sync::Lazy;
pub use std::borrow::Cow;
pub use std::collections::HashMap;

use glam::*;
pub use hecs::*;

use itertools::Itertools;
use thunderdome::{Arena, Index};

mod collider;
mod query_filter;
mod rigid_body;
mod spatial;
mod tests;

pub use collider::*;
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

pub struct Physics {
    pub gravity: Vec2,

    pub rbd_set: RigidBodySet,
    pub col_set: ColliderSet,
    pub query_pipeline: QueryPipeline,

    pub spatial_hash: SpatialHash,
    pub constraints: Vec<Constraint>,

    pub use_spatial_hash: bool,

    pub collision_send: Sender<CollisionEvent>,
    pub collision_recv: Receiver<CollisionEvent>,
    // pub contact_force_recv: Receiver<ContactForceEvent>,
    pub accumulator: f64,
    pub time: f64,
}

impl Physics {
    pub fn new(gravity: Vec2, use_spatial_hash: bool) -> Self {
        let (send, recv) = std::sync::mpsc::channel();

        Self {
            gravity,

            rbd_set: RigidBodySet::new(),
            col_set: ColliderSet::new(),
            query_pipeline: QueryPipeline::new(),

            use_spatial_hash,
            constraints: vec![],

            collision_send: send,
            collision_recv: recv,

            accumulator: 0.0,
            time: 0.0,
            spatial_hash: SpatialHash::new(2.0),
        }
    }

    pub fn step(&mut self, substeps: i32, delta: f64) {
        let _span = span!("step");
        let _span = span!("integrate");
        self.integrate(substeps, delta as f32);
        self.time += delta;
    }

    pub fn fixed_step(&mut self, substeps: i32, frame_time: f64) {
        let _span = span!("step");
        self.accumulator += frame_time;

        let delta = 1.0 / 60.0;
        let mut max_steps = 3;

        while self.accumulator >= delta && max_steps > 0 {
            let _span = span!("integrate");
            self.integrate(substeps, delta as f32);

            self.accumulator -= delta;
            self.time += delta;
            max_steps -= 1;
        }
    }

    pub fn get_rbd(&self, handle: RigidBodyHandle) -> Option<&RigidBody> {
        self.rbd_set.get(handle)
    }

    pub fn get_mut_rbd(&mut self, handle: RigidBodyHandle) -> Option<&mut RigidBody> {
        self.rbd_set.get_mut(handle)
    }

    pub fn rbd_count(&self) -> usize {
        self.rbd_set.len()
    }

    pub fn insert_rbd(&mut self, rbd: RigidBody) -> RigidBodyHandle {
        let position = rbd.position;
        let radius = rbd.radius;

        let handle = self.rbd_set.insert(rbd);
        self.spatial_hash
            .insert_with_id(handle.0.to_bits(), position, radius);
        handle
    }

    pub fn insert_collider_with_parent(
        &mut self,
        collider: Collider,
        rbd_handle: RigidBodyHandle,
    ) -> ColliderHandle {
        self.col_set
            .insert_with_parent(collider, rbd_handle, &mut self.rbd_set)
    }

    pub fn remove_rbd(&mut self, handle: RigidBodyHandle) {
        if let Some(rbd) = self.rbd_set.get(handle) {
            for col_handle in rbd.colliders() {
                self.col_set.remove(*col_handle);
            }
        }

        self.rbd_set.remove_rbd(handle);
        self.spatial_hash.remove(handle.0.to_bits());
    }

    pub fn update_rigid_body_position(&mut self, id: u64, offset: Vec2) {
        if let Some(rigid_body) = self
            .rbd_set
            .get_mut(RigidBodyHandle(Index::from_bits(id).unwrap()))
        {
            self.spatial_hash.move_point(id, offset);
            rigid_body.position += offset;
        }
    }

    pub fn brute_force_collisions(&mut self) {
        let _span = span!("brute_force_collisions");

        let keys = self.col_set.arena.iter().map(|(idx, _)| idx).collect_vec();

        let mut count = 0;

        for (i, idx_a) in keys.iter().enumerate() {
            for idx_b in keys.iter().take(i) {
                // for idx_b in keys.iter() {
                //     if idx_a >= idx_b {
                //         continue;
                //     }

                let (Some(col_a), Some(col_b)) = self.col_set.arena.get2_mut(*idx_a, *idx_b) else { continue; };

                let Some(parent_a) = col_a.parent else { continue; };
                let Some(parent_b) = col_b.parent else { continue; };

                if !col_a.collision_groups.test(col_b.collision_groups) {
                    continue;
                }

                let axis = col_a.absolute_position - col_b.absolute_position;
                let distance = axis.length();
                let min_dist = col_a.radius + col_b.radius;

                if distance < min_dist {
                    let (Some(rbd_a), Some(rbd_b)) = self.rbd_set.arena.get2_mut(parent_a.handle.0, parent_b.handle.0) else { continue; };

                    if !col_a.flags.is_sensor && !col_b.flags.is_sensor {
                        let n = axis / distance;
                        let delta = min_dist - distance;

                        let ratio = Self::mass_ratio(rbd_a, rbd_b);

                        rbd_a.position += ratio * delta * n;
                        rbd_b.position -= (1.0 - ratio) * delta * n;
                    }

                    count += 1;

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

        perf_counter_inc("collisions", count);
    }

    pub fn spatial_collisions(&mut self) {
        let _span = span!("spatial_collisions");

        let keys = self.col_set.arena.iter().map(|(idx, _)| idx).collect_vec();
        let mut count = 0;

        for (_i, idx_a) in keys.iter().enumerate() {
            let col_a = self.col_set.arena.get(*idx_a).unwrap();
            let parent_a = col_a.parent.unwrap();
            let rbd_a = self.rbd_set.arena.get(parent_a.handle.0).unwrap();

            const MAX_COLLIDER_RADIUS: f32 = 1.0;

            let relevant_rigid_bodies = self
                .spatial_hash
                .query(rbd_a.position, col_a.radius + MAX_COLLIDER_RADIUS);

            // for idx_b in keys.iter() {
            //     let idx_b = *idx_b;
            for cell_point in relevant_rigid_bodies {
                let idx_b = Index::from_bits(cell_point.id).unwrap();

                if let Some(col_b) = self.col_set.arena.get(idx_b) {
                    if idx_a >= &idx_b {
                        continue;
                    }

                    let parent_b = col_b.parent.unwrap();
                    // let rbd_b =
                    //     self.rbd_set.arena.get(parent_b.handle.0).unwrap();

                    if !col_a.collision_groups.test(col_b.collision_groups) {
                        continue;
                    }

                    let axis = col_a.absolute_position - col_b.absolute_position;
                    let distance = axis.length();
                    let min_dist = col_a.radius + col_b.radius;

                    if distance < min_dist {
                        let parent_a_handle = parent_a.handle.0;
                        let parent_b_handle = parent_b.handle.0;

                        let (Some(rbd_a), Some(rbd_b)) = self
                            .rbd_set
                            .arena
                            .get2_mut(parent_a_handle, parent_b_handle)
                             else { continue; };

                        if !col_a.flags.is_sensor && !col_b.flags.is_sensor {
                            let n = axis / distance;
                            let delta = min_dist - distance;

                            let ratio = Self::mass_ratio(rbd_a, rbd_b);

                            rbd_a.position += ratio * delta * n;
                            rbd_b.position -= (1.0 - ratio) * delta * n;
                        }

                        count += 1;

                        self.collision_send
                            .send(CollisionEvent::Started(
                                ColliderHandle(*idx_a),
                                ColliderHandle(idx_b),
                                CollisionEventFlags::empty(),
                            ))
                            .unwrap();
                    }
                }
            }
        }

        perf_counter_inc("collisions", count);
    }

    fn mass_ratio(a: &RigidBody, b: &RigidBody) -> f32 {
        1.0 - a.mass / (a.mass + b.mass)
    }

    fn update_objects(&mut self, delta: f32) {
        let _span = span!("update positions");

        for (idx, body) in self.rbd_set.arena.iter_mut() {
            if let Some(req_velocity) = body.velocity_request.take() {
                body.position_old = body.position - req_velocity * delta;
            }

            let displacement = body.position - body.position_old;

            self.spatial_hash
                .move_point(idx.to_bits(), body.position - body.position_old);

            body.position_old = body.position;
            body.position += displacement + body.acceleration * delta * delta;

            body.acceleration = Vec2::ZERO;

            body.calculated_velocity = displacement / delta;
        }

        for (_, body) in self.rbd_set.arena.iter_mut() {
            for col_handle in body.colliders() {
                if let Some(collider) = self.col_set.get_mut(*col_handle) {
                    collider.absolute_position = body.position + collider.offset;
                }
            }
        }
    }

    fn apply_constraints(&mut self) {
        for constraint in self.constraints.iter() {
            for (_, body) in self.rbd_set.arena.iter_mut() {
                let obj = constraint.position;
                let radius = constraint.radius;

                let to_obj = body.position - obj;
                let dist = to_obj.length();

                if dist > (radius - body.radius) {
                    let n = to_obj / dist;
                    body.position = obj + n * (radius - body.radius);
                }
            }
        }
    }

    fn integrate(&mut self, substeps: i32, delta: f32) {
        let step_delta = delta / substeps as f32;

        for _ in 0..substeps {
            let _span = span!("substep");

            self.apply_constraints();

            if self.use_spatial_hash {
                self.spatial_collisions();
            } else {
                self.brute_force_collisions();
            }

            self.update_objects(step_delta);
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
