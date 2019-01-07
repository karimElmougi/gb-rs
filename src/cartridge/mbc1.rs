use crate::cartridge::Cartridge;

pub struct MBC1 {
    rom: Vec<u8>,
    rom_bank: usize,
    rom_banking_enabled: bool,
    ram: [u8; 0x8000],
    ram_bank: usize,
    ram_enabled: bool,
    ram_changed: bool,
}

pub fn new(rom: Vec<u8>) -> MBC1 {
    MBC1 {
        rom,
        rom_bank: 1,
        rom_banking_enabled: false,
        ram: [0; 0x8000],
        ram_bank: 0,
        ram_enabled: false,
        ram_changed: false,
    }
}

impl MBC1 {
    fn enable_ram(&mut self, value: usize) {
        let test = value & 0xf;
        if test == 0xa {
            self.ram_enabled = true;
        } else {
            self.ram_enabled = false;
        }
    }

    fn change_lower_rom_bank(&mut self, value: usize) {
        self.rom_bank &= 0xe0;
        self.rom_bank |= (value & 0x1f) as usize;
        if self.rom_bank == 0 {
            self.rom_bank = 1;
        }
    }

    fn change_upper_rom_bank(&mut self, value: usize) {
        self.rom_bank &= 0x1f;
        self.rom_bank |= (value & 0xe0) as usize & 224;
        if self.rom_bank == 0 {
            self.rom_bank = 1;
        }
    }
}

impl Cartridge for MBC1 {
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
            0x2000...0x3fff => self.change_lower_rom_bank(value),
            0x4000...0x5fff => {
                if self.rom_banking_enabled {
                    self.change_upper_rom_bank(value);
                } else {
                    self.ram_bank = value & 0x03;
                }
            },
            0x6000...0x7fff => {
                if value & 0x01 == 0 {
                    self.rom_banking_enabled = true;
                    self.ram_bank = 0;
                } else {
                    self.rom_banking_enabled = false;
                }
            },
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
