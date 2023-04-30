use crate::*;
use rapier2d::prelude::*;

pub struct RapierEngine {
    pub rbd_set: RigidBodySet,
    pub col_set: ColliderSet,
    pub integration_params: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
}

impl RapierEngine {
    pub fn new(gravity: Vec2) -> Self {
        let rbd_set = RigidBodySet::new();
        let col_set = ColliderSet::new();

        let integration_params = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();

        Self {
            rbd_set,
            col_set,
            integration_params,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
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
