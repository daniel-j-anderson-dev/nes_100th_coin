use nes_100th_coin::{Emulator, test_rom::EXAMPLE_1};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut emulator = Emulator::new();
    emulator.reset("../test_roms/1_Example.nes")?;

    Ok(())
}
