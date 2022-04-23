const WIDTH: usize = 64;
const HEIGHT: usize = 32;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    opcode: u16,
    memory: [u8; 4096],
    reg: [u8; 16],
    index: u16,                    // can be u12
    pc: u16,                       // can be u12
    display: [u8; WIDTH * HEIGHT], // can be u2
    stack: [u16; 16],
    sp: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn get_chip() -> Chip8 {
        let mut chip8 = Chip8 {
            opcode: 0,
            memory: [0; 4096],
            reg: [0; 16],
            index: 0,
            pc: 0x200,
            display: [0; WIDTH * HEIGHT],
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        };

        let mut i = 0;
        while i < 80 {
            chip8.memory[i + 0x50] = FONTSET[i];
            i += 1;
        }

        return chip8;
    }

    pub fn load_game(&mut self, data: &[u8]) {
        let mut i = 0;
        while i < data.len() {
            self.memory[i + 0x200] = data[i];
            i += 1;
        }
    }

    pub fn emulate_cycle(&mut self) {
        self.fetch();
        self.decode_and_execute();

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP");
            }
            self.sound_timer -= 1;
        }
    }

    fn fetch(&mut self) {
        self.opcode =
            (self.memory[self.pc as usize] << 8 | self.memory[self.pc as usize + 1]) as u16;
    }

    fn decode_and_execute(&mut self) {}
}
