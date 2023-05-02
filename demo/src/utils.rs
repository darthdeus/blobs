use crate::*;
use std::{
    collections::hash_map::{DefaultHasher, Entry},
    collections::HashMap,
    hash::{Hash, Hasher},
};

pub fn flip_coin(p: f32) -> bool {
    gen_range(0.0, 1.0) < p
}

pub fn random_dir() -> Vec2 {
    let angle = gen_range(0.0, std::f32::consts::PI * 2.0);

    Vec2::new(angle.cos(), angle.sin())
}

pub fn random_vec(min: f32, max: f32) -> Vec2 {
    random_dir() * gen_range(min, max)
}

pub fn random_offset(radius: f32) -> Vec2 {
    random_dir() * gen_range(0.0, radius)
}

pub fn random_circle(radius: f32) -> Vec2 {
    random_offset(radius)
}

pub fn random_box(center: Vec2, size: Vec2) -> Vec2 {
    center
        + vec2(
            gen_range(-size.x, size.x) / 2.0,
            gen_range(-size.y, size.y) / 2.0,
        )
}

pub fn random_around(position: Vec2, min: f32, max: f32) -> Vec2 {
    position + random_vec(min, max)
}

pub fn random() -> f32 {
    gen_range(0.0, 1.0)
}

pub fn default_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

pub struct Cooldowns {
    data: HashMap<u64, f32>,
}

impl Cooldowns {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    pub fn tick(&mut self, delta: f32) {
        for (_, val) in self.data.iter_mut() {
            if *val > 0.0 {
                *val -= delta;
            }
        }
    }

    pub fn can_use<T: Hash>(&mut self, key: T, total: f32) -> bool {
        match self.data.entry(default_hash(&key)) {
            Entry::Occupied(mut slot) => {
                let result = *slot.get() <= 0.0;

                if result {
                    slot.insert(total);
                }

                result
            }
            Entry::Vacant(slot) => {
                slot.insert(total);
                true
            }
        }
    }

    pub fn can_use_random_not_first<T: Hash>(&mut self, key: T, total: f32, spread: f32) -> bool {
        match self.data.entry(default_hash(&key)) {
            Entry::Occupied(mut slot) => {
                let result = *slot.get() <= 0.0;

                if result {
                    let half = (spread * total) / 2.0;
                    slot.insert(total + gen_range(-half, half));
                }

                result
            }
            Entry::Vacant(slot) => {
                let half = (spread * total) / 2.0;
                slot.insert(total + gen_range(-half, half));
                false
            }
        }
    }
}

pub fn draw_circle(position: Vec2, radius: f32, color: Color) {
    // macroquad::shapes::draw_circle(position.x, position.y, radius, color);
    draw_poly(
        position.x, position.y, 40, // 40 * (radius as u8).max(1),
        radius, 0., color,
    );
}

pub trait ColorExtensions {
    fn alpha(&self, value: f32) -> Color;
    fn mix(&self, other: Color, value: f32) -> Color;
    fn egui(&self) -> egui::Color32;
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

    fn egui(&self) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        )
    }

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
    pub gravity_mod: f32,
    pub collision_groups: blobs::InteractionGroups,
    pub body_type: RigidBodyType,
}

impl Default for RigidBodyDesc {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            initial_velocity: None,
            radius: 0.5,
            mass: 1.0,
            is_sensor: false,
            gravity_mod: 1.0,
            collision_groups: blobs::InteractionGroups::default(),
            body_type: RigidBodyType::Dynamic,
        }
    }
}
