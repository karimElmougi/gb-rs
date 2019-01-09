use crate::cartridge::Cartridge;

pub struct MBC3 {
    rom: Vec<u8>,
    rom_bank: usize,
    ram: [u8; 0x8000],
    ram_bank: usize,
    ram_enabled: bool,
    ram_changed: bool,
}

pub fn new(rom: Vec<u8>) -> MBC3 {
    MBC3 {
        rom,
        rom_bank: 1,
        ram: [0; 0x8000],
        ram_bank: 0,
        ram_enabled: false,
        ram_changed: false,
    }
}

impl MBC3 {
    fn enable_ram(&mut self, value: usize) {
        let test = value & 0xf;
        if test == 0xa {
            self.ram_enabled = true;
        } else {
            self.ram_enabled = false;
        }
    }
}

impl Cartridge for MBC3 {
    fn read_rom(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        if addr < 0x4000 {
            self.rom[addr]
        } else {
            let new_addr = self.rom_bank * 0x4000 + addr - 0x4000;
            self.rom[new_addr]
        }
    }

    fn read_ram(&self, addr: u16) -> u8 {
        let new_addr = self.ram_bank * 0x2000 + addr as usize - 0xa000;
        self.ram[new_addr]
    }

    fn write_rom(&mut self, addr: u16, value: u8) {
        let value = value as usize;
        match addr {
            0...0x1fff => self.enable_ram(value),
            0x2000...0x3fff => {
                self.rom_bank = (value & 0x7f) as usize;
                if self.rom_bank == 0 {
                    self.rom_bank = 1;
                }
            },
            0x4000...0x5fff => self.ram_bank = value as usize & 0x03,
            _ => {}
        };
    }

    fn write_ram(&mut self, addr: u16, value: u8) {
        if self.ram_enabled {
            self.ram_changed = true;
            let new_addr = (addr - 0xa000) as usize + self.ram_bank * 0x2000;
            self.ram[new_addr] = value;
        }
    }
}
