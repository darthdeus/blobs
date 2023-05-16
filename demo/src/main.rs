use blobs::{perf_counters::perf_counters_new_frame, *};
use thunderdome::{Arena, Index};

use glam::*;
use macroquad::{
    color::colors::*,
    input::{is_key_down, is_key_pressed},
    miniquad::conf::Platform,
    prelude::{
        is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position,
        set_camera, Camera2D, Color, KeyCode, MouseButton,
    },
    rand::gen_range,
    shapes::{draw_line, draw_poly},
    time::{get_fps, get_frame_time},
    window::{clear_background, next_frame, screen_height, screen_width, Conf},
};

#[cfg(feature = "rapier")]
mod rapier_engine;
mod simulation;
mod utils;

#[cfg(feature = "rapier")]
pub use crate::rapier_engine::*;
pub use crate::simulation::*;
pub use crate::utils::*;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn window_conf() -> Conf {
    Conf {
        window_title: "FLOAT".to_owned(),
        window_width: 1920,
        window_height: 1080,
        platform: Platform {
            swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn make_world(gravity: Vec2) -> Simulation {
    let mut blob_physics = blobs::Physics::new(gravity, false);

    blob_physics.constraints.push(Constraint {
        position: Vec2::ZERO,
        radius: 4.0,
    });

    let sim = Simulation::new(blob_physics);

    // {
    //     let spacing = 0.3;
    //     let num = 10;
    //     let w = num;
    //     let h = num;
    //
    //     let offset = -vec2(w as f32 * spacing, h as f32 * spacing) / 2.0;
    //
    //     let grid = grids::Grid::filled_with(w, h, |x, y| {
    //         let idx = sim.balls.insert(TestObject {
    //             position: Vec2::ZERO,
    //             color: YELLOW,
    //         });
    //
    //         let rbd_handle = spawn_rbd_entity(
    //             &mut sim.physics,
    //             idx,
    //             RigidBodyDesc {
    //                 position: vec2(x as f32 * spacing, y as f32 * spacing) + offset,
    //                 radius: 0.1,
    //                 // gravity_mod: 0.0,
    //                 ..Default::default()
    //             },
    //         );
    //
    //         (idx, rbd_handle)
    //     });
    //
    //     for (x, y, (_, rbd_handle_a)) in grid.iter() {
    //         if x < grid.width() - 1 {
    //             let rbd_handle_b = grid[(x + 1, y)].1;
    //             sim.physics
    //                 .create_fixed_joint(*rbd_handle_a, rbd_handle_b, Vec2::ZERO, Vec2::ZERO);
    //         }
    //         if y < grid.width() - 1 {
    //             let rbd_handle_b = grid[(x, y + 1)].1;
    //             sim.physics
    //                 .create_fixed_joint(*rbd_handle_a, rbd_handle_b, Vec2::ZERO, Vec2::ZERO);
    //         }
    //     }
    //
    //     let a = sim.balls.insert(TestObject {
    //         position: Vec2::ZERO,
    //         color: ORANGE,
    //     });
    //
    //     let cloth_pin = spawn_rbd_entity(
    //         &mut sim.physics,
    //         a,
    //         RigidBodyDesc {
    //             position: vec2(0.0, 3.5),
    //             body_type: RigidBodyType::Static,
    //             collision_groups: groups(0, 0),
    //             // gravity_mod: 0.0,
    //             ..Default::default()
    //         },
    //     );
    //
    //     let grid_anchor = grid[(5, 0)].1;
    //
    //     // let spring = blobs.springs.insert(Spring {
    //     //     rigid_body_a: grid_anchor,
    //     //     rigid_body_b: cloth_pin,
    //     //     rest_length: 1.0,
    //     //     stiffness: 3000.0,
    //     //     damping: 50.0,
    //     // });
    //
    //     sim.physics.create_fixed_joint_with_distance(
    //         grid_anchor,
    //         cloth_pin,
    //         Vec2::ZERO,
    //         Vec2::ZERO,
    //         0.1,
    //     );
    // }

    // spawn_body(&mut sim, vec2(0.5, 0.5), BLUE);
    // spawn_body(&mut sim, vec2(0.5, 0.5), BLUE);

    // {
    //     let id = sim.balls.insert(TestObject {
    //         position: Vec2::ZERO,
    //         color: PINK,
    //     });
    //
    //     let desc = RigidBodyDesc {
    //         position: vec2(-4.0, -1.0),
    //         radius: 0.4,
    //         gravity_mod: 0.0,
    //         ..Default::default()
    //     };
    //
    //     let rbd = rbd_from_desc(id, desc);
    //
    //     let rbd_handle = sim.physics.insert_rbd(rbd);
    //
    //     sim.physics.insert_collider_with_parent(
    //         collider_from_desc(
    //             id,
    //             rbd_handle,
    //             Affine2::from_translation(vec2(0.0, 0.0)),
    //             desc,
    //         ),
    //         rbd_handle,
    //     );
    //
    //     sim.physics.insert_collider_with_parent(
    //         collider_from_desc(
    //             id,
    //             rbd_handle,
    //             Affine2::from_translation(vec2(1.0, 0.0)),
    //             desc,
    //         ),
    //         rbd_handle,
    //     );
    // }

    // Fixed Joint Test
    // {
    //     let a = sim.balls.insert(TestObject {
    //         position: Vec2::ZERO,
    //         color: YELLOW,
    //     });
    //
    //     let b = sim.balls.insert(TestObject {
    //         position: Vec2::ZERO,
    //         color: GREEN,
    //     });
    //
    //     let rbd_a = spawn_rbd_entity(
    //         &mut sim.physics,
    //         a,
    //         RigidBodyDesc {
    //             position: vec2(3.0, 0.0),
    //             // gravity_mod: 0.0,
    //             ..Default::default()
    //         },
    //     );
    //
    //     let rbd_b = spawn_rbd_entity(
    //         &mut sim.physics,
    //         b,
    //         RigidBodyDesc {
    //             position: vec2(-3.0, 0.0),
    //             gravity_mod: 0.1,
    //             ..Default::default()
    //         },
    //     );
    //
    //     sim.physics
    //         .create_fixed_joint(rbd_a, rbd_b, Vec2::ZERO, Vec2::ZERO);
    // }

    sim
}

// fn spawn_body(sim: &mut Simulation, position: Vec2, color: Color) -> RigidBodyHandle {
//     let a = sim.balls.insert(TestObject { position, color });
//
//     spawn_rbd_entity(
//         &mut sim.physics,
//         a,
//         RigidBodyDesc {
//             position,
//             // gravity_mod: 0.0,
//             ..Default::default()
//         },
//     )
// }

#[derive(Copy, Clone, Debug)]
pub struct DragState {
    pub index: RigidBodyHandle,
    pub start: Vec2,
    pub offset: Vec2,
    pub spring: SpringHandle,
}

#[derive(Copy, Clone, Debug)]
pub struct HoverState {
    pub index: RigidBodyHandle,
    pub position: Vec2,
}

#[macroquad::main(window_conf)]
async fn main() {
    // let texture: Texture2D = load_texture("assets/happy-tree.png").await.unwrap();

    // let gravity = vec2(0.0, -30.0);
    let gravity = vec2(0.0, 0.0);

    let mut drag: Option<DragState> = None;
    let hover: Option<HoverState> = None;
    // let mut sim = Simulation::new(Box::new(rapier_physics));

    let mut enable_autospawn = false;
    let mut cooldowns = Cooldowns::new();

    let mut frame_index = 0;
    let mut sim = make_world(gravity);

    let a = sim.balls.insert(TestObject {
        position: Vec2::ZERO,
        color: PINK,
    });

    let mouse_rbd = spawn_rbd_entity(
        &mut sim.physics,
        a,
        RigidBodyDesc {
            position: Vec2::ZERO,
            body_type: RigidBodyType::Static,
            collision_groups: groups(0, 0),
            radius: 0.1,
            // gravity_mod: 0.0,
            ..Default::default()
        },
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

    let torque_test_rbd = spawn_rbd_entity(
        &mut sim.physics,
        a,
        RigidBodyDesc {
            position: vec2(-2.0, 2.0),
            body_type: RigidBodyType::Dynamic,
            collision_groups: groups(0, 0),
            radius: 0.1,
            // gravity_mod: 0.0,
            ..Default::default()
        },
    );

    loop {
        let delta = get_frame_time();
        frame_index += 1;

        perf_counters_new_frame(delta as f64);

        let physics_time = {
            let start = instant::now();

            if frame_index > 20 {
                sim.physics.fixed_step(delta as f64);
            }

            let end = instant::now();

            #[cfg(target_arch = "wasm32")]
            let result = (end - start);
            #[cfg(not(target_arch = "wasm32"))]
            let result = (end - start) / 1000.0;

            result
        };

        if is_key_down(KeyCode::F1) && is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::Key1) {
            enable_autospawn = !enable_autospawn;
        }

        if is_key_down(KeyCode::Key3) {
            let joint_iterations = sim.physics.joint_iterations;
            let substeps = sim.physics.substeps;

            sim = make_world(gravity);

            sim.physics.joint_iterations = joint_iterations;
            sim.physics.substeps = substeps;
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        let ratio = screen_width() / screen_height();
        let w = 20.0;
        let h = w / ratio;
        let camera = Camera2D::from_display_rect(macroquad::prelude::Rect {
            x: -w / 2.0,
            y: h / 2.0,
            w,
            h: -h,
        });

        set_camera(&camera);

        clear_background(Color::new(0.03, 0.03, 0.03, 1.0));

        // draw_rectangle(Vec2::ZERO.as_world(), 50.0, 50.0, BLACK);
        draw_circle(Vec2::ZERO, 4.0, WHITE.alpha(0.05));

        cooldowns.tick(delta);
        // for (_, rbd) in physics.rbd_set.arena.iter() {
        //     draw_circle(rbd.position, rbd.radius, RED);
        // }

        // for (_, body) in sim.physics.rbd_set.arena.iter_mut() {
        //     body.rotation += 1.0 * delta;
        // }

        let (mouse_x, mouse_y) = mouse_position();
        let _mouse_screen = vec2(mouse_x, mouse_y);
        // Working around macroquad using different version of glam.
        let mouse_world = camera.screen_to_world(macroquad::math::vec2(mouse_x, mouse_y));
        let mouse_world = vec2(mouse_world.x, mouse_world.y);

        let mut wants_ball = false;
        let mut random_radius = false;
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

            let rbd = sim.physics.get_mut_rbd(torque_test_rbd).unwrap();

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
        let debug = sim.physics.debug_data();

        for collider in debug.colliders.iter() {
            draw_circle(collider.transform.translation, collider.radius, BLUE);

            let r = collider.radius;

            let a = collider.transform.translation;
            let b = a + collider.transform.angle_dir() * r;

            draw_line(a.x, a.y, b.x, b.y, 0.05, DARKBLUE);

            // draw_texture_ex(
            //     texture,
            //     collider.transform.translation.x - r,
            //     collider.transform.translation.y - r,
            //     BLUE.alpha(0.4),
            //     DrawTextureParams {
            //         dest_size: Some(macroquad::prelude::vec2(r * 2.0, r * 2.0)),
            //         rotation: angle,
            //         ..Default::default()
            //     },
            // );
        }

        for body in debug.bodies.iter() {
            if body.transform.translation.distance(mouse_world) < 0.1 {
                continue;
            }

            let r = 0.5;
            draw_circle(body.transform.translation, r, PINK.alpha(0.5));

            let a = body.transform.translation;
            let b = a + body.transform.angle_dir() * r;

            draw_line(a.x, a.y, b.x, b.y, 0.05, DARKBLUE);
        }

        for spring in debug.springs.iter() {
            draw_line(
                spring.body_a.x,
                spring.body_a.y,
                spring.body_b.x,
                spring.body_b.y,
                0.1,
                BLUE,
            );
        }

        for joint in debug.joints.iter() {
            draw_line(
                joint.body_a.x,
                joint.body_a.y,
                joint.body_b.x,
                joint.body_b.y,
                0.05,
                YELLOW.alpha(0.5),
            );
        }

        // draw_circle(mouse_world, 0.3, WHITE);

        // draw_circle(vec2(1.0, 1.0), 1.0, RED);
        // draw_circle(vec2(1.0, -1.0), 1.0, GREEN);
        // draw_circle(vec2(-1.0, -1.0), 1.0, BLUE);
        // draw_circle(vec2(-1.0, 1.0), 1.0, YELLOW);

        egui_macroquad::ui(|ctx| {
            ctx.set_pixels_per_point(1.5);

            egui::Window::new("Physics Parameters").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("substeps");
                    ui.add(egui::DragValue::new(&mut sim.physics.substeps));
                });

                ui.horizontal(|ui| {
                    ui.label("joint iterations");
                    ui.add(egui::DragValue::new(&mut sim.physics.joint_iterations));
                });
            });

            egui::Window::new("Performance")
                .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(0.0, 0.0))
                .default_width(250.0)
                .show(ctx, |ui| {
                    ui.label(format!("FPS: {}", get_fps()));
                    ui.label(format!("Physics: {:0.6}", physics_time));

                    ui.separator();

                    if sim.physics.rbd_set.len() > 0 {
                        ui.label(format!("Rigid Bodies: {}", sim.physics.rbd_set.len()));
                        ui.label(format!("Colliders: {}", sim.collider_count()));

                        ui.separator();
                    }

                    // Display performance counters
                    ui.label("Perf Counters");
                    for (counter_name, counter) in
                        perf_counters::PerfCounters::global().counters.iter()
                    {
                        ui.label(format!(
                            "{:<15}: {:<15.0}",
                            counter_name, counter.decayed_average,
                        ));
                    }

                    // #[cfg(not(target_arch = "wasm32"))]
                    // {
                    //     let _span = tracy_span!("memory_stats");
                    //
                    //     if let Some(usage) = memory_stats::memory_stats() {
                    //         ui.label(format!(
                    //             "Physical Mem: {} MB",
                    //             usage.physical_mem / (1024 * 1024)
                    //         ));
                    //         ui.label(format!(
                    //             "Virtual Mem: {} MB",
                    //             usage.virtual_mem / (1024 * 1024)
                    //         ));
                    //     } else {
                    //         ui.label(format!(
                    //             "Couldn't get the current memory usage :("
                    //         ));
                    //     }
                    // }

                    #[cfg(feature = "jemalloc")]
                    {
                        let _span = blobs::tracy_span!("jemalloc stats");
                        ui.separator();

                        ui.label("jemalloc");

                        jemalloc_ctl::epoch::advance().unwrap();

                        let allocated = jemalloc_ctl::stats::allocated::read().unwrap();
                        let resident = jemalloc_ctl::stats::resident::read().unwrap();
                        ui.label(format!("{} MB allocated", allocated / (1024 * 1024)));
                        ui.label(format!("{} MB resident", resident / (1024 * 1024)));
                    }
                });

            if !ctx.wants_pointer_input() {
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
            }
        });

        if sim.body_count() < 200 && enable_autospawn {
            if cooldowns.can_use("ball", 0.1) {
                wants_ball = true;
            }
        }

        if wants_ball {
            sim.spawn_ball(
                RigidBodyDesc {
                    position,
                    initial_velocity: Some(random_circle(3.0)),
                    radius: if random_radius {
                        gen_range(0.05, 0.2)
                    } else {
                        gen_range(0.05, 0.1)
                    },
                    mass: 1.0,
                    is_sensor: false,
                    ..Default::default()
                },
                RED,
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

        egui_macroquad::draw();

        next_frame().await
    }
}
