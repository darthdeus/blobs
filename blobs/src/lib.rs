use glam::*;

pub struct Collider {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

type Id = u64;

pub struct ColliderHandle(Id);
pub struct RigidBodyHandle(Id);

pub struct RigidBody {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,

    pub velocity: Vec2,
    pub angular_velocity: f32,

    pub colilders: Vec<Collider>,
}

pub struct BlobPhysics {
    bodies: Vec<RigidBody>,
}

impl BlobPhysics {
    pub fn step(&mut self, delta: f32) {
        for body in self.bodies.iter_mut() {
            body.position += body.velocity * delta;
            body.rotation += body.angular_velocity * delta;
        }
    }
}
