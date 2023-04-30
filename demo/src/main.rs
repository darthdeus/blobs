use blobs::*;
use glam::*;
use macroquad::{
    color::colors::*,
    input::{is_key_down, is_key_pressed},
    prelude::{is_mouse_button_pressed, set_camera, Camera2D, Color, KeyCode},
    shapes::draw_circle,
    text::draw_text,
    time::get_frame_time,
    window::{clear_background, next_frame, screen_height, screen_width}, rand::gen_range,
};

pub trait ColorExtensions {
    fn alpha(&self, value: f32) -> Color;
    fn mix(&self, other: Color, value: f32) -> Color;
    // fn egui(&self) -> egui::Color32;
    fn darken(&self, amount: f32) -> Color;
}

impl ColorExtensions for Color {
    fn alpha(&self, value: f32) -> Color {
        Color::new(self.r, self.g, self.b, value)
    }

    fn mix(&self, other: Color, value: f32) -> Color {
        let a = 1.0 - value;
        let b = value;

        Color::new(
            self.r * a + other.r * b,
            self.g * a + other.g * b,
            self.b * a + other.b * b,
            self.a * a + other.a * b,
        )
    }

    // fn egui(&self) -> egui::Color32 {
    //     egui::Color32::from_rgba_unmultiplied(
    //         (self.r * 255.0) as u8,
    //         (self.g * 255.0) as u8,
    //         (self.b * 255.0) as u8,
    //         (self.a * 255.0) as u8,
    //     )
    // }

    fn darken(&self, amount: f32) -> Color {
        let amount = 1.0 - amount;
        Color::new(self.r * amount, self.g * amount, self.b * amount, self.a)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RigidBodyDesc {
    pub position: Vec2,
    pub initial_velocity: Option<Vec2>,
    pub radius: f32,
    pub mass: f32,
    pub is_sensor: bool,
    pub collision_groups: InteractionGroups,
}

pub fn spawn_rbd_entity(physics: &mut Physics, desc: RigidBodyDesc) {
    // let entity = world.reserve_entity();
    // let user_data: u128 = entity.to_bits().get().into();

    let rbd = RigidBody {
        position: desc.position,
        position_old: desc.position,
        mass: desc.mass,
        velocity_request: desc.initial_velocity,
        calculated_velocity: Vec2::ZERO,
        acceleration: Vec2::ZERO,
        rotation: 0.0,
        scale: Vec2::ONE,
        radius: desc.radius,
        // angular_velocity: 0.0,
        colliders: vec![],
        user_data: 0,
        // user_data,
        body_type: RigidBodyType::KinematicVelocityBased,
        collision_groups: desc.collision_groups,
    };

    let rbd_handle = physics.insert_rbd(rbd);

    let collider = Collider {
        offset: Vec2::ZERO,
        absolute_position: desc.position,
        rotation: 0.0,
        scale: Vec2::ONE,
        // user_data,
        user_data: 0,
        parent: Some(ColliderParent {
            handle: rbd_handle,
            pos_wrt_parent: Vec2::ZERO,
        }),
        radius: desc.radius,
        flags: ColliderFlags {
            is_sensor: desc.is_sensor,
        },
        collision_groups: desc.collision_groups,
        shape: Box::new(Ball {
            radius: desc.radius,
        }),
    };

    // let collider = ColliderBuilder::ball(size)
    //     .user_data(user_data)
    //     .active_events(ActiveEvents::COLLISION_EVENTS)
    //     .active_collision_types(
    //         ActiveCollisionTypes::default()
    //             | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
    //     )
    //     .collision_groups(collision_groups);

    physics.insert_collider_with_parent(collider, rbd_handle);

    // commands.insert(
    //     entity,
    //     (
    //         RbdHandleComponent(rbd_handle),
    //         Transform::position(desc.position),
    //         Velocity(desc.initial_velocity.unwrap_or(Vec2::ZERO)),
    //     ),
    // );
    // commands.insert(entity, components);
    //
    // entity
}

#[macroquad::main("FLOAT")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    let mut physics = Physics::new(vec2(0.0, -300.0), false);

    // physics.spawn_kinematic_ball(
    //     c.world,
    //     c.commands,
    //     0.4,
    //     random_vec(-3.0, 3.0),
    //     Some(random_vec(-1.0, 1.0)),
    //     groups(1, 1),
    //     (Sprite::new("1px".to_string(), splat(0.0), 0, RED),),
    // );

    physics.constraints.push(Constraint {
        position: Vec2::ZERO,
        radius: 4.0,
    });

    spawn_rbd_entity(
        &mut physics,
        RigidBodyDesc {
            position: Vec2::ZERO,
            initial_velocity: Some(vec2(5.0, 2.0)),
            radius: 0.5,
            mass: 1.0,
            is_sensor: false,
            collision_groups: InteractionGroups::default(),
        },
    );

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        physics.step(8, get_frame_time() as f64);

        if is_key_down(KeyCode::F1) && is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::Right) {
            x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            x -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            y += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            y -= 1.0;
        }

        let ratio = screen_width() / screen_height();
        let w = 20.0;
        let h = w / ratio;

        set_camera(&Camera2D::from_display_rect(macroquad::prelude::Rect {
            x: -w / 2.0,
            y: -h / 2.0,
            w,
            h,
        }));

        draw_circle(x, y, 15.0, YELLOW);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);

        clear_background(BLACK);
        // draw_rectangle(Vec2::ZERO.as_world(), 50.0, 50.0, BLACK);
        draw_circle(0.0, 0.0, 4.0, WHITE.alpha(0.1));

        for (_, rbd) in physics.rbd_set.arena.iter() {
            draw_circle(rbd.position.x, rbd.position.y, rbd.radius, RED);
        }

        if is_mouse_button_pressed(macroquad::prelude::MouseButton::Left) {
            spawn_rbd_entity(
                &mut physics,
                RigidBodyDesc {
                    position: vec2(gen_range(-2.0, 2.0), gen_range(-2.0, 2.0)),
                    initial_velocity: Some(vec2(5.0, 2.0)),
                    radius: 0.5,
                    mass: 1.0,
                    is_sensor: false,
                    collision_groups: InteractionGroups::default(),
                },
            );
        }

        // let mut wants_ball = false;
        // let mut random_radius = false;
        // let mut position = random_around(vec2(1.0, 1.0), 0.1, 0.2);
        //
        // if is_mouse_button_down(MouseButton::Left) {
        //     if c.cooldowns.can_use("ball", 0.005) {
        //         wants_ball = true;
        //         random_radius = true;
        //         position = c.mouse_world;
        //     }
        // }
        //
        // if c.physics.rbd_count() < 200 {
        //     if c.cooldowns.can_use("ball", 0.1) {
        //         wants_ball = true;
        //     }
        // }
        //
        // if is_mouse_button_pressed(MouseButton::Right) {
        //     random_radius = false;
        //     wants_ball = true;
        // }
        //
        // if wants_ball {
        //     c.physics.spawn_kinematic_ball(
        //         c.world,
        //         c.commands,
        //         if random_radius {
        //             gen_range(0.05, 0.2)
        //         } else {
        //             gen_range(0.05, 0.1)
        //         },
        //         position,
        //         Some(random_vec(1.0, 50.0)),
        //         groups(1, 1),
        //         (Sprite::new("1px".to_string(), splat(0.0), 0, RED),),
        //     );
        // }

        next_frame().await
    }
}
