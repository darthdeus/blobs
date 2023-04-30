use crate::*;

pub struct Simulation {
    pub balls: Arena<Vec2>,
    pub physics: Box<dyn PhysicsEngine>,
}

impl Simulation {
    pub fn new(physics: Box<dyn PhysicsEngine>) -> Self {
        Self {
            balls: Arena::new(),
            physics,
        }
    }

    pub fn body_count(&self) -> usize {
        self.balls.len()
    }

    pub fn collider_count(&self) -> usize {
        self.physics.collider_count()
    }

    pub fn spawn_ball(&mut self, desc: RigidBodyDesc) {
        let id = self.balls.insert(Vec2::ZERO);
        self.physics.spawn_ball(id, desc);
    }
}

pub trait PhysicsEngine {
    fn step(&mut self, delta: f64);

    fn spawn_ball(&mut self, id: Index, desc: RigidBodyDesc);
    fn collider_count(&self) -> usize;

    // fn colliders(&self) -> impl Iterator<Item = (Vec2, f32)>;
    fn colliders(&self) -> Vec<(Vec2, f32)>;
}

impl PhysicsEngine for blobs::Physics {
    fn step(&mut self, delta: f64) {
        self.step(8, delta);
    }

    fn spawn_ball(&mut self, id: Index, desc: RigidBodyDesc) {
        spawn_rbd_entity(self, id, desc);
    }

    fn colliders(&self) -> Vec<(Vec2, f32)> {
        self.rbd_set
            .arena
            .iter()
            .map(|(_, x)| (x.position, x.radius))
            .collect()
    }

    fn collider_count(&self) -> usize {
        self.col_set.arena.len()
    }
}
