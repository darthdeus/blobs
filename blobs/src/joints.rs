use crate::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct JointHandle(pub Index);

#[derive(Debug)]
pub struct FixedJoint {
    pub rigid_body_a: RigidBodyHandle,
    pub rigid_body_b: RigidBodyHandle,

    // Local anchor point in RigidBody A's coordinate system
    pub anchor_a: Vec2,
    // Local anchor point in RigidBody B's coordinate system
    pub anchor_b: Vec2,
    // Distance between anchor points when the joint was created
    pub distance: f32,

    pub target_angle: f32,
}
