mod mbc0;
mod mbc1;
mod mbc3;

use std::fs::File;
use std::io::Read;

pub trait Cartridge {
    fn read_rom(&self, addr: u16) -> u8;
    fn read_ram(&self, addr: u16) -> u8;
    fn write_rom(&mut self, addr: u16, value: u8);
    fn write_ram(&mut self, addr: u16, value: u8);
}

pub fn new(rom_name: &str) -> Box<Cartridge> {
    let mut rom = vec![];
    let _ = File::open(rom_name)
        .expect("Could not read ROM file")
        .read_to_end(&mut rom);

    match rom[0x147] {
        0x00 => Box::new(mbc0::new(rom)),
        0x01...0x03 => Box::new(mbc1::new(rom)),
        0x08...0x0d => Box::new(mbc0::new(rom)),
        0x0f...0x13 => Box::new(mbc3::new(rom)),
        _ => panic!("Unsupported cartridge type"),
    }
}
