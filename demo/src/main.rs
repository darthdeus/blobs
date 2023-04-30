use std::time::Instant;

use blobs::{
    perf_counters::{self, perf_counters_new_frame},
    tracy_span, Constraint,
};
use thunderdome::{Arena, Index};

use glam::*;
use macroquad::{
    color::colors::*,
    input::{is_key_down, is_key_pressed},
    miniquad::conf::Platform,
    prelude::{
        is_mouse_button_down, is_mouse_button_pressed, mouse_position, set_camera, Camera2D, Color,
        KeyCode, MouseButton,
    },
    rand::gen_range,
    shapes::draw_poly,
    time::{get_fps, get_frame_time},
    window::{clear_background, next_frame, screen_height, screen_width, Conf},
};

mod rapier_engine;
mod simulation;
mod utils;

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

#[macroquad::main(window_conf)]
async fn main() {
    let gravity = vec2(0.0, -30.0);
    let mut blob_physics = blobs::Physics::new(gravity, false);
    let mut rapier_physics = RapierEngine::new(gravity);

    blob_physics.constraints.push(Constraint {
        position: Vec2::ZERO,
        radius: 4.0,
    });

    let mut sim = Simulation::new(Box::new(blob_physics));
    // let mut sim = Simulation::new(Box::new(rapier_physics));

    let mut cooldowns = Cooldowns::new();

    // physics.spawn_kinematic_ball(
    //     c.world,
    //     c.commands,
    //     0.4,
    //     random_vec(-3.0, 3.0),
    //     Some(random_vec(-1.0, 1.0)),
    //     groups(1, 1),
    //     (Sprite::new("1px".to_string(), splat(0.0), 0, RED),),
    // );

    sim.spawn_ball(RigidBodyDesc::default());

    loop {
        let delta = get_frame_time();

        perf_counters_new_frame(delta as f64);

        let physics_time = {
            let start = Instant::now();
            sim.physics.step(delta as f64);
            let end = Instant::now();

            (end - start).as_secs_f32()
        };

        if is_key_down(KeyCode::F1) && is_key_pressed(KeyCode::Escape) {
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

        for (position, radius) in sim.physics.colliders() {
            draw_circle(position, radius, RED);
        }

        // for (_, rbd) in physics.rbd_set.arena.iter() {
        //     draw_circle(rbd.position, rbd.radius, RED);
        // }

        if is_mouse_button_pressed(macroquad::prelude::MouseButton::Left) {
            sim.spawn_ball(RigidBodyDesc {
                position: vec2(gen_range(-2.0, 2.0), gen_range(-2.0, 2.0)),
                initial_velocity: Some(vec2(5.0, 2.0)),
                radius: 0.5,
                mass: 1.0,
                is_sensor: false,
                ..Default::default()
            });

            // spawn_rbd_entity(
            //     &mut physics,
            // );
        }

        let (mouse_x, mouse_y) = mouse_position();
        let _mouse_screen = vec2(mouse_x, mouse_y);
        // Working around macroquad using different version of glam.
        let mouse_world = camera.screen_to_world(macroquad::math::vec2(mouse_x, mouse_y));
        let mouse_world = vec2(mouse_world.x, mouse_world.y);

        let mut wants_ball = false;
        let mut random_radius = false;
        let mut position = random_around(vec2(1.0, 1.0), 0.1, 0.2);

        if is_mouse_button_down(MouseButton::Left) {
            if cooldowns.can_use("ball", 0.005) {
                wants_ball = true;
                random_radius = true;
                position = mouse_world;
            }
        }

        if sim.body_count() < 200 {
            if cooldowns.can_use("ball", 0.1) {
                wants_ball = true;
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            random_radius = false;
            wants_ball = true;
        }

        if wants_ball {
            sim.spawn_ball(RigidBodyDesc {
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
            });

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

        draw_circle(mouse_world, 0.3, WHITE);

        // draw_circle(vec2(1.0, 1.0), 1.0, RED);
        // draw_circle(vec2(1.0, -1.0), 1.0, GREEN);
        // draw_circle(vec2(-1.0, -1.0), 1.0, BLUE);
        // draw_circle(vec2(-1.0, 1.0), 1.0, YELLOW);

        egui_macroquad::ui(|ctx| {
            ctx.set_pixels_per_point(1.5);

            egui::Window::new("Performance")
                .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(0.0, 0.0))
                .default_width(250.0)
                .show(ctx, |ui| {
                    ui.label(format!("FPS: {}", get_fps()));
                    ui.label(format!("Physics: {}", physics_time));

                    // ui.separator();
                    // if let Some(game_loop) = c.game_loop {
                    //     game_loop.lock().performance_metrics(&mut c.world, ui);
                    // }

                    ui.separator();

                    if sim.body_count() > 0 {
                        ui.label(format!("Rigid Bodies: {}", sim.body_count()));
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
                    ui.separator();

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
                        let _span = tracy_span!("jemalloc stats");
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

        egui_macroquad::draw();

        next_frame().await
    }
}
