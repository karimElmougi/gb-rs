use crate::cartridge::Cartridge;

pub struct MBC1 {
    rom: Vec<u8>,
}

pub fn new(rom: Vec<u8>) -> MBC1 {
    MBC1 { rom }
}

impl Cartridge for MBC1 {
    fn read_rom(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn read_ram(&self, _addr: u16) -> u8 {
        0
    }

    fn write_rom(&mut self, _addr: u16, _value: u8) {}

    fn write_ram(&mut self, _addr: u16, _value: u8) {}
}
