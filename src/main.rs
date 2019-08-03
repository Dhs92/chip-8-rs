#![allow(dead_code)]
mod cpu;
use crate::cpu::*;
use std::io;

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
    Ok(())
}

fn get_opcode(b1: u8, b2: u8) -> u16 {
    (b1 as u16) << 8 | (b2 as u16)
}

