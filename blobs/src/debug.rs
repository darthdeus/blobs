use crate::*;

#[derive(Clone, Debug)]
pub struct DebugData {
    pub bodies: Vec<DebugRigidBody>,
    pub joints: Vec<DebugJoint>,
    pub colliders: Vec<DebugCollider>,
    pub springs: Vec<DebugSpring>,
}

#[derive(Copy, Clone, Debug)]
pub struct DebugRigidBody {
    pub transform: Affine2,
}

#[derive(Copy, Clone, Debug)]
pub struct DebugJoint {
    pub body_a: Vec2,
    pub body_b: Vec2,
}

#[derive(Copy, Clone, Debug)]
pub struct DebugCollider {
    pub transform: Affine2,
    pub radius: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct DebugSpring {
    pub body_a: Vec2,
    pub body_b: Vec2,
}

pub(crate) fn make_debug_data(physics: &Physics) -> DebugData {
    let bodies = physics
        .rbd_set
        .arena
        .iter()
        .map(|(_, body)| {
            let transform = Affine2::from_angle_translation(body.rotation, body.position);
            DebugRigidBody { transform }
        })
        .collect();

    let joints = physics
        .joints
        .iter()
        .map(|(_, joint)| {
            let body_a = physics.rbd_set.arena[*joint.rigid_body_a].position;
            let body_b = physics.rbd_set.arena[*joint.rigid_body_b].position;
            DebugJoint { body_a, body_b }
        })
        .collect();

    let colliders = vec![];

    // let colliders = physics
    //     .col_set
    //     .arena
    //     .iter()
    //     .map(|(_, collider)| {
    //         let radius = match collider.shape.as_ball() {
    //             Some(ball) => ball.radius,
    //             None => {
    //                 println!("Invalid shape, expected ball");
    //                 1.0
    //             }
    //         };
    //
    //         DebugCollider {
    //             transform: collider.absolute_transform,
    //             radius,
    //         }
    //     })
    //     .collect();

    let springs = physics
        .springs
        .iter()
        .map(|(_, spring)| {
            let body_a = physics.rbd_set.arena[*spring.rigid_body_a].position;
            let body_b = physics.rbd_set.arena[*spring.rigid_body_b].position;
            DebugSpring { body_a, body_b }
        })
        .collect();

    DebugData {
        bodies,
        joints,
        colliders,
        springs,
    }
}
