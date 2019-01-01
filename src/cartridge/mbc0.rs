use crate::cartridge::Cartridge;

pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC0 {
    pub fn new(rom: Vec<u8>) -> MBC0 {
        MBC0 { rom }
    }
}

impl Cartridge for MBC0 {
    fn read_rom(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
    fn read_ram(&self, _addr: u16) -> u8 {
        0
    }
    fn write_rom(&mut self, _addr: u16, _value: u8) {}
    fn write_ram(&mut self, _addr: u16, _value: u8) {}
}
