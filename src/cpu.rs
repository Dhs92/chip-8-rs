use std::fs;
use std::io;
use std::io::prelude::*;
use std::fmt;

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

        match opcode & 0xF000 { // match against nibbles &'d with 0xF
            0x0000 => match opcode & 0x00FF {
                0x00E0 => { // CLS - Clear display
                    print!("Opcode: [{:>04X}] ", opcode);
                    Ok(())
                }
                0x00EE => { // RET
                    print!("Opcode: [{:>04X}] ", opcode);
                    Ok(())
                }
                _ => { // 0nnn - SYS addr
                    print!("Opcode: [{:>04X}] ", opcode);
                    Ok(())
                }
            },
            0x1000 => { // 1nnn - JP addr
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x2000 => { // 2nnn - CALL addr
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x3000 => { // 3xkk - SE Vx, byte
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x4000 => { // 4xkk - SNE Vx, byte
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x5000 => { // 5xy0 - SE Vx, Vy 
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x6000 => { // 6xkk - LD Vx, byte
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x7000 => { // 7xkk - ADD Vx, byte
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0x8000 => match opcode & 0x000F { // bitwise operations
                0x0000 => { // 8xy0 - LD Vx, Vy
                    Ok(())
                }
                0x0001 => { // OR Vx, Vy
                    print!("Opcode: [{:>04X}] ", opcode);
                    Ok(())
                }
                0x0002 => { // AND Vx, Vy
                    print!("Opcode: [{:>04X}] ", opcode);
                    Ok(())
                }
                0x0003 => { // XOR Vx, Vy
                    print!("Opcode: [{:>04X}] ", opcode);
                    Ok(())
                }
                0x0004 => { // ADD Vx, Vy
                    print!("Opcode: [{:>04X}] ", opcode);
                    Ok(())
                }
                0x0005 => { // SUB Vx, Vy
                    print!("Opcode: [{:>04X}] ", opcode);
                    Ok(())
                }
                0x0006 => { // SHR Vx {, Vy}
                    print!("Opcode: [{:>04X}] ", opcode);
                    Ok(())
                }
                0x0007 => { // SUBN Vx, Vy
                    print!("Opcode: [{:>04X}] ", opcode);
                    Ok(())
                }
                0x000E => { // SHL Vx, Vy
                    print!("Opcode: [{:>04X}] ", opcode);
                    Ok(())
                }
                _ => Ok(()),
            },
            0x9000 => { // 9xy0 - SNE Vx, Vy
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0xA000 => { // Annn - LD I, addr
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0xB000 => { // Bnnn - JP V0, addr
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0xC000 => { // Cxkk - RND Vx, byte
                print!("Opcode: [{:>04X}] ", opcode);
                Ok(())
            }
            0xD000 => { // Dxyn - DRW Vx, Vy, nibble 
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
}

pub struct Display {
    pub vram: [u8; 32 * 64],
}

pub struct Keypad {}
