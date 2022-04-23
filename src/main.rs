use std::fs;
use chip8::Chip8;

mod chip8;

fn main() {

    let _game = fs::read("DVN8.ch8").expect("Error reading file");
    let game = _game.as_slice();

    let mut chip8: Chip8 = Chip8::get_chip();
    chip8.load_game(game);
    
    println!("done")
}
