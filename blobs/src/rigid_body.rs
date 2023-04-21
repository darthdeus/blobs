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

    pub mass: f32,

    pub rotation: f32,
    pub scale: Vec2,

    pub acceleration: Vec2,
    pub radius: f32,

    pub velocity_request: Option<Vec2>,
    pub calculated_velocity: Vec2,

    pub colliders: Vec<ColliderHandle>,

    pub user_data: u128,

    pub body_type: RigidBodyType,
    pub collision_groups: InteractionGroups,
}

impl RigidBody {
    pub fn translation(&self) -> Vec2 {
        self.position
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity_request = Some(velocity);
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.calculated_velocity
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
        self.body_type == RigidBodyType::KinematicPositionBased ||
            self.body_type == RigidBodyType::KinematicVelocityBased
    }

    pub fn is_fixed(&self) -> bool {
        self.body_type == RigidBodyType::Fixed
    }
}

// TODO: Check how kinematic velocity and position based is implemented
// https://github.com/dimforge/bevy_rapier/blob/8dbc80d035b102a208de436a629fdc3b57a5224b/src/dynamics/rigid_body.rs
// TODO: does rapier use verlet?
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
