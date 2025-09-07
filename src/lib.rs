mod error;
mod test_rom;

use std::{fs, io};

pub struct Cpu {
    program_counter: u16,
    A: u8,
    X: u8,
    Y: u8,
    ram: [u8; 0x800],
    rom: Vec<u8>,
    header: Vec<u8>,
}
// Constructors
impl Cpu {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            A: 0,
            X: 0,
            Y: 0,
            ram: [0; _],
            rom: Vec::new(),
            header: Vec::new(),
        }
    }
}

// accessors
impl Cpu {
    pub fn read(&self, address: u16) -> Option<u8> {
        let address = address as usize;
        if address < 0x800 {
            self.ram.get(address)
        } else if address >= 0x8000 {
            self.rom.get(address - self.rom.len())
        } else {
            None
        }
        .copied()
    }
}

// mutators
impl Cpu {
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        if let Some(header) = rom.get(..0x10) {
            self.header.copy_from_slice(header)
        };
        if let Some(rom) = rom.get(0x10..) {
            self.rom.copy_from_slice(rom)
        };
    }

    pub fn increment_program_counter(&mut self) {
        let pcl = self.read(0xFFFC).unwrap_or_default() as u16;
        let pch = self.read(0xFFFD).unwrap_or_default() as u16;
        self.program_counter = (pch * 0x100) + pcl;
    }

    pub fn reset(&mut self, file_path: &str) -> Result<(), io::Error> {
        self.load_rom(fs::read(file_path)?);

        self.increment_program_counter();

        Ok(())
    }
}

pub enum Instruction {
    LDA = 0xA9,
    LDX = 0xA2,
    LDY = 0xA0,
    HLT = 0x02,
}
