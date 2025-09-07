pub struct Cpu {
    program_counter: u16,
    A: u8,
    X: u8,
    Y: u8,
    ram: [u8; 0x800],
    rom: Vec<u8>,
}
impl Cpu {
    pub fn read(&self, address: u16) -> Option<u8> {
        self.ram.get(address as usize).copied()
    }
}

pub enum Instruction {
    LDA = 0xA9,
    LDX = 0xA2,
    LDY = 0xA0,
    HLT = 0x02,
}