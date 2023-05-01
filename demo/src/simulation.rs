use crate::*;

pub struct TestObject {
    pub position: Vec2,
    pub color: Color,
}

pub struct Simulation {
    pub balls: Arena<TestObject>,
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

    pub fn spawn_ball(&mut self, desc: RigidBodyDesc, color: Color) {
        let id = self.balls.insert(TestObject {
            position: Vec2::ZERO,
            color,
        });
        self.physics.spawn_ball(id, desc);
    }

    pub fn cast_physics<T: 'static>(&mut self) -> &mut T {
        self.physics.as_any().downcast_mut().unwrap()
    }
}

pub trait PhysicsEngine {
    fn as_any(&mut self) -> &mut dyn Any;
    fn step(&mut self, delta: f64);

    fn spawn_ball(&mut self, id: Index, desc: RigidBodyDesc);
    fn collider_count(&self) -> usize;

    // fn colliders(&self) -> impl Iterator<Item = (Vec2, f32)>;
    fn colliders(&self) -> Vec<(Vec2, f32)>;
    fn collider(&self, index: Index) -> SimpleCollider;
}

pub struct SimpleCollider {
    pub position: Vec2,
    pub radius: f32,
}

impl PhysicsEngine for blobs::Physics {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn step(&mut self, delta: f64) {
        self.fixed_step(delta);
    }

    fn spawn_ball(&mut self, id: Index, desc: RigidBodyDesc) {
        spawn_rbd_entity(self, id, desc);
    }

    fn collider_count(&self) -> usize {
        self.col_set.arena.len()
    }

    fn colliders(&self) -> Vec<(Vec2, f32)> {
        self.rbd_set
            .arena
            .iter()
            .map(|(_, x)| (x.position, x.radius))
            .collect()
    }

    fn collider(&self, index: Index) -> SimpleCollider {
        let collider = self.col_set.arena.get(index).unwrap();

        SimpleCollider {
            position: collider.absolute_position,
            radius: collider.radius,
        }
    }
}

pub fn spawn_rbd_entity(
    physics: &mut blobs::Physics,
    id: Index,
    desc: RigidBodyDesc,
) -> blobs::RigidBodyHandle {
    // let entity = world.reserve_entity();
    // let user_data: u128 = entity.to_bits().get().into();
    use blobs::*;
    let user_data: u128 = id.to_bits() as u128;

    let rbd = RigidBody {
        position: desc.position,
        position_old: desc.position,
        mass: desc.mass,
        gravity_mod: desc.gravity_mod,
        velocity_request: desc.initial_velocity,
        calculated_velocity: Vec2::ZERO,
        acceleration: Vec2::ZERO,
        rotation: 0.0,
        scale: Vec2::ONE,
        radius: desc.radius,
        // angular_velocity: 0.0,
        colliders: vec![],
        connected_joints: vec![],
        user_data,
        body_type: RigidBodyType::KinematicVelocityBased,
        collision_groups: desc.collision_groups,
    };

    let rbd_handle = physics.insert_rbd(rbd);

    let collider = Collider {
        offset: Vec2::ZERO,
        absolute_position: desc.position,
        rotation: 0.0,
        scale: Vec2::ONE,
        user_data,
        parent: Some(rbd_handle),
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

    rbd_handle
}
