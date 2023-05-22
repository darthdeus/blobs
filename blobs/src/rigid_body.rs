use std::ops::Deref;

use thunderdome::Index;

use crate::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct RigidBodyHandle(pub Index);

impl Deref for RigidBodyHandle {
    type Target = Index;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct RigidBody {
    pub position: Vec2,
    pub position_old: Vec2,

    pub calculated_mass: f32,
    pub gravity_mod: f32,

    // in radians
    pub rotation: f32,
    // in radians per second
    pub angular_velocity: f32,
    // in Newton-meters
    pub torque: f32,
    // moment of inertia in kg*m^2
    pub inertia: f32,

    pub scale: Vec2,

    pub acceleration: Vec2,

    pub velocity_request: Option<Vec2>,
    pub calculated_velocity: Vec2,

    pub colliders: Vec<ColliderHandle>,
    pub connected_joints: Vec<JointHandle>,

    pub user_data: u128,

    pub body_type: RigidBodyType,
}

impl RigidBody {
    pub fn transform(&self) -> Affine2 {
        Affine2::from_angle_translation(self.rotation, self.position)
    }

    pub fn translation(&self) -> Vec2 {
        self.position
    }

    pub fn update_mass_and_inertia(&mut self, col_set: &ColliderSet) {
        self.calculated_mass = 0.0;
        self.inertia = 0.0;

        for col_handle in self.colliders.iter() {
            if let Some(collider) = &col_set.get(*col_handle) {
                self.calculated_mass += collider.mass();
                self.inertia += collider.inertia();
            } else {
                eprintln!("Collider {:?} not found in collider set", col_handle);
            }
        }

        // println!("Mass: {}", self.calculated_mass);
        // println!("Inertia: {}", self.inertia);
    }

    pub fn apply_impulse(&mut self, impulse: Vec2) {
        if !self.is_static() {
            // Convert impulse to velocity change (J = mv, so Δv = J/m)
            self.add_velocity(impulse / self.calculated_mass);
        }
    }

    pub fn apply_impulse_at_point(&mut self, impulse: Vec2, world_point: Vec2) {
        if !self.is_static() {
            // Apply linear impulse
            self.apply_impulse(impulse);

            // Apply rotational impulse (angular impulse)
            let lever_arm = world_point - self.position;
            // 2d cross product?
            let angular_impulse = lever_arm.perp_dot(impulse);
            // Convert angular impulse to angular velocity change (J = Iω, so Δω = J/I)
            self.angular_velocity += angular_impulse / self.inertia;
        }
    }

    pub fn add_velocity(&mut self, velocity: Vec2) {
        self.set_velocity(self.get_velocity() + velocity);
    }

    pub fn apply_force(&mut self, force: Vec2) {
        if !self.is_static() {
            // Convert force to acceleration (F = ma, so a = F/m)
            self.acceleration += force / self.calculated_mass;
        }
    }

    pub fn apply_force_at_point(&mut self, force: Vec2, world_point: Vec2) {
        if !self.is_static() {
            // Apply linear force
            self.apply_force(force);

            // Apply rotational force (torque)
            let lever_arm = world_point - self.position;
            // 2d cross product?
            self.torque += lever_arm.perp_dot(force);
        }
    }

    /// Unlike apply_force_at_point this only applies torque.
    pub fn apply_torque_at_point(&mut self, force: Vec2, world_point: Vec2) {
        if !self.is_static() {
            let lever_arm = world_point - self.position;
            self.torque += lever_arm.perp_dot(force);
        }
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
        self.body_type == RigidBodyType::KinematicPositionBased
            || self.body_type == RigidBodyType::KinematicVelocityBased
    }

    pub fn accelerate(&mut self, a: Vec2) {
        self.acceleration += a;
    }

    pub fn is_static(&self) -> bool {
        self.body_type == RigidBodyType::Static
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
    Static = 1,
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

pub struct RigidBodyBuilder {
    position: Vec2,
    position_old: Vec2,
    gravity_mod: f32,
    rotation: f32,
    scale: Vec2,
    acceleration: Vec2,
    velocity_request: Option<Vec2>,
    calculated_velocity: Vec2,
    colliders: Vec<ColliderHandle>,
    connected_joints: Vec<JointHandle>,
    user_data: u128,
    body_type: RigidBodyType,
}

impl RigidBodyBuilder {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            position_old: Vec2::new(0.0, 0.0),
            gravity_mod: 1.0,
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
            acceleration: Vec2::new(0.0, 0.0),
            velocity_request: None,
            calculated_velocity: Vec2::new(0.0, 0.0),
            colliders: Vec::new(),
            connected_joints: Vec::new(),
            user_data: 0,
            body_type: RigidBodyType::Dynamic,
        }
    }

    pub fn position(mut self, position: Vec2) -> Self {
        self.position_old = position;
        self.position = position;
        self
    }

    pub fn gravity_mod(mut self, gravity_mod: f32) -> Self {
        self.gravity_mod = gravity_mod;
        self
    }

    pub fn rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn acceleration(mut self, acceleration: Vec2) -> Self {
        self.acceleration = acceleration;
        self
    }

    pub fn velocity_request(mut self, velocity_request: Vec2) -> Self {
        self.velocity_request = Some(velocity_request);
        self
    }

    pub fn calculated_velocity(mut self, calculated_velocity: Vec2) -> Self {
        self.calculated_velocity = calculated_velocity;
        self
    }

    pub fn colliders(mut self, colliders: Vec<ColliderHandle>) -> Self {
        self.colliders = colliders;
        self
    }

    pub fn connected_joints(mut self, connected_joints: Vec<JointHandle>) -> Self {
        self.connected_joints = connected_joints;
        self
    }

    pub fn user_data(mut self, user_data: u128) -> Self {
        self.user_data = user_data;
        self
    }

    pub fn body_type(mut self, body_type: RigidBodyType) -> Self {
        self.body_type = body_type;
        self
    }

    pub fn build(self) -> RigidBody {
        RigidBody {
            position: self.position,
            position_old: self.position_old,
            gravity_mod: self.gravity_mod,
            rotation: self.rotation,

            calculated_mass: 1.0,
            angular_velocity: 0.0,
            torque: 0.0,
            inertia: 1.0,

            scale: self.scale,

            acceleration: self.acceleration,
            velocity_request: self.velocity_request,
            calculated_velocity: self.calculated_velocity,
            colliders: self.colliders,
            connected_joints: self.connected_joints,
            user_data: self.user_data,
            body_type: self.body_type,
        }
    }
}
