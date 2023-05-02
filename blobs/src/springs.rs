use std::ops::Deref;

use crate::*;

#[derive(Copy, Clone, Debug)]
pub struct SpringHandle(pub Index);

impl Deref for SpringHandle {
    type Target = Index;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Spring {
    pub rigid_body_a: RigidBodyHandle,
    pub rigid_body_b: RigidBodyHandle,
    pub rest_length: f32,
    pub stiffness: f32,
    pub damping: f32,
}

impl Spring {
    pub fn apply_force(&self, rbd_set: &mut RigidBodySet) {
        let (rbd_a, rbd_b) = rbd_set
            .arena
            .get2_mut(*self.rigid_body_a, *self.rigid_body_b)
            .zip_unwrap();

        let delta_position = rbd_b.position - rbd_a.position;
        let distance = delta_position.length();
        let direction = delta_position / distance;

        // let force_magnitude = self.stiffness * (distance - self.rest_length)
        //     + self.damping * (body_a.get_velocity() - position_b).dot(direction);

        let relative_velocity = rbd_a.get_velocity() - rbd_b.get_velocity();
        let damping_force = self.damping * relative_velocity.dot(direction) * direction;

        let force_magnitude = self.stiffness * (distance - self.rest_length) - damping_force;

        let force = direction * force_magnitude;

        rbd_a.apply_force(force);
        rbd_b.apply_force(-force);
    }
}
