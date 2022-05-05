
use raylib::prelude::*;
mod chip8;
use chip8::Chip8;
use std::fs;

const SCALING: usize = 10;
const WINDOW_WIDTH: usize = chip8::WIDTH * SCALING;
const WINDOW_HEIGHT: usize = chip8::HEIGHT * SCALING;

fn main() {

    // Initialize Raylib window
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("Chippotto")
        .build();
    //rl.set_target_fps(60);
        

    // Load game
    let _game = fs::read("test_opcode.ch8").expect("[!] Error reading file");
    let game = _game.as_slice();
    let mut chip8: Chip8 = Chip8::get_chip();
    chip8.load_game(game);

    let mut n = 0;

    // Main loop
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let img = chip8.get_display();

        for y in 0..32 {
            for x in 0..64 {
                if img[64 * y + x] == 1 {
                    d.draw_rectangle((x * SCALING) as i32, (y * SCALING) as i32, SCALING as i32, SCALING as i32, Color::WHITE)
                }
            }
        }
       

        d.clear_background(Color::BLACK);

        chip8.emulate_cycle();
        n += 1;
        //println!("N: {}", n);
    }
}
