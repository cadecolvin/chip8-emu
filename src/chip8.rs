use rand::{thread_rng, Rng};

pub struct Chip8 {
    pub stack: [u16; 16],
    pub stack_pointer: u8,
    pub program_counter: u16,

    pub memory: [u8; 4096],

    pub registers: [u8; 16],
    pub addr_register: u16,

    pub screen: [[bool; 64]; 32],

    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            stack: [0; 16],
            stack_pointer: 0,
            program_counter: 0,
            memory: [0; 4096],
            registers: [0; 16],
            addr_register: 0,
            screen: [[false; 64]; 32],
            delay_timer: 0,
            sound_timer: 0,
        }
    }
    pub fn execute_cycle(&mut self) {
        let opcode = self.read_word();
        self.process_opcode(opcode);
    }

    fn read_word(&self) -> u16 {
        (self.memory[self.program_counter as usize] as u16) << 8 |
        (self.memory[(self.program_counter+1) as usize] as u16)
    }

    fn process_opcode(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = (opcode & 0x00F0) as usize;
        let map = (opcode & 0xF000) >> 12;

        match map {
            0x1 => {
                self.program_counter = opcode & 0x0FFF;
            },
            0x2 => {
                unimplemented!();
            },
            0x3 => {
                let target_val = (opcode & 0x00FF) as u8;
                if self.registers[x] == target_val {
                    self.program_counter += 1;
                }
            },
            0x4 => {
                let target_val = (opcode & 0x00FF) as u8;
                if self.registers[x] != target_val {
                    self.program_counter += 1;
                }
            },
            0x5 => {
                if self.registers[x] == self.registers[y] {
                    self.program_counter += 1;
                }
            },
            0x6 => {
                let value = (opcode & 0x00FF) as u8;
                self.registers[x] = value;
            },
            0x7 => {
                let value = (opcode & 0x00FF) as u8;
                self.registers[x] = self.registers[x].wrapping_add(value);
            },
            0x8 => {
                let sub_map = (opcode & 0x000F) as u8;
                match sub_map {
                    0x0 => self.registers[x] = self.registers[y],
                    0x1 => self.registers[x] |= self.registers[y],
                    0x2 => self.registers[x] &= self.registers[y],
                    0x3 => self.registers[x] ^= self.registers[y],
                    0x4 => {
                        let (total, overflow) = self.registers[x].overflowing_add(self.registers[y]);
                        self.registers[x] = total;
                        self.registers[0x0F] = if overflow {1} else {0};
                    },
                    0x5 => {
                        let vx = self.registers[x];
                        let vy = self.registers[y];
                        self.registers[0x0F] = if vy > vx {1} else {0};
                        self.registers[x] = vx.wrapping_sub(vy);
                    },
                    0x6 => {
                        self.registers[0xF] = self.registers[x] & 0x1;
                        self.registers[x] >>= 1;
                    },
                    0x7 => {
                        let vx = self.registers[x];
                        let vy = self.registers[y];
                        self.registers[0xF] = if vy > vy {1} else {0};
                        self.registers[x] = vy.wrapping_sub(vx);
                    },
                    0xE => {
                        self.registers[0xF] = self.registers[x] & 0x80;
                        self.registers[x] <<= 1;
                    },
                    _ => ()
                };
            },
            0x9 => {
                if self.registers[x] != self.registers[y] {
                    self.program_counter += 1;
                }
            },
            0xA => {
                self.addr_register = opcode & 0x0FFF;
            },
            0xB => {
                let address = opcode & 0x0FFF;
                self.program_counter = self.registers[0] as u16 + address;
            },
            0xC => {
                let value = opcode & 0x00FF;
                let mut rng = thread_rng();
                let rand_num = rng.gen_range(0, 255);
                self.registers[x] = (rand_num & value) as u8;
            },
            0xD => {
                let x_coord = self.registers[x];
                let y_coord = self.registers[y];
                let height = (opcode & 0x000F) as u8;

                let mut sprite_idx = self.addr_register;
                let mut pixel_flipped = false;

                for screen_y in y_coord..y_coord + height {
                    let sprite_row = self.memory[sprite_idx as usize];

                    for bit in 0..8 {
                        let sprite_pixel = if (sprite_row & 0x80) >> bit > 0 {
                            true
                        } else {
                            false
                        };

                        let screen_x = x_coord + bit;
                        let current_pixel_val = self.screen[screen_y as usize][screen_x as usize];
                        let new_pixel_val = sprite_pixel ^ current_pixel_val;

                        if new_pixel_val && current_pixel_val {
                            pixel_flipped = true;
                        }

                        self.screen[screen_y as usize][screen_x as usize] = new_pixel_val;
                    }
                }
            },
            0xE => {
                unimplemented!();
            },
            0xF => {
                unimplemented!();
            },
            _ => ()
        }
    }
}