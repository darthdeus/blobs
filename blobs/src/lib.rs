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
}

pub struct BlobPhysics {}
