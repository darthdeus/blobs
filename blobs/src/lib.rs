use glam::*;

pub struct Rotation {}

impl Rotation {
    pub fn angle(&self) -> f32 {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct Collider {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,

    pub user_data: u128,
}

impl Collider {
    pub fn parent(&self) -> Option<RigidBodyHandle> {
        todo!();
    }

    pub fn translation(&self) -> Vec2 {
        self.position
    }

    pub fn rotation(&self) -> Rotation {
        todo!()
    }

    pub fn compute_aabb(&self) -> Aabb {
        todo!()
    }

    pub fn collider(&self) -> ! {
        todo!()
    }

    pub fn shape(&self) -> &dyn std::any::Any {
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

type Id = u64;

#[derive(Copy, Clone, Debug)]
pub struct ColliderHandle(Id);

#[derive(Copy, Clone, Debug)]
pub struct RigidBodyHandle(Id);

#[derive(Clone, Debug)]
pub struct RigidBody {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,

    pub velocity: Vec2,
    pub angular_velocity: f32,

    pub colilders: Vec<Collider>,

    pub user_data: u128,
}

impl RigidBody {
    pub fn translation(&self) -> Vec2 {
        self.position
    }

    pub fn linvel(&self) -> &Vec2 {
        &self.velocity
    }
}

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

pub struct Physics {
    pub rbd_set: RigidBodySet,
    pub col_set: ColliderSet,
}

impl Physics {
    pub fn new(gravity: Vec2) -> Self {
        todo!();
    }

    pub fn step(&mut self, delta: f32) {}

    pub fn remove_rbd(&mut self, handle: RigidBodyHandle) {
        todo!()
    }
}

pub struct RigidBodySet {}

impl RigidBodySet {
    pub fn get(&self, handle: RigidBodyHandle) -> Option<&RigidBody> {
        todo!();
    }

    pub fn remove_rbd(&mut self, handle: RigidBodyHandle) {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!();
    }
}

pub struct ColliderSet {}

impl ColliderSet {
    pub fn get(&self, handle: ColliderHandle) -> Option<&Collider> {
        todo!();
    }

    pub fn len(&self) -> usize {
        todo!();
    }
}

pub struct ColliderBuilder {}

pub struct RigidBodyBuilder {}
