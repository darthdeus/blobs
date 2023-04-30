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

fn spawn_rbd_entity(physics: &mut blobs::Physics, id: Index, desc: RigidBodyDesc) {
    // let entity = world.reserve_entity();
    // let user_data: u128 = entity.to_bits().get().into();
    use blobs::*;

    let rbd = RigidBody {
        position: desc.position,
        position_old: desc.position,
        mass: desc.mass,
        velocity_request: desc.initial_velocity,
        calculated_velocity: Vec2::ZERO,
        acceleration: Vec2::ZERO,
        rotation: 0.0,
        scale: Vec2::ONE,
        radius: desc.radius,
        // angular_velocity: 0.0,
        colliders: vec![],
        user_data: 0,
        // user_data,
        body_type: RigidBodyType::KinematicVelocityBased,
        collision_groups: desc.collision_groups,
    };

    let rbd_handle = physics.insert_rbd(rbd);

    let collider = Collider {
        offset: Vec2::ZERO,
        absolute_position: desc.position,
        rotation: 0.0,
        scale: Vec2::ONE,
        // user_data,
        user_data: 0,
        parent: Some(ColliderParent {
            handle: rbd_handle,
            pos_wrt_parent: Vec2::ZERO,
        }),
        radius: desc.radius,
        flags: ColliderFlags {
            is_sensor: desc.is_sensor,
        },
        collision_groups: desc.collision_groups,
        shape: Box::new(Ball {
            radius: desc.radius,
        }),
    };

    // let collider = ColliderBuilder::ball(size)
    //     .user_data(user_data)
    //     .active_events(ActiveEvents::COLLISION_EVENTS)
    //     .active_collision_types(
    //         ActiveCollisionTypes::default()
    //             | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
    //     )
    //     .collision_groups(collision_groups);

    physics.insert_collider_with_parent(collider, rbd_handle);

    // commands.insert(
    //     entity,
    //     (
    //         RbdHandleComponent(rbd_handle),
    //         Transform::position(desc.position),
    //         Velocity(desc.initial_velocity.unwrap_or(Vec2::ZERO)),
    //     ),
    // );
    // commands.insert(entity, components);
    //
    // entity
}
