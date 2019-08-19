use crate::drivers::Display;
use rand::prelude::*;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
pub(crate) const SCREEN_WIDTH: usize = 64;
pub(crate) const SCREEN_HEIGHT: usize = 32;

#[derive(Debug)]
pub enum Error {
    InvalidOpCode(u16),
    InvalidValue(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Keypad {}

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
    pub fn load_file(&mut self, filename: &std::path::Path) -> io::Result<usize> {
        let mut file = fs::File::open(filename)?;
        let mut buffer: [u8; 3584] = [0; 3584];
        let bytes_read = file.read(&mut buffer)?;

        for (&byte, mem) in buffer
            .iter()
            .zip(self.memory.iter_mut().skip(self.pc as usize))
            .take(bytes_read)
        {
            *mem = byte;
        }
        log::debug!("{:>04X?}", self.memory.to_vec());
        Ok(bytes_read)
    }

    fn get_opcode(&mut self) -> u16 {
        if self.pc < 4095 {
            let b1: u16 = u16::from(self.memory[self.pc as usize]);
            let b2: u16 = u16::from(self.memory[self.pc as usize + 1]);
            let opcode = b1 << 8 | b2;
            log::debug!("0x{:>04X}", opcode);
            opcode
        } else {
            log::debug!("No opcodes remaining");
            0
        }
    }

    // TODO implement OPCODEs
    pub fn execute_opcode(&mut self) -> Result<(), Error> {
        let opcode = self.get_opcode();
        let x = get_x(opcode);
        let y = get_y(opcode);
        let n = get_n(opcode);
        let kk = get_kk(opcode);
        let nnn = get_nnn(opcode);

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
                self.pc = nnn;
            }
            0x2000 => {
                // 2nnn - CALL addr
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = nnn;
            }
            0x3000 => {
                // 3xkk - SE Vx, byte
                if u16::from(self.v[x]) == kk {
                    self.increment_pc();
                }
            }
            0x4000 => {
                // 4xkk - SNE Vx, byte
                if u16::from(self.v[x]) != kk {
                    self.increment_pc();
                }
            }
            0x5000 => {
                // 5xy0 - SE Vx, Vy
                if u16::from(self.v[x]) == u16::from(self.v[y]) {
                    self.increment_pc();
                }
            }
            0x6000 => {
                // 6xkk - LD Vx, byte
                self.v[x] = kk as u8;
            }
            0x7000 => {
                // 7xkk - ADD Vx, byte
                let v_tmp = self.v[x].overflowing_add(kk as u8);

                self.v[x] = v_tmp.0;
            }
            0x8000 => match opcode & 0x000F {
                // bitwise operations
                0x0000 => {
                    // 8xy0 - LD Vx, Vy
                    self.v[x] = self.v[y];
                }
                0x0001 => {
                    // OR Vx, Vy
                    self.v[x] |= self.v[y];
                }
                0x0002 => {
                    // AND Vx, Vy
                    self.v[x] &= self.v[y];
                }
                0x0003 => {
                    // XOR Vx, Vy
                    self.v[x] ^= self.v[y];
                }
                0x0004 => {
                    // ADD Vx, Vy
                    let (add, overflow) = self.v[x].overflowing_add(self.v[y]);
                    if overflow {
                        self.v[15] = 1
                    } else {
                        self.v[15] = 0;
                    }
                    self.v[x] = add;
                }
                0x0005 => {
                    // SUB Vx, Vy
                    if self.v[x] > self.v[y] {
                        self.v[15] = 1;
                    } else {
                        self.v[15] = 0;
                    }

                    self.v[x] = self.v[y].overflowing_sub(self.v[x]).0;
                }
                0x0006 => {
                    // SHR Vx {, Vy}
                    if self.v[x] & 0b0000_0001 == 0b0000_0001 {
                        self.v[15] = 1;
                    } else {
                        self.v[15] = 0;
                    }

                    self.v[x] >>= 1;
                }
                0x0007 => {
                    // SUBN Vx, Vy
                    if self.v[y] > self.v[x] {
                        self.v[15] = 1;
                    } else {
                        self.v[15] = 0;
                    }

                    self.v[x] = self.v[x].overflowing_sub(self.v[y]).0;
                }
                0x000E => {
                    // SHL Vx {, Vy}
                    if self.v[x] & 0b1000_0000 == 0b1000_0000 {
                        self.v[15] = 1;
                    } else {
                        self.v[15] = 0;
                    }

                    self.v[x] <<= 1;
                }
                _ => (),
            },
            0x9000 => {
                // 9xy0 - SNE Vx, Vy
                if self.v[x] != self.v[y] {
                    self.increment_pc();
                    self.increment_pc();
                }
            }
            0xA000 => {
                // Annn - LD I, addr
                self.i = nnn;
            }
            0xB000 => {
                // Bnnn - JP V0, addr
                self.pc = u16::from(self.v[0]) + nnn;
            }
            0xC000 => {
                // Cxkk - RND Vx, byte
                self.v[x] = rand::random::<u8>() & kk as u8;
            }
            0xD000 => {
                // TODO: rewrite so that it actually works && follows the spec
                // Dxyn - DRW Vx, Vy, nibble
                //let mut y_local = y;
                //let mut x_local;

                for (byte, bools) in self
                    .memory
                    .iter()
                    .skip(self.i as usize)
                    .take(n as usize)
                    .map(|byte| byte.as_bools())
                    .enumerate()
                {
                    for (bit, b) in bools.iter().enumerate() {
                        if *b {
                            if self.display.set_pixel(x + bit, y + byte).unwrap() {
                                self.v[15] = 1;
                            } else {
                                self.v[15] = 0;
                            }
                        }
                    }
                }
            }
            0xE000 => match opcode & 0x00FF {
                0x009E => {
                    // Ex9E - SKP Vx
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x00A1 => {
                    // ExA1 - SKNP Vx
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                _ => (),
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => {
                    // Fx07 - LD Vx, DT
                    self.v[x] = self.dt;
                }
                0x000A => {
                    // Fx0A - LD Vx, K

                }
                0x0015 => {
                    // Fx15 - LD DT, Vx
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0018 => {
                    // Fx18 - LD ST, Vx
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x001E => {
                    // Fx1E - ADD I, Vx
                    self.i += x as u16;
                }
                0x0029 => {
                    // Fx29 - LD F, Vx
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0033 => {
                    // Fx33 - LD B, Vx
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0055 => {
                    // Fx55 - LD [I], Vx
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                0x0065 => {
                    // Fx65 - LD Vx, [I]
                    print!("Opcode: [{:>04X}] ", opcode);
                }
                _ => (),
            },
            _ => {
                println!("Invalid Opcode: [{:04X}] ", opcode);

                return Err(Error::InvalidOpCode(opcode));
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
    usize::from((opcode & 0x0F00) >> 8)
}

#[inline(always)]
fn get_y(opcode: u16) -> usize {
    usize::from((opcode & 0x00F0) >> 4)
}

#[inline(always)]
fn get_kk(opcode: u16) -> u16 {
    opcode & 0x00FF
}

#[inline(always)]
fn get_nnn(opcode: u16) -> u16 {
    opcode & 0x0FFF
}

#[inline(always)]
fn get_n(opcode: u16) -> u16 {
    opcode & 0x000F
}

// Thanks @Nick12
pub trait AsBoolSlice {
    fn is_set(self, mask: u8) -> bool;
    fn is_set_n(self, n: u8) -> bool;
    fn as_bools(self) -> [bool; 8];
}

impl AsBoolSlice for u8 {
    #[inline]
    fn is_set(self, mask: u8) -> bool {
        (self & mask) == mask
    }

    #[inline]
    fn is_set_n(self, bit_number: u8) -> bool {
        self.is_set(1u8 << bit_number)
    }

    #[inline]
    fn as_bools(self) -> [bool; 8] {
        [
            self.is_set_n(7),
            self.is_set_n(6),
            self.is_set_n(5),
            self.is_set_n(4),
            self.is_set_n(3),
            self.is_set_n(2),
            self.is_set_n(1),
            self.is_set_n(0),
        ]
    }
}
