use display::display::Display;
use rand::Rng;
use raylib::prelude::*;

mod cpu;
mod display;
mod memory;

const DISPLAY_WIDTH: i32 = 1280;
const DISPLAY_HEIGHT: i32 = 720;

const IMAGE_WIDTH: i32 = 160;
const IMAGE_HEIGHT: i32 = 144;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(DISPLAY_WIDTH as i32, DISPLAY_HEIGHT as i32)
        .title("Hello, world")
        .build();

    let mut display = Display::new(
        &mut rl,
        &thread,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        Color::WHITE,
        4,
        Vector2 { x: 150.0, y: 100.0 },
    );

    while !rl.window_should_close() {
        display.update_texture();

        display.draw_pixel(50, 50, Color::RED);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text("Hello, World", 12, 12, 20, Color::WHITE);
        display.draw_texture(&mut d, Color::WHITE);
    }
}
