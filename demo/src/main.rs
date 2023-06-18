pub use blobs::{perf_counters::perf_counters_new_frame, *};
pub use thunderdome::{Arena, Index};

pub use glam::*;
pub use macroquad::{
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
mod demos;

#[cfg(feature = "rapier")]
pub use crate::rapier_engine::*;
pub use crate::simulation::*;
pub use crate::utils::*;
pub use crate::demos::*;

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

pub struct DemoContext<'a> {
    pub cooldowns: &'a mut Cooldowns,
    pub mouse_world: Vec2,
    pub delta: f64,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut demo = BallsDemo::new();
    let mut frame_index = 0;

    let mut cooldowns = Cooldowns::new();
    let mut draw_bodies = false;

    loop {
        let delta = get_frame_time() as f64;
        frame_index += 1;

        if is_key_pressed(KeyCode::Q) {
            std::process::exit(0);
        }

        if is_key_pressed(KeyCode::E) {
            draw_bodies = !draw_bodies;
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

        let (mouse_x, mouse_y) = mouse_position();
        let _mouse_screen = vec2(mouse_x, mouse_y);
        // Working around macroquad using different version of glam.
        let mouse_world = camera.screen_to_world(macroquad::math::vec2(mouse_x, mouse_y));
        let mouse_world = vec2(mouse_world.x, mouse_world.y);

        cooldowns.tick(delta as f32);

        let mut c = DemoContext {
            cooldowns: &mut cooldowns,
            mouse_world,
            delta,
        };

        perf_counters_new_frame(delta);

        if is_key_down(KeyCode::Key3) {
            // TODO:
            // let joint_iterations = self.physics.borrow_mut().joint_iterations;
            // let substeps = self.physics.borrow_mut().substeps;
            //
            // sim = make_world(gravity);
            //
            // sim.physics.joint_iterations = joint_iterations;
            // sim.physics.substeps = substeps;
        }

        let physics_time = {
            let start = instant::now();

            if frame_index > 20 {
                demo.update(&mut c);
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

        egui_macroquad::ui(|ctx| {
            ctx.set_pixels_per_point(1.5);

            let mut physics = demo.physics_mut();
            let physics= &mut *physics;

            egui::Window::new("Physics Parameters").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("substeps");
                    ui.add(egui::DragValue::new(&mut physics.substeps));
                });

                ui.horizontal(|ui| {
                    ui.label("joint iterations");
                    ui.add(egui::DragValue::new(&mut physics.joint_iterations));
                });
            });

            egui::Window::new("Performance")
                .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(0.0, 0.0))
                .default_width(250.0)
                .show(ctx, |ui| {
                    ui.label(format!("FPS: {}", get_fps()));
                    ui.label(format!("Physics: {:0.6}", physics_time));

                    ui.separator();

                    if physics.rbd_set.len() > 0 {
                        ui.label(format!("Rigid Bodies: {}", physics.rbd_set.len()));
                        ui.label(format!("Colliders: {}", physics.col_set.len()));

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

                    #[cfg(all(feature = "memory-stats", not(target_arch = "wasm32")))]
                    {
                        let _span = tracy_span!("memory_stats");
                    
                        if let Some(usage) = memory_stats::memory_stats() {
                            ui.label(format!(
                                "Physical Mem: {} MB",
                                usage.physical_mem / (1024 * 1024)
                            ));
                            ui.label(format!(
                                "Virtual Mem: {} MB",
                                usage.virtual_mem / (1024 * 1024)
                            ));
                        } else {
                            ui.label(format!(
                                "Couldn't get the current memory usage :("
                            ));
                        }
                    }

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

        });

        debug_draw_physics(demo.debug_data(), mouse_world, draw_bodies);

        egui_macroquad::draw();

        next_frame().await
    }
}

    pub fn debug_draw_physics(debug: DebugData, mouse_world: Vec2, draw_bodies: bool) {
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

        if draw_bodies {
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
}
