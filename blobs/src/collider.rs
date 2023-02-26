use crate::*;

#[derive(Clone, Debug)]
pub struct Collider {
    pub offset: Vec2,
    pub rotation: f32,
    pub scale: Vec2,

    pub user_data: u128,
    pub parent: Option<ColliderParent>,

    pub size: f32,

    pub flags: ColliderFlags,

    pub(crate) absolute_position: Vec2,
    pub collision_groups: InteractionGroups,
}

impl Collider {
    pub fn parent(&self) -> Option<RigidBodyHandle> {
        self.parent.map(|x| x.handle)
    }

    pub fn is_sensor(&self) -> bool {
        todo!()
    }

    pub fn translation(&self) -> Vec2 {
        self.absolute_position
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

    pub fn shape(&self) -> &dyn Shape {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// Information about the rigid-body this collider is attached to.
pub struct ColliderParent {
    /// Handle of the rigid-body this collider is attached to.
    pub handle: RigidBodyHandle,
    /// Const position of this collider relative to its parent rigid-body.
    pub pos_wrt_parent: Vec2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// A set of flags for controlling collision/intersection filtering, modification, and events.
pub struct ColliderFlags {
    // pub active_collision_types: ActiveCollisionTypes,
    // pub collision_groups: InteractionGroups,
    // pub solver_groups: InteractionGroups,
    // pub active_hooks: ActiveHooks,
    // pub active_events: ActiveEvents,
}

impl Default for ColliderFlags {
    fn default() -> Self {
        Self {
            // active_collision_types: ActiveCollisionTypes::default(),
            // collision_groups: InteractionGroups::all(),
            // solver_groups: InteractionGroups::all(),
            // active_hooks: ActiveHooks::empty(),
            // active_events: ActiveEvents::empty(),
        }
    }
}
