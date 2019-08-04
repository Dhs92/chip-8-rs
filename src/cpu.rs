use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub enum Error {
    InvalidOpCode(),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Cpu {
    // index register
    pub i: u16,
    // program counter
    pub pc: u16,
    // memory
    pub memory: [u8; 4096],
    // registers
    pub v: [u8; 16],
    // peripherals
    pub keypad: Keypad,
    pub display: Display,
    // stack
    pub stack: [u16; 16],
    // stack pointer
    pub sp: u8,
    // delay timer
    pub dt: u8,
}

impl Cpu {
    pub fn load_file(&mut self, filename: &str) -> io::Result<usize> {
        let mut file = fs::File::open(filename)?;
        let mut buffer: [u8; 3584] = [0; 3584];
        let bytes_read = file.read(&mut buffer)?;

        // for (i, byte) in buffer.iter().enumerate().take(bytes_read).skip(self.pc as usize) {
        //     self.memory[i] = *byte;
        // }

        self.memory[self.pc as usize..].copy_from_slice(&buffer[self.pc as usize..bytes_read]);

        Ok(bytes_read)
    }

    fn get_opcode(&mut self) -> u16 {
        let b1: u16 = self.memory[self.pc as usize] as u16;
        let b2: u16 = self.memory[self.pc as usize + 1] as u16;

        b1 << 8 | b2
    }

    // TODO implement OPCODEs
    fn execute_opcode(&mut self) -> Result<(), Error> {
        let opcode = self.get_opcode();

        match opcode & 0xF000 {
            // match against nibbles &'d with 0xF
            0x0000 => match opcode & 0x00FF {
                0x00E0 => {
                    // CLS - Clear display
                    self.display.clear();
                }
                0x00EE => {
                    // RET
                    self.pc = self.stack[self.sp as usize];
                    self.sp -= 1;
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                _ => {
                    // 0nnn - SYS addr, no longer used
                    print!("Opcode: [{:>04X}] ", opcode);
                }
            },
            0x1000 => {
                // 1nnn - JP addr
                let nnn = opcode & 0x0FFF;
                self.pc = nnn;
            }
            0x2000 => {
                // 2nnn - CALL addr
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = opcode & 0x0FFF;
            }
            0x3000 => {
                // 3xkk - SE Vx, byte
                if self.v[(opcode & 0x0F00) as usize] as u16 == opcode & 0x00FF {
                    self.pc += 2;
                }
            }
            0x4000 => {
                // 4xkk - SNE Vx, byte
                if self.v[(opcode & 0x0F00) as usize] as u16 != opcode & 0x00FF {
                    self.pc += 2;
                }
            }
            0x5000 => {
                // 5xy0 - SE Vx, Vy
                if self.v[(opcode & 0x0F00) as usize] == self.v[(opcode & 0x00F0) as usize] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                // 6xkk - LD Vx, byte
                print!("Opcode: [{:>04X}] ", opcode);
            }
            0x7000 => {
                // 7xkk - ADD Vx, byte
                print!("Opcode: [{:>04X}] ", opcode);
            }
            0x8000 => match opcode & 0x000F {
                // bitwise operations
                0x0000 => { // 8xy0 - LD Vx, Vy
                    self.v[(opcode & 0x0F00) as usize] = self.v[(opcode & 0x00F0) as usize];
                }
                0x0001 => {
                    // OR Vx, Vy
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0002 => {
                    // AND Vx, Vy
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0003 => {
                    // XOR Vx, Vy
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0004 => {
                    // ADD Vx, Vy
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0005 => {
                    // SUB Vx, Vy
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0006 => {
                    // SHR Vx {, Vy}
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0007 => {
                    // SUBN Vx, Vy
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x000E => {
                    // SHL Vx, Vy
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                _ => (),
            },
            0x9000 => {
                // 9xy0 - SNE Vx, Vy
                print!("Opcode: [{:>04X}] ", opcode);
            }
            0xA000 => {
                // Annn - LD I, addr
                print!("Opcode: [{:>04X}] ", opcode);
            }
            0xB000 => {
                // Bnnn - JP V0, addr
                print!("Opcode: [{:>04X}] ", opcode);
            }
            0xC000 => {
                // Cxkk - RND Vx, byte
                print!("Opcode: [{:>04X}] ", opcode);
            }
            0xD000 => {
                // Dxyn - DRW Vx, Vy, nibble
                print!("Opcode: [{:>04X}] ", opcode);
            }
            0xE000 => match opcode & 0x00FF {
                0x009E => {
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x00A1 => {
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                _ => (),
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => {
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x000A => {
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0015 => {
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0018 => {
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x001E => {
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0029 => {
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0033 => {
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0055 => {
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0065 => {
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                _ => (),
            },
            _ => {
                print!("Invalid Opcode: [{:04X}] ", opcode);

                //Err(Error::InvalidOpCode())
            }
        }
        self.pc += 2;
        Ok(())
    }
}

pub struct Display {
    pub vram: [bool; SCREEN_HEIGHT * SCREEN_WIDTH],
}

impl Display {
    pub fn clear(&mut self) {
        self.vram = [false; SCREEN_HEIGHT * SCREEN_WIDTH];
    }
}
pub struct Keypad {}
