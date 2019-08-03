#![allow(dead_code)]
mod cpu;
use crate::cpu::*;
use itertools::Itertools;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Error {
    InvalidOpCode(),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

struct Cartridge {
    pub rom: [u8; 3584],
    pub size: usize,
}

static mut CPU: Cpu = Cpu {
    pc: 0x200,
    i: 0,
    memory: [0; 4096],
    v: [0; 16],
    keypad: Keypad {},
    display: Display { vram: [0; 32 * 64] },
    stack: [0; 16],
    sp: 0,
    dt: 0,
};

fn main() -> io::Result<()> {
    let mut file = fs::File::open("15_Puzzle_[Roger Ivie].ch8")?;
    let mut memory: [u8; 3584] = [0; 3584];
    let bytes_read = file.read(&mut memory)?;

    for chunk in &memory.iter().chunks(2) {
        let mut opcode = 0;
        let mut b1: u16 = 0;
        let mut b2: u16 = 0;
        for (i, b) in chunk.enumerate() {
            if i == 1 {
                b1 = *b as u16
            } else {
                b2 = *b as u16
            }
        }
        opcode = b1 << 8 | b2;
        decode_opcode(opcode).unwrap()
    }

    //println!("{:4X?}", memory);

    Ok(())
}

fn get_opcode(b1: u8, b2: u8) -> u16 {
    (b1 as u16) << 8 | (b2 as u16)
}

fn decode_opcode(opcode: u16) -> Result<(), Error> {
    match opcode & 0xF000 {
        0x0000 => match opcode & 0x00FF {
            0x00E0 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x00EE => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            _ => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
        },
        0x1000 => {
            print!("Opcode: [{:>04X}] ", opcode);
            Ok(())
        }
        0x2000 => {
            print!("Opcode: [{:>04X}] ", opcode);
            Ok(())
        }
        0x3000 => {
            print!("Opcode: [{:>04X}] ", opcode);
            Ok(())
        }
        0x4000 => {
            print!("Opcode: [{:>04X}] ", opcode);
            Ok(())
        }
        0x5000 => {
            print!("Opcode: [{:>04X}] ", opcode);
            Ok(())
        }
        0x6000 => {
            print!("Opcode: [{:>04X}] ", opcode);
            Ok(())
        }
        0x7000 => {
            print!("Opcode: [{:>04X}] ", opcode);
            Ok(())
        }
        0x8000 => match opcode & 0x000F {
            0x0001 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x0002 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x0003 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x0004 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x0005 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x0006 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x0007 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x000E => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            _ => Ok(()),
        },
        0x9000 => {
            print!("Opcode: [{:>04X}] ", opcode);
            Ok(())
        }
        0xA000 => {
            print!("Opcode: [{:>04X}] ", opcode);
            Ok(())
        }
        0xB000 => {
            print!("Opcode: [{:>04X}] ", opcode);
            Ok(())
        }
        0xC000 => {
            print!("Opcode: [{:>04X}] ", opcode);
            Ok(())
        }
        0xD000 => {
            print!("Opcode: [{:>04X}] ", opcode);
            Ok(())
        }
        0xE000 => match opcode & 0x00FF {
            0x009E => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x00A1 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            _ => Ok(()),
        },
        0xF000 => match opcode & 0x00FF {
            0x0007 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x000A => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x0015 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x0018 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x001E => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x0029 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x0033 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x0055 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x0065 => {
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            _ => Ok(()),
        },
        _ => {
            print!("Invalid Opcode: [{:04X}] ", opcode);
            Ok(())
            //Err(Error::InvalidOpCode())
        }
    }
}
