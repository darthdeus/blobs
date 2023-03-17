use thunderdome::Index;

use crate::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct RigidBodyHandle(pub Index);

#[derive(Copy, Clone, Debug)]
pub struct RbdHandleComponent(pub RigidBodyHandle);

#[derive(Clone, Debug)]
pub struct RigidBody {
    pub position: Vec2,
    pub position_old: Vec2,

    pub rotation: f32,
    pub scale: Vec2,

    pub velocity: Vec2,
    pub angular_velocity: f32,

    pub colliders: Vec<ColliderHandle>,

    pub user_data: u128,

    pub body_type: RigidBodyType,
    pub collision_groups: InteractionGroups,
}

impl RigidBody {
    pub fn translation(&self) -> Vec2 {
        self.position
    }

    pub fn linvel(&self) -> &Vec2 {
        &self.velocity
    }

    pub fn set_linvel(&mut self, velocity: Vec2, _wakeup: bool) {
        self.velocity = velocity;
    }

    pub fn colliders(&self) -> impl Iterator<Item = &ColliderHandle> {
        self.colliders.iter()
    }

    pub fn body_type(&self) -> RigidBodyType {
        self.body_type
    }

    pub fn is_dynamic(&self) -> bool {
        self.body_type == RigidBodyType::Dynamic
    }

    pub fn is_kinematic(&self) -> bool {
        self.body_type == RigidBodyType::KinematicPositionBased
            || self.body_type == RigidBodyType::KinematicVelocityBased
    }

    pub fn is_fixed(&self) -> bool {
        self.body_type == RigidBodyType::Fixed
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]

/// The status of a body, governing the way it is affected by external forces.
pub enum RigidBodyType {
    /// A `RigidBodyType::Dynamic` body can be affected by all external forces.
    Dynamic = 0,
    /// A `RigidBodyType::Fixed` body cannot be affected by external forces.
    Fixed = 1,
    /// A `RigidBodyType::KinematicPositionBased` body cannot be affected by any external forces but can be controlled
    /// by the user at the position level while keeping realistic one-way interaction with dynamic bodies.
    ///
    /// One-way interaction means that a kinematic body can push a dynamic body, but a kinematic body
    /// cannot be pushed by anything. In other words, the trajectory of a kinematic body can only be
    /// modified by the user and is independent from any contact or joint it is involved in.
    KinematicPositionBased = 2,
    /// A `RigidBodyType::KinematicVelocityBased` body cannot be affected by any external forces but can be controlled
    /// by the user at the velocity level while keeping realistic one-way interaction with dynamic bodies.
    ///
    /// One-way interaction means that a kinematic body can push a dynamic body, but a kinematic body
    /// cannot be pushed by anything. In other words, the trajectory of a kinematic body can only be
    /// modified by the user and is independent from any contact or joint it is involved in.
    KinematicVelocityBased = 3,
    // Semikinematic, // A kinematic that performs automatic CCD with the fixed environment to avoid traversing it?
    // Disabled,
}
