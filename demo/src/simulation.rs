use crate::*;

pub use std::cell::RefCell;
pub use std::ops::{Deref, DerefMut};
pub use std::rc::Rc;
pub use std::{cell::Ref, cell::RefMut};

pub struct OwnedRigidBodyHandle {
    pub handle: RigidBodyHandle,
    pub physics: Rc<RefCell<Physics>>,
}

impl OwnedRigidBodyHandle {
    pub fn deref(&self) -> impl Deref<Target = RigidBody> + '_ {
        Ref::map(self.physics.borrow(), |x| {
            x.rbd_set.get(self.handle).unwrap()
        })
    }

    pub fn deref_mut(&self) -> impl DerefMut<Target = RigidBody> + '_ {
        RefMut::map(self.physics.borrow_mut(), |x| {
            x.rbd_set.get_mut(self.handle).unwrap()
        })
    }
}

impl Drop for OwnedRigidBodyHandle {
    fn drop(&mut self) {
        self.physics.borrow_mut().remove_rbd(self.handle);
    }
}

pub struct OwnedColliderHandle {
    pub handle: ColliderHandle,
    pub physics: Rc<RefCell<Physics>>,
}

impl Drop for OwnedColliderHandle {
    fn drop(&mut self) {
        self.physics.borrow_mut().remove_col(self.handle);
    }
}

pub struct GameObject {
    pub collider: Option<OwnedColliderHandle>,
    pub rbd: Option<OwnedRigidBodyHandle>,
}

pub struct TestObject {
    pub position: Vec2,
    pub color: Color,
}

pub struct Simulation {
    pub balls: Arena<TestObject>,
    pub physics: Rc<RefCell<Physics>>,
}

impl Simulation {
    pub fn new(physics: Physics) -> Self {
        Self {
            balls: Arena::new(),
            physics: Rc::new(RefCell::new(physics)),
        }
    }

    pub fn body_count(&self) -> usize {
        self.balls.len()
    }

    pub fn collider_count(&self) -> usize {
        self.physics.borrow().col_set.len()
    }

    pub fn spawn_ball(&mut self, desc: RigidBodyDesc, color: Color) -> RigidBodyHandle {
        let id = self.balls.insert(TestObject {
            position: Vec2::ZERO,
            color,
        });

        spawn_rbd_entity(&mut self.physics.borrow_mut(), id, desc);

        RigidBodyHandle(id)
    }
}

pub fn rbd_from_desc(id: Index, desc: RigidBodyDesc) -> RigidBody {
    let user_data: u128 = id.to_bits() as u128;

    RigidBodyBuilder::new()
        .position(desc.position)
        .gravity_mod(desc.gravity_mod)
        .velocity_request(desc.initial_velocity.unwrap_or(Vec2::ZERO))
        .body_type(desc.body_type)
        .user_data(user_data)
        .build()
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
        mass_override: None,
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
