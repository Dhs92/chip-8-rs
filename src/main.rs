#![allow(dead_code)]
mod cpu;
use crate::cpu::*;
use std::io;
use std::fmt;
use std::fs;

#[derive(Debug)]
enum Error {
    InvalidOpCode(),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
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
    let mut opcode = String::new();

    println!("Reading to string...");
    io::stdin().read_line(&mut opcode)?;

    println!("Input: {}", opcode);

    let opcode: String = opcode.trim_end().to_string();

    println!("Post trim: {}", opcode);

    let decoded: Vec<u8> = hex::decode(opcode).expect("Decoding failed");
    println!("{:X?}", decoded);

    let opcode: u16 = (decoded[0] as u16) << 8 | (decoded[1] as u16);
    //assert_eq!(opcode, 0xA123);
    decode_opcode(opcode).unwrap();

    Ok(())
}

fn decode_opcode(opcode: u16) -> Result<(), Error> {
    match opcode & 0xF000 {
        0xA000 => {
            println!("Opcode: [{:>04X}]", opcode);
            Ok(())
        }
        0x0000 => match opcode & 0x00FF {
            0x00E0 => {
                println!("E1: [{:>4X}]", opcode);
                Ok(())
            }
            0x00EE => {
                println!("E2: [{:>4X}]", opcode);
                Ok(())
            }
            _ => {
                println!("E3: [{:>4X}]", opcode);
                Ok(())
            }
        },
        0x1000 => {
            println!("Opcode: [{:>04X}]", opcode);
            Ok(())
        }
        0x2000 => {
            println!("Opcode: [{:>04X}]", opcode);
            Ok(())
        }
        0x3000 => {
            println!("Opcode: [{:>04X}]", opcode);
            Ok(())
        }
        0x4000 => {
            println!("Opcode: [{:>04X}]", opcode);
            Ok(())
        }
        0x5000 => {
            println!("Opcode: [{:>04X}]", opcode);
            Ok(())
        }
        0x6000 => {
            println!("Opcode: [{:>04X}]", opcode);
            Ok(())
        }
        0x7000 => {
            println!("Opcode: [{:>04X}]", opcode);
            Ok(())
        }
        0x8000 => match opcode & 0x000F {
            0x0001 => {
                println!("Opcode: [{:>04X}]", opcode);
                Ok(())
            }
            0x0002 => {
                println!("Opcode: [{:>04X}]", opcode);
                Ok(())
            }
            0x0003 => {
                println!("Opcode: [{:>04X}]", opcode);
                Ok(())
            }
            0x0004 => {
                println!("Opcode: [{:>04X}]", opcode);
                Ok(())
            }
            0x0005 => {
                println!("Opcode: [{:>04X}]", opcode);
                Ok(())
            }
            0x0006 => {
                println!("Opcode: [{:>04X}]", opcode);
                Ok(())
            }
            0x0007 => {
                println!("Opcode: [{:>04X}]", opcode);
                Ok(())
            }
            0x000E => {
                println!("Opcode: [{:>04X}]", opcode);
                Ok(())
            }
            _ => Ok(()),
        },
        _ => {
            println!("Invalid Opcode: [{:04X}]", opcode);
            Err(Error::InvalidOpCode())
        }
    }
}
