use crate::cartridge::Cartridge;

struct SpecialReader {
    addr: u16,
    read: fn(u16) -> u8
}

struct SpecialWriter {
    addr: u16,
    write: fn(u16, u8)
}

pub struct MMU {
    memory: [u8;65536],
    special_readers: Vec<SpecialReader>,
    special_writers: Vec<SpecialWriter>,
    cartridge: Box<Cartridge>
}

pub fn new(cartridge: Box<Cartridge>) -> MMU {
    MMU {
        memory: [0;65536],
        special_readers: vec![],
        special_writers: vec![],
        cartridge
    }
}

impl MMU {
    pub fn register_special_reader(&mut self, addr: u16, read: fn(u16) -> u8) {
        self.special_readers.push(SpecialReader{addr, read})
    }

    pub fn register_special_writer(&mut self, addr: u16, write: fn(u16, u8)) {
        self.special_writers.push(SpecialWriter{addr, write})
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        for reader in self.special_readers.iter() {
            if addr == reader.addr {
                return (reader.read)(addr)
            }
        }
        match addr {
            0x0000 ... 0x7fff => self.cartridge.read_rom(addr),
            0xa000 ... 0xbfff => self.cartridge.read_ram(addr),
            _ => self.memory[addr as usize]
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        for writer in self.special_writers.iter() {
            if addr == writer.addr {
                return (writer.write)(addr, value)
            }
        }
        match addr {
            0x0000 ... 0x7fff => self.cartridge.write_rom(addr, value),
            0xa000 ... 0xbfff => self.cartridge.write_ram(addr, value),
            0xff02 | 0xff70 => (),
            0xff44 | 0xff4d => self.memory[addr as usize] = 0,
            0xff46 => { self.memory[addr as usize] = value; self.dma_transfer(value) }
            _ => self.memory[addr as usize] = value
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let addr = addr as usize;
        (self.memory[addr] | self.memory[addr+1]<<8) as u16
    }

    pub fn write_word(&mut self, addr: u16, value: u16) {
        self.write_byte(addr, (value&0xff) as u8);
        self.write_byte(addr, (value>>8) as u8);
    }

    fn dma_transfer(&mut self, value: u8) {
        let addr = (value as u16) << 8;
        for i in 0..0xa0 {
            self.write_byte(0xfe00+i, self.read_byte(addr+i));
        }
    }
}