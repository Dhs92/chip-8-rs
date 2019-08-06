use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

#[derive(Debug)]
pub enum Error {
    InvalidOpCode(),
    InvalidValue(String),
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

        //self.memory[self.pc as usize..].copy_from_slice(&buffer[self.pc as usize..bytes_read]);
        for (i, byte) in buffer.iter().enumerate().take(bytes_read).skip(self.pc as usize) {
             self.memory[i] = *byte;
        }
        
        Ok(bytes_read)
    }

    fn get_opcode(&mut self) -> u16 {
        let b1: u16 = u16::from(self.memory[self.pc as usize]);
        let b2: u16 = u16::from(self.memory[self.pc as usize + 1]);

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
                }
                _ => (),
            },
            0x1000 => {
                // 1nnn - JP addr
                self.pc = get_nnn(opcode);
            }
            0x2000 => {
                // 2nnn - CALL addr
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = get_nnn(opcode);
            }
            0x3000 => {
                // 3xkk - SE Vx, byte
                if u16::from(self.v[get_x(opcode)]) == get_kk(opcode) {
                    self.increment_pc();
                }
            }
            0x4000 => {
                // 4xkk - SNE Vx, byte
                if u16::from(self.v[get_x(opcode)]) != get_kk(opcode) {
                    self.increment_pc();
                }
            }
            0x5000 => {
                // 5xy0 - SE Vx, Vy
                if u16::from(self.v[get_x(opcode)]) == u16::from(self.v[get_y(opcode)]) {
                    self.increment_pc();
                }
            }
            0x6000 => {
                // 6xkk - LD Vx, byte
                self.v[get_x(opcode)] = get_kk(opcode) as u8;
            }
            0x7000 => {
                // 7xkk - ADD Vx, byte
                self.v[get_x(opcode)] += get_kk(opcode) as u8;
            }
            0x8000 => match opcode & 0x000F {
                // bitwise operations
                0x0000 => {
                    // 8xy0 - LD Vx, Vy
                    self.v[get_x(opcode)] = self.v[get_y(opcode)];
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

                return Err(Error::InvalidOpCode())
            }
        }
        self.increment_pc();
        Ok(())
    }

    #[inline(always)]
    fn increment_pc(&mut self) {
        self.pc += 2;
    }
}

#[inline(always)]
fn compare(b1: u16, b2: u16) -> bool {
    b1 == b2
}

#[inline(always)]
fn get_x(opcode: u16) -> usize {
    usize::from(opcode & 0x0F00)
}

#[inline(always)]
fn get_y(opcode: u16) -> usize {
    usize::from(opcode & 0x00F0)
}

#[inline(always)]
fn get_kk(opcode: u16) -> u16 {
    opcode & 0x00FF
}

#[inline(always)]
fn get_nnn(opcode: u16) -> u16 {
    opcode & 0x0FFF
}

pub struct Display {
    pub vram: [u8; SCREEN_HEIGHT * SCREEN_WIDTH * 4],
}

impl Display {
    pub fn clear(&mut self) {
        self.vram = [0x00; SCREEN_HEIGHT * SCREEN_WIDTH * 4];
    }
    // fixme: support alpha channel
    pub unsafe fn set_pixel_greyscale(&mut self, pos: usize, val: u8) {
        // set to unsafe as it does not do any bounds checking
        self.vram
            .iter_mut()
            .skip(pos * 4)
            .take(3)
            .for_each(|pixel| *pixel = val);
        self.vram
            .iter_mut()
            .skip(pos * 4 + 3)
            .take(1)
            .for_each(|pixel| *pixel = 0xFF);
    }
    pub fn set_pixel(&mut self, x: usize, y: usize) -> Result<(), Error> {
        if x < 64 && y < 32 {
            unsafe { self.set_pixel_greyscale(x + (SCREEN_WIDTH * y), 0xFF) };
        } else if x >= 64 {
            return Err(Error::InvalidValue(
                format!("Value x is too large ({} is > 63)", x),
            ));
        } else if y >= 32 {
            return Err(Error::InvalidValue(
                format!("Value y is too large ({} is > 31)", y),
            ));
        } else {
            return Err(Error::InvalidValue(
                "An unknown error has occurred!".to_string(),
            ));
        }
        Ok(())
    }
}
pub struct Keypad {}
