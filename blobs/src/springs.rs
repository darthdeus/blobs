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

pub enum SpringConnection {
    RigidBody(RigidBodyHandle),
    Location(Vec2),
}

pub struct Spring {
    pub rigid_body_a: RigidBodyHandle,
    pub connection_b: SpringConnection,
    pub rest_length: f32,
    pub stiffness: f32,
    pub damping: f32,
}

impl Spring {
    pub fn apply_force(&self, rbd_set: &mut RigidBodySet) {
        let body_a = rbd_set.arena.get(self.rigid_body_a.0).unwrap();

        let position_b = match &self.connection_b {
            SpringConnection::RigidBody(handle_b) => {
                let body_b = rbd_set.arena.get(handle_b.0).unwrap();
                body_b.position
            }
            SpringConnection::Location(location) => *location,
        };

        let delta_position = position_b - body_a.position;
        let distance = delta_position.length();
        let direction = delta_position / distance;

        // let force_magnitude = self.stiffness * (distance - self.rest_length)
        //     + self.damping * (body_a.get_velocity() - position_b).dot(direction);

        let relative_velocity = body_a.get_velocity() - position_b;
        let damping_force = self.damping * relative_velocity.dot(direction) * direction;

        let force_magnitude = self.stiffness * (distance - self.rest_length) - damping_force;

        let force = direction * force_magnitude;

        let body_a = rbd_set.arena.get_mut(self.rigid_body_a.0).unwrap();
        body_a.apply_force(force);

        if let SpringConnection::RigidBody(handle_b) = &self.connection_b {
            let body_b = rbd_set.arena.get_mut(handle_b.0).unwrap();
            body_b.apply_force(-force);
        }
    }
}
