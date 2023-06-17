use crate::*;

pub struct BallsDemo {
    pub enable_autospawn: bool,
    pub random_radius: bool,

    pub sim: Simulation,
    pub drag: Option<DragState>,
    pub hover: Option<HoverState>,
}

impl BallsDemo {
    pub fn new() -> Self {
        // let gravity = vec2(0.0, -30.0);
        let gravity = vec2(0.0, 0.0);

        // let mut sim = Simulation::new(Box::new(rapier_physics));

        let mut sim = make_world(gravity);

        let a = sim.balls.insert(TestObject {
            position: Vec2::ZERO,
            color: PINK,
        });

        let mouse_rbd = sim.spawn_ball(
            RigidBodyDesc {
                position: Vec2::ZERO,
                body_type: RigidBodyType::Static,
                collision_groups: groups(0, 0),
                radius: 0.1,
                // gravity_mod: 0.0,
                ..Default::default()
            },
            RED,
        );

        // let pos = vec2(5.0, 5.0);
        //
        // let rbd = RigidBody {
        //     position: pos,
        //     position_old: pos,
        //     gravity_mod: 1.0,
        //     mass: 1.0,
        //     velocity_request: Some(vec2(1.0, 0.)),
        //     calculated_velocity: Vec2::ZERO,
        //     acceleration: Vec2::ZERO,
        //     rotation: 0.0,
        //     scale: Vec2::ONE,
        //     // angular_velocity: 0.0,
        //     colliders: vec![],
        //     user_data: 0,
        //     connected_joints: vec![],
        //     body_type: RigidBodyType::KinematicVelocityBased,
        //     collision_groups: groups(0, 0),
        // };
        //
        // let rbd_handle = sim.physics.insert_rbd(rbd);

        let a = sim.balls.insert(TestObject {
            position: Vec2::ZERO,
            color: GREEN,
        });

        // let torque_test_rbd = spawn_rbd_entity(
        //     &mut sim.physics,
        //     a,
        //     RigidBodyDesc {
        //         position: vec2(-2.0, 2.0),
        //         body_type: RigidBodyType::Dynamic,
        //         collision_groups: groups(0, 0),
        //         radius: 0.1,
        //         // gravity_mod: 0.0,
        //         ..Default::default()
        //     },
        // );

        Self {
            enable_autospawn: true,
            random_radius: true,

            sim,
            drag: None,
            hover: None,
        }
    }

    pub fn physics(&self) -> Ref<Physics> {
        self.sim.physics.borrow()
    }

    pub fn physics_mut(&self) -> RefMut<Physics> {
        self.sim.physics.borrow_mut()
    }
}

impl Demo for BallsDemo {
    fn update(&mut self, c: &mut DemoContext) {
        if is_key_down(KeyCode::Key1) {
            self.enable_autospawn = !self.enable_autospawn;
        }
        let mut physics = self.physics_mut();
        let physics = &mut *physics;

        physics.fixed_step(c.delta);

        let mut wants_ball = false;
        let position = random_around(vec2(1.0, 1.0), 0.1, 0.2);

        // for (index, object) in sim.balls.iter() {
        //     let collider = sim.physics.col_set.arena.get(index).unwrap();
        //     let rbd_handle = collider.parent.unwrap();
        //
        //     let mut hovered = false;
        //
        //     if mouse_rbd != rbd_handle {
        //         if mouse_world.distance(collider.absolute_translation()) < collider.radius {
        //             hovered = true;
        //
        //             hover = Some(HoverState {
        //                 index: rbd_handle,
        //                 position: collider.absolute_translation(),
        //             });
        //         }
        //     }
        //
        //     let color = if hovered {
        //         RED.mix(object.color, 0.2)
        //     } else {
        //         object.color
        //     };
        //
        //     let rbd = sim.physics.get_rbd(rbd_handle).unwrap();
        //
        //     draw_circle(collider.absolute_translation(), collider.radius, color);
        //     let a = collider.absolute_translation();
        //     let b = a + vec2(rbd.rotation.cos(), rbd.rotation.sin()) * 0.4;
        //     draw_line(a.x, a.y, b.x, b.y, 0.05, YELLOW);
        //
        //     let r = collider.radius;
        //
        //     draw_texture_ex(
        //         texture,
        //         collider.absolute_translation().x - r,
        //         collider.absolute_translation().y - r,
        //         color.alpha(0.4),
        //         DrawTextureParams {
        //             dest_size: Some(macroquad::prelude::vec2(
        //                 collider.radius * 2.0,
        //                 collider.radius * 2.0,
        //             )),
        //             rotation: rbd.rotation,
        //             ..Default::default()
        //         },
        //     );
        // }

        if physics.rbd_set.len() < 200 && self.enable_autospawn {
            if c.cooldowns.can_use("ball", 0.1) {
                wants_ball = true;
            }
        }

        if wants_ball {
            spawn_rbd_entity(
                physics,
                // Just temporarily for now
                thunderdome::Index::from_bits(1 << 32).unwrap(),
                RigidBodyDesc {
                    position,
                    initial_velocity: Some(random_circle(3.0)),
                    radius: if self.random_radius {
                        gen_range(0.05, 0.2)
                    } else {
                        gen_range(0.05, 0.1)
                    },
                    mass: 1.0,
                    is_sensor: false,
                    ..Default::default()
                },
            );

            // physics.spawn_kinematic_ball(
            //     world,
            //     c.commands,
            //     if random_radius {
            //         gen_range(0.05, 0.2)
            //     } else {
            //         gen_range(0.05, 0.1)
            //     },
            //     position,
            //     Some(random_vec(1.0, 50.0)),
            //     groups(1, 1),
            //     (Sprite::new("1px".to_string(), splat(0.0), 0, RED),),
            // );
        }
    }

    fn debug_data(&self) -> DebugData {
        self.physics().debug_data()
    }
}
