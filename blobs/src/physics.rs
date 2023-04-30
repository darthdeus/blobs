use crate::*;

pub struct Physics {
    pub gravity: Vec2,

    pub rbd_set: RigidBodySet,
    pub col_set: ColliderSet,
    pub query_pipeline: QueryPipeline,

    pub spatial_hash: SpatialHash,
    pub constraints: Vec<Constraint>,

    pub use_spatial_hash: bool,

    pub collision_send: Sender<CollisionEvent>,
    pub collision_recv: Receiver<CollisionEvent>,
    // pub contact_force_recv: Receiver<ContactForceEvent>,
    pub accumulator: f64,
    pub time: f64,
}

impl Physics {
    pub fn new(gravity: Vec2, use_spatial_hash: bool) -> Self {
        let (send, recv) = std::sync::mpsc::channel();

        Self {
            gravity,

            rbd_set: RigidBodySet::new(),
            col_set: ColliderSet::new(),
            query_pipeline: QueryPipeline::new(),

            use_spatial_hash,
            constraints: vec![],

            collision_send: send,
            collision_recv: recv,

            accumulator: 0.0,
            time: 0.0,
            spatial_hash: SpatialHash::new(2.0),
        }
    }

    pub fn step(&mut self, substeps: i32, delta: f64) {
        let _span = tracy_span!("step");
        self.integrate(substeps, delta as f32);
        self.time += delta;
    }

    pub fn fixed_step(&mut self, substeps: i32, frame_time: f64) {
        let _span = tracy_span!("step");
        self.accumulator += frame_time;

        let delta = 1.0 / 60.0;
        let mut max_steps = 3;

        while self.accumulator >= delta && max_steps > 0 {
            let _span = tracy_span!("integrate");
            self.integrate(substeps, delta as f32);

            self.accumulator -= delta;
            self.time += delta;
            max_steps -= 1;
        }
    }

    pub fn get_rbd(&self, handle: RigidBodyHandle) -> Option<&RigidBody> {
        self.rbd_set.get(handle)
    }

    pub fn get_mut_rbd(&mut self, handle: RigidBodyHandle) -> Option<&mut RigidBody> {
        self.rbd_set.get_mut(handle)
    }

    pub fn rbd_count(&self) -> usize {
        self.rbd_set.len()
    }

    pub fn insert_rbd(&mut self, rbd: RigidBody) -> RigidBodyHandle {
        let position = rbd.position;
        let radius = rbd.radius;

        let handle = self.rbd_set.insert(rbd);
        self.spatial_hash
            .insert_with_id(handle.0.to_bits(), position, radius);
        handle
    }

    pub fn insert_collider_with_parent(
        &mut self,
        collider: Collider,
        rbd_handle: RigidBodyHandle,
    ) -> ColliderHandle {
        self.col_set
            .insert_with_parent(collider, rbd_handle, &mut self.rbd_set)
    }

    pub fn remove_rbd(&mut self, handle: RigidBodyHandle) {
        if let Some(rbd) = self.rbd_set.get(handle) {
            for col_handle in rbd.colliders() {
                self.col_set.remove(*col_handle);
            }
        }

        self.rbd_set.remove_rbd(handle);
        self.spatial_hash.remove(handle.0.to_bits());
    }

    pub fn update_rigid_body_position(&mut self, id: u64, offset: Vec2) {
        if let Some(rigid_body) = self
            .rbd_set
            .get_mut(RigidBodyHandle(Index::from_bits(id).unwrap()))
        {
            self.spatial_hash.move_point(id, offset);
            rigid_body.position += offset;
        }
    }

    pub fn brute_force_collisions(&mut self) {
        let _span = tracy_span!("brute_force_collisions");

        let keys = self.col_set.arena.iter().map(|(idx, _)| idx).collect_vec();

        let mut count = 0;

        for (i, idx_a) in keys.iter().enumerate() {
            for idx_b in keys.iter().take(i) {
                // for idx_b in keys.iter() {
                //     if idx_a >= idx_b {
                //         continue;
                //     }

                let (Some(col_a), Some(col_b)) = self.col_set.arena.get2_mut(*idx_a, *idx_b) else { continue; };

                let Some(parent_a) = col_a.parent else { continue; };
                let Some(parent_b) = col_b.parent else { continue; };

                if !col_a.collision_groups.test(col_b.collision_groups) {
                    continue;
                }

                let axis = col_a.absolute_position - col_b.absolute_position;
                let distance = axis.length();
                let min_dist = col_a.radius + col_b.radius;

                if distance < min_dist {
                    let (Some(rbd_a), Some(rbd_b)) = self.rbd_set.arena.get2_mut(parent_a.handle.0, parent_b.handle.0) else { continue; };

                    if !col_a.flags.is_sensor && !col_b.flags.is_sensor {
                        let n = axis / distance;
                        let delta = min_dist - distance;

                        let ratio = Self::mass_ratio(rbd_a, rbd_b);

                        rbd_a.position += ratio * delta * n;
                        rbd_b.position -= (1.0 - ratio) * delta * n;
                    }

                    count += 1;

                    self.collision_send
                        .send(CollisionEvent::Started(
                            ColliderHandle(*idx_a),
                            ColliderHandle(*idx_b),
                            CollisionEventFlags::empty(),
                        ))
                        .unwrap();
                }
            }
        }

        perf_counter_inc("collisions", count);
    }

