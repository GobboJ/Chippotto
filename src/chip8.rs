use rand::Rng;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

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

    pub fn get_display(&self) -> [u8; WIDTH * HEIGHT] {
        self.display
    }

    fn clear_display(&mut self) {
        self.display = [0; WIDTH * HEIGHT];
    }

    fn fetch(&mut self) {
        self.opcode = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[self.pc as usize + 1] as u16);
    }

    fn decode_and_execute(&mut self) {
        match self.opcode & 0xF000 {
            0x000 => match self.opcode & 0x00FF {
                0x00E0 => {
                    self.clear_display();
                    self.pc += 2;
                }
                0x00EE => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
                _ => {
                    println!("{} is not a valid opcode", self.opcode);
                    self.pc += 2;
                }
            },
            0x1000 => {
                self.pc = self.opcode & 0x0FFF;
            }
            0x2000 => {
                self.stack[self.sp as usize] = self.pc + 2;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            }
            0x3000 => {
                let i = ((self.opcode & 0x0F00) >> 8) as usize;
                if self.reg[i] != (self.opcode & 0x00FF) as u8 {
                    self.pc += 2;
                } else {
                    self.pc += 4;
                }
            }
            0x4000 => {
                let i = ((self.opcode & 0x0F00) >> 8) as usize;
                if self.reg[i] == (self.opcode & 0x00FF) as u8 {
                    self.pc += 2;
                } else {
                    self.pc += 4;
                }
            }
            0x5000 => {
                let i = ((self.opcode & 0x0F00) >> 8) as usize;
                let j = ((self.opcode & 0x00F0) >> 4) as usize;
                if self.reg[i] != self.reg[j] {
                    self.pc += 2;
                } else {
                    self.pc += 4;
                }
            }
            0x6000 => {
                let i = ((self.opcode & 0x0F00) >> 8) as usize;
                self.reg[i] = (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }
            0x7000 => {
                let i = ((self.opcode & 0x0F00) >> 8) as usize;
                self.reg[i] = (self.reg[i] as u16 + (self.opcode & 0x00FF)) as u8;
                self.pc += 2;
            }
            0x8000 => {
                let i = ((self.opcode & 0x0F00) >> 8) as usize;
                let j = ((self.opcode & 0x00F0) >> 4) as usize;
                match self.opcode & 0x000F {
                    0x0000 => {
                        self.reg[i] = self.reg[j];
                    }
                    0x0001 => {
                        self.reg[i] |= self.reg[j];
                    }
                    0x0002 => {
                        self.reg[i] &= self.reg[j];
                    }
                    0x0003 => {
                        self.reg[i] ^= self.reg[j];
                    }
                    0x0004 => {
                        if self.reg[i] as u16 + self.reg[j] as u16 > 0xFF {
                            self.reg[15] = 1;
                        } else {
                            self.reg[15] = 0;
                        }
                        self.reg[i] = (self.reg[i] as u16 + self.reg[j] as u16) as u8;
                    }
                    0x0005 => {
                        if self.reg[i] > self.reg[j] {
                            self.reg[15] = 1;
                        } else {
                            self.reg[15] = 0;
                        }
                        self.reg[i] = (self.reg[i] as i16 - self.reg[j] as i16) as u8;
                    }
                    0x0006 => {
                        self.reg[i] = self.reg[j];
                        self.reg[i] >>= 1;
                        self.reg[15] = self.reg[j] & 0b0000_0001;
                    }
                    0x0007 => {
                        if self.reg[j] > self.reg[i] {
                            self.reg[15] = 1;
                        } else {
                            self.reg[15] = 0;
                        }
                        self.reg[i] = (self.reg[j] as i16 - self.reg[i] as i16) as u8;
                    }
                    0x000E => {
                        self.reg[i] = self.reg[j];
                        self.reg[i] <<= 1;
                        self.reg[15] = (self.reg[j] & 0b1000_0000) >> 7;
                    }
                    _ => {
                        println!("{} is not a valid opcode", self.opcode);
                    }
                }
                self.pc += 2;
            }
            0x9000 => {
                let i = ((self.opcode & 0x0F00) >> 8) as usize;
                let j = ((self.opcode & 0x00F0) >> 4) as usize;
                if self.reg[i] == self.reg[j] {
                    self.pc += 2;
                } else {
                    self.pc += 4;
                }
            }
            0xA000 => {
                self.index = self.opcode & 0x0FFF;
                self.pc += 2;
            }
            0xB000 => {
                self.pc = (self.opcode & 0x0FFF) + self.reg[0] as u16;
            }
            0xC000 => {
                let i = ((self.opcode & 0x0F00) >> 8) as usize;
                self.reg[i] = rand::thread_rng().gen_range(0..255) & (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }
            0xD000 => {
                // Drawing
                let x = self.reg[((self.opcode & 0x0F00) >> 8) as usize] as u16;
                let y = self.reg[((self.opcode & 0x00F0) >> 4) as usize] as u16;
                let h = self.opcode & 0x000F;
                let mut pixel = 0;

                self.reg[15] = 0;
                for yline in 0..h {
                    pixel = self.memory[(self.index + yline as u16) as usize];
                    for xline in 0..8 {
                        if (pixel & (0x80 >> xline)) != 0 {
                            if self.display[(x + xline + ((y + yline) * 64)) as usize] == 1 {
                                self.reg[15] = 1;
                            }
                            self.display
                                [((x + xline + ((y + yline) * 64)) % (64 * 32)) as usize] ^= 1;
                        }
                    }
                }
                self.pc += 2;
            }
            0xE000 => {
                // Keys
                self.pc += 2;
            }
            0xF000 => {
                // Keys, timers, other stuff
                match self.opcode & 0x00FF {
                    0x0007 => {
                        let i = ((self.opcode & 0x0F00) >> 8) as usize;
                        self.reg[i] = self.delay_timer;
                    }
                    0x000A => {
                        // TODO
                        println!("TODO");
                    }
                    0x0015 => {
                        let i = ((self.opcode & 0x0F00) >> 8) as usize;
                        self.delay_timer = self.reg[i];
                    }
                    0x0018 => {
                        let i = ((self.opcode & 0x0F00) >> 8) as usize;
                        self.sound_timer = self.reg[i];
                    }
                    0x001E => {
                        let i = ((self.opcode & 0x0F00) >> 8) as usize;
                        self.index += self.reg[i] as u16;
                    }
                    0x0029 => {
                        let i = ((self.opcode & 0x0F00) >> 8) as usize;
                        self.index = self.reg[i] as u16 * 5;
                    }
                    0x0033 => {
                        let i = ((self.opcode & 0x0F00) >> 8) as usize;
                        self.memory[self.index as usize] = self.reg[i] / 100;
                        self.memory[self.index as usize + 1] = (self.reg[i] % 100) / 10;
                        self.memory[self.index as usize + 2] = self.reg[i] % 10;
                    }
                    0x0055 => {
                        let i = ((self.opcode & 0x0F00) >> 8) as usize;
                        for v in 0..i + 1 {
                            self.memory[self.index as usize + v] = self.reg[v];
                        }
                        self.index += i as u16 + 1;
                    }
                    0x0065 => {
                        let i = ((self.opcode & 0x0F00) >> 8) as usize;
                        for v in 0..i + 1 {
                            self.reg[v] = self.memory[self.index as usize + v];
                        }
                        self.index += i as u16 + 1;
                    }
                    _ => {
                        println!("{} is not a valid opcode", self.opcode)
                    }
                }
                self.pc += 2;
            }
            _ => {
                self.pc += 2;
            }
        }
    }
}
