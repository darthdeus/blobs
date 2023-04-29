use blobs::*;
use glam::*;
use macroquad::{
    color::colors::*,
    input::{is_key_down, is_key_pressed},
    prelude::{Color, KeyCode},
    shapes::draw_circle,
    text::draw_text,
    time::get_frame_time,
    window::{clear_background, next_frame, screen_height, screen_width},
};

#[macroquad::main("FLOAT")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    let mut physics = Physics::new(vec2(0.0, -2.0), false);

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

        draw_circle(x, y, 15.0, YELLOW);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
        next_frame().await
    }
}
