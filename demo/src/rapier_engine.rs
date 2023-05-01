use crate::*;
use rapier2d::prelude::*;

pub struct RapierEngine {
    pub gravity: Vec2,
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
            gravity,
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
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn step(&mut self, delta: f64) {
        self.integration_params.dt = delta as f32;

        self.physics_pipeline.step(
            &vector![self.gravity.x, self.gravity.y],
            &self.integration_params,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rbd_set,
            &mut self.col_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &(),
            &(),
        );

        // for (handle, rbd) in self.rbd_set.iter_mut() {
        //     if rbd.translation().norm() > 3.5 {
        //         rbd.set_translation(rbd.translation().normalize() * 3.5, true);
        //     }
        // }

        // for (_handle, col) in self.col_set.iter_mut() {
        //     if col.translation().norm() > 3.5 {
        //         col.set_translation(col.translation().normalize() * 3.5);
        //     }
        // }
    }

    fn spawn_ball(&mut self, id: Index, desc: RigidBodyDesc) {
        let user_data: u128 = id.to_bits() as u128;

        let col = rapier2d::prelude::ColliderBuilder::ball(desc.radius)
            .user_data(user_data)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .active_collision_types(
                ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_DYNAMIC,
            )
            .collision_groups(InteractionGroups::new(0b0001.into(), 0b0001.into()));

        let rbd = RigidBodyBuilder::dynamic()
            .translation(vector![desc.position.x, desc.position.y])
            .user_data(user_data)
            .build();

        let rbd_handle = self.rbd_set.insert(rbd);
        let _col_handle = self
            .col_set
            .insert_with_parent(col, rbd_handle, &mut self.rbd_set);
    }

    fn collider_count(&self) -> usize {
        self.col_set.len()
    }

    fn colliders(&self) -> Vec<(Vec2, f32)> {
        self.col_set
            .iter()
            .filter_map(|(_handle, collider)| {
                if let Some(ball) = collider.shape().as_ball() {
                    Some((
                        vec2(collider.translation().x, collider.translation().y),
                        ball.radius,
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    fn collider(&self, index: Index) -> SimpleCollider {
        todo!()
    }
}