    pub fn spatial_collisions(&mut self) {
        let _span = tracy_span!("spatial_collisions");

        let keys = self.col_set.arena.iter().map(|(idx, _)| idx).collect_vec();
        let mut count = 0;

        for (_i, idx_a) in keys.iter().enumerate() {
            let col_a = self.col_set.arena.get(*idx_a).unwrap();
            let parent_a = col_a.parent.unwrap();
            let rbd_a = self.rbd_set.arena.get(parent_a.handle.0).unwrap();

            const MAX_COLLIDER_RADIUS: f32 = 1.0;

            let relevant_rigid_bodies = self
                .spatial_hash
                .query(rbd_a.position, col_a.radius + MAX_COLLIDER_RADIUS);

            // for idx_b in keys.iter() {
            //     let idx_b = *idx_b;
            for cell_point in relevant_rigid_bodies {
                let idx_b = Index::from_bits(cell_point.id).unwrap();

                if let Some(col_b) = self.col_set.arena.get(idx_b) {
                    if idx_a >= &idx_b {
                        continue;
                    }

                    let parent_b = col_b.parent.unwrap();
                    // let rbd_b =
                    //     self.rbd_set.arena.get(parent_b.handle.0).unwrap();

                    if !col_a.collision_groups.test(col_b.collision_groups) {
                        continue;
                    }

                    let axis = col_a.absolute_position - col_b.absolute_position;
                    let distance = axis.length();
                    let min_dist = col_a.radius + col_b.radius;

                    if distance < min_dist {
                        let parent_a_handle = parent_a.handle.0;
                        let parent_b_handle = parent_b.handle.0;

                        let (Some(rbd_a), Some(rbd_b)) = self
                            .rbd_set
                            .arena
                            .get2_mut(parent_a_handle, parent_b_handle)
                             else { continue; };

                        if !col_a.flags.is_sensor && !col_b.flags.is_sensor {
                            let n = axis / distance;
                            let delta = min_dist - distance;

                            let ratio = Self::mass_ratio(rbd_a, rbd_b);

                            rbd_a.position += ratio * delta * n;
                            rbd_b.position -= (1.0 - ratio) * delta * n;
                        }

                        count += 1;

                        self.collision_send
                            .send(CollisionEvent::Started(
                                ColliderHandle(*idx_a),
                                ColliderHandle(idx_b),
                                CollisionEventFlags::empty(),
                            ))
                            .unwrap();
                    }
                }
            }
        }

        perf_counter_inc("collisions", count);
    }

    fn mass_ratio(a: &RigidBody, b: &RigidBody) -> f32 {
        1.0 - a.mass / (a.mass + b.mass)
    }

    fn update_objects(&mut self, delta: f32) {
        let _span = tracy_span!("update positions");

        for (idx, body) in self.rbd_set.arena.iter_mut() {
            if let Some(req_velocity) = body.velocity_request.take() {
                body.position_old = body.position - req_velocity * delta;
            }

            let displacement = body.position - body.position_old;

            self.spatial_hash
                .move_point(idx.to_bits(), body.position - body.position_old);

            body.position_old = body.position;
            body.position += displacement + body.acceleration * delta * delta;

            body.acceleration = Vec2::ZERO;

            body.calculated_velocity = displacement / delta;
        }

        for (_, body) in self.rbd_set.arena.iter_mut() {
            for col_handle in body.colliders() {
                if let Some(collider) = self.col_set.get_mut(*col_handle) {
                    collider.absolute_position = body.position + collider.offset;
                }
            }
        }
    }

    fn apply_gravity(&mut self) {
        for (_, body) in self.rbd_set.arena.iter_mut() {
            body.accelerate(self.gravity);
        }
    }

    fn apply_constraints(&mut self) {
        for constraint in self.constraints.iter() {
            for (_, body) in self.rbd_set.arena.iter_mut() {
                let obj = constraint.position;
                let radius = constraint.radius;

                let to_obj = body.position - obj;
                let dist = to_obj.length();

                if dist > (radius - body.radius) {
                    let n = to_obj / dist;
                    body.position = obj + n * (radius - body.radius);
                }
            }
        }
    }

    fn integrate(&mut self, substeps: i32, delta: f32) {
        let _span = tracy_span!("integrate");
        let step_delta = delta / substeps as f32;

        for _ in 0..substeps {
            let _span = tracy_span!("substep");

            self.apply_gravity();

            if self.use_spatial_hash {
                self.spatial_collisions();
            } else {
                self.brute_force_collisions();
            }

            self.apply_constraints();
            self.update_objects(step_delta);
        }

        // for (_, obj_a) in self.rbd_set.arena.iter_mut() {
        //     for (_, obj_b) in self.rbd_set.arena.iter_mut() {
        //         // let obj = Vec2::ZERO;
        //         // let to_obj = body.position - obj;
        //         // let dist = to_obj.length();
        //         // let radius = 3.0;
        //         //
        //         // if dist > (radius - 0.5) {
        //         //     let n = to_obj / dist;
        //         //     body.position = obj + n * (dist - 0.5);
        //         // }
        //     }
        // }

        // for (i, (col_a_id, col_a)) in self.col_set.arena.iter().enumerate() {
        //     for (col_b_id, col_b) in self.col_set.arena.iter().take(i) {
        //         if !col_a.collision_groups.test(col_b.collision_groups) {
        //             continue;
        //         }
        //
        //         let distance =
        //             col_a.absolute_position.distance(col_b.absolute_position);
        //
        //         if distance < col_a.size + col_b.size {
        //             self.collision_send
        //                 .send(CollisionEvent::Started(
        //                     ColliderHandle(col_a_id),
        //                     ColliderHandle(col_b_id),
        //                     CollisionEventFlags::empty(),
        //                 ))
        //                 .unwrap();
        //         }
        //     }
        // }
    }
}
