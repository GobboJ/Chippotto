use chip8::Chip8;
use raylib::{ffi::IsKeyDown, prelude::*};
use std::{env, fs};
mod chip8;

const SCALING: usize = 10;
const WINDOW_WIDTH: usize = chip8::WIDTH * SCALING;
const WINDOW_HEIGHT: usize = chip8::HEIGHT * SCALING;
const MAIN_COLOR: Color = Color::GOLD;
const BACK_COLOR: Color = Color::DARKGREEN;

fn main() {
    // Initialize Raylib window
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("Chippotto")
        .build();
    rl.set_target_fps(500);

    // Load game
    let args: Vec<String> = env::args().collect();
    let _game = fs::read(&args[1]).expect("[!] Error reading file");
    rl.set_window_title(&thread, &format!("Chippotto [{}]", &args[1]));
    let game = _game.as_slice();
    let mut chip8: Chip8 = Chip8::get_chip();
    chip8.load_game(game);

    // Main loop
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let img = chip8.get_display();

        unsafe {
            for i in 48..58 {
                if IsKeyDown(i) {
                    chip8.key_press(i as u8 - 48);
                }
            }
            for i in 65..71 {
                if IsKeyDown(i) {
                    chip8.key_press(i as u8 - 55);
                }
            }
        }

        for y in 0..32 {
            for x in 0..64 {
                if img[64 * y + x] == 1 {
                    d.draw_rectangle(
                        (x * SCALING) as i32,
                        (y * SCALING) as i32,
                        SCALING as i32,
                        SCALING as i32,
                        MAIN_COLOR,
                    )
                }
            }
        }

        chip8.emulate_cycle();

        d.clear_background(BACK_COLOR);
    }
}
