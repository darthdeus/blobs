use rapier2d::prelude::*;
use crate::*;

pub struct RapierEngine {

}

impl RapierEngine {
    pub fn new(gravity: Vec2) -> Self {
        let rbd_set = RigidBodySet::new();

        Self {

        }
    }
}

impl PhysicsEngine for RapierEngine {
    fn step(&mut self, delta: f64) {
        todo!()
    }

    fn spawn_ball(&mut self, id: Index, desc: RigidBodyDesc) {
        todo!()
    }

    fn collider_count(&self) -> usize {
        todo!()
    }

    fn colliders(&self) -> Vec<(Vec2, f32)> {
        todo!()
    }
}
