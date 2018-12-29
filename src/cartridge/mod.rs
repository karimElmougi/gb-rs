mod mbc0;

use std::io::Read;
use std::fs::File;

pub trait Cartridge {
    fn read_rom(&self, addr: u16) -> u8;
    fn read_ram(&self, addr: u16) -> u8;
    fn write_rom(&mut self, addr: u16, value: u8);
    fn write_ram(&mut self, addr: u16, value: u8);
}

pub fn new(rom_name: &str) -> impl Cartridge {
    let mut rom = vec![];
    let _ = File::open(rom_name).expect("Could not read ROM file").read_to_end(&mut rom);

    match rom[0x147] {
        0x00 => mbc0::MBC0::new(rom),
        _ => mbc0::MBC0::new(vec![0; 10])
    }
}
