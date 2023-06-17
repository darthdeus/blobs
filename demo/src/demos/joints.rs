
        {
            let mut force = None;

            if is_key_down(KeyCode::W) {
                force = Some(vec2(0.0, 1.0));
            }
            if is_key_down(KeyCode::S) {
                force = Some(vec2(0.0, -1.0));
            }
            if is_key_down(KeyCode::A) {
                force = Some(vec2(-1.0, 0.0));
            }
            if is_key_down(KeyCode::D) {
                force = Some(vec2(1.0, 0.0));
            }

            let rbd = self
                .physics
                .borrow_mut()
                .get_mut_rbd(torque_test_rbd)
                .unwrap();

            let force_point = if is_key_down(KeyCode::LeftShift) {
                mouse_world
            } else {
                rbd.position
            };

            if let Some(force) = force {
                rbd.apply_force_at_point(force, force_point);

                let a = force_point;
                let b = a + force;

                draw_line(a.x, a.y, b.x, b.y, 0.2, RED);
            }
        }

        // draw_circle(mouse_world, 0.3, WHITE);


        // draw_circle(vec2(1.0, 1.0), 1.0, RED);
        // draw_circle(vec2(1.0, -1.0), 1.0, GREEN);
        // draw_circle(vec2(-1.0, -1.0), 1.0, BLUE);
        // draw_circle(vec2(-1.0, 1.0), 1.0, YELLOW);
        //

        // if !ctx.wants_pointer_input() {
        // if is_mouse_button_down(MouseButton::Left) {
        //     if cooldowns.can_use("ball", 0.005) {
        //         wants_ball = true;
        //         random_radius = true;
        //         position = mouse_world;
        //     }
        // }

        if let Some(hover) = hover {
            if drag.is_none() && is_mouse_button_pressed(MouseButton::Left) {
                let spring = sim.physics.springs.insert(Spring {
                    rigid_body_a: hover.index,
                    rigid_body_b: mouse_rbd,
                    rest_length: 1.0,
                    stiffness: 1000.0,
                    damping: 50.0,
                });

                drag = Some(DragState {
                    spring: SpringHandle(spring),
                    index: hover.index,
                    start: hover.position,
                    offset: mouse_world - hover.position,
                });
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(drag) = drag {
                sim.physics.springs.remove(*drag.spring);
            }

            drag = None;
        }

        sim.physics.rbd_set.get_mut(mouse_rbd).unwrap().position = mouse_world;

        if let Some(_drag) = drag {
            if is_mouse_button_down(MouseButton::Left) {
                // let blobs = sim.cast_physics::<blobs::Physics>();

                // let rbd = blobs.rbd_set.arena.get_mut(drag.index.0).unwrap();

                // rbd.position = drag.start + mouse_world - drag.offset;
                // rbd.position = mouse_world;
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            random_radius = false;
            wants_ball = true;
        }

        // if is_mouse_button_pressed(macroquad::prelude::MouseButton::Left) {
        //     sim.spawn_ball(
        //         RigidBodyDesc {
        //             position: vec2(gen_range(-2.0, 2.0), gen_range(-2.0, 2.0)),
        //             initial_velocity: Some(vec2(5.0, 2.0)),
        //             radius: 0.5,
        //             mass: 1.0,
        //             is_sensor: false,
        //             ..Default::default()
        //         },
        //         RED,
        //     );
        //
        //     // spawn_rbd_entity(
        //     //     &mut physics,
        //     // );
        // }
        // }
