use crate::*;

pub struct TestObject {
    pub position: Vec2,
    pub color: Color,
}

pub struct Simulation {
    pub balls: Arena<TestObject>,
    pub physics: Physics,
}

impl Simulation {
    pub fn new(physics: Physics) -> Self {
        Self {
            balls: Arena::new(),
            physics,
        }
    }

    pub fn body_count(&self) -> usize {
        self.balls.len()
    }

    pub fn collider_count(&self) -> usize {
        self.physics.col_set.len()
    }

    pub fn spawn_ball(&mut self, desc: RigidBodyDesc, color: Color) {
        let id = self.balls.insert(TestObject {
            position: Vec2::ZERO,
            color,
        });
        spawn_rbd_entity(&mut self.physics, id, desc);
    }
}

// pub trait PhysicsEngine {
//     fn as_any(&mut self) -> &mut dyn Any;
//     fn step(&mut self, delta: f64);
//
//     fn spawn_ball(&mut self, id: Index, desc: RigidBodyDesc);
//     fn collider_count(&self) -> usize;
// }

// fn collider(&self, index: Index) -> SimpleCollider;
// impl PhysicsEngine for blobs::Physics {
//     fn as_any(&mut self) -> &mut dyn Any {
//         self
//     }
//
//     fn step(&mut self, delta: f64) {
//         self.fixed_step(delta);
//     }
//
//     fn spawn_ball(&mut self, id: Index, desc: RigidBodyDesc) {
//         spawn_rbd_entity(self, id, desc);
//     }
//
//     fn collider_count(&self) -> usize {
//         self.col_set.arena.len()
//     }
//
//     fn colliders(&self) -> Vec<(Vec2, f32)> {
//         self.rbd_set
//             .arena
//             .iter()
//             .map(|(_, x)| (x.position, x.radius))
//             .collect()
//     }
//
//     fn collider(&self, index: Index) -> SimpleCollider {
//         let collider = self.col_set.arena.get(index).unwrap();
//
//         SimpleCollider {
//             position: collider.absolute_translation(),
//             radius: collider.radius,
//         }
//     }
// }

pub fn rbd_from_desc(id: Index, desc: RigidBodyDesc) -> RigidBody {
    let user_data: u128 = id.to_bits() as u128;

    RigidBody {
        position: desc.position,
        position_old: desc.position,
        mass: desc.mass,
        gravity_mod: desc.gravity_mod,
        velocity_request: desc.initial_velocity,
        calculated_velocity: Vec2::ZERO,
        acceleration: Vec2::ZERO,
        rotation: 0.0,
        scale: Vec2::ONE,
        // angular_velocity: 0.0,
        colliders: vec![],
        connected_joints: vec![],
        user_data,
        body_type: desc.body_type,
        collision_groups: desc.collision_groups,
    }
}

pub fn collider_from_desc(
    id: Index,
    parent: RigidBodyHandle,
    offset: Affine2,
    desc: RigidBodyDesc,
) -> Collider {
    Collider {
        offset,
        absolute_transform: Affine2::from_translation(desc.position),
        user_data: id.to_bits() as u128,
        parent: Some(parent),
        radius: desc.radius,
        flags: ColliderFlags {
            is_sensor: desc.is_sensor,
        },
        collision_groups: desc.collision_groups,
        shape: Box::new(Ball {
            radius: desc.radius,
        }),
    }
}

pub fn spawn_rbd_entity(
    physics: &mut blobs::Physics,
    id: Index,
    desc: RigidBodyDesc,
) -> blobs::RigidBodyHandle {
    // let entity = world.reserve_entity();
    let rbd = rbd_from_desc(id, desc);

    let rbd_handle = physics.insert_rbd(rbd);

    let collider = collider_from_desc(id, rbd_handle, Affine2::IDENTITY, desc);

    physics.insert_collider_with_parent(collider, rbd_handle);

    // let collider = ColliderBuilder::ball(size)
    //     .user_data(user_data)
    //     .active_events(ActiveEvents::COLLISION_EVENTS)
    //     .active_collision_types(
    //         ActiveCollisionTypes::default()
    //             | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
    //     )
    //     .collision_groups(collision_groups);

    rbd_handle
}
