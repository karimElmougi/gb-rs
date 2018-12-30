use crate::cartridge::Cartridge;
use crate::interrupts::INTERRUPT_FLAG_ADDR;
use crate::interrupts::write_interrupt;
use crate::interrupts::InterruptFlag;

const DIVIDER_ADDR: u16 = 0xff04;
const COUNTER_ADDR: u16 = 0xff05;
const MODULO_ADDR: u16 = 0xff06;
const TIMER_CONTROL_ADDR: u16 = 0xff07;

pub struct MMU {
    pub interrupts_enabled: bool,
    pub enabling_interrupts: bool,
    pub is_halted: bool,
    memory: [u8;65536],
    cartridge: Box<Cartridge>,
    timer_counter: i32,
    divider_counter: i32,
}

pub fn new(cartridge: Box<Cartridge>) -> MMU {
    MMU {
        memory: [0;65536],
        cartridge,
        timer_counter: 1024,
        divider_counter: 0,
        interrupts_enabled: false,
        enabling_interrupts: false,
        is_halted: false,
    }
}

impl MMU {
    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000 ... 0x7fff => self.cartridge.read_rom(addr),
            0xa000 ... 0xbfff => self.cartridge.read_ram(addr),
            INTERRUPT_FLAG_ADDR => self.memory[addr as usize] | 0xe0,
            _ => self.memory[addr as usize]
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000 ... 0x7fff => self.cartridge.write_rom(addr, value),
            0xa000 ... 0xbfff => self.cartridge.write_ram(addr, value),
            0xff02 | 0xff70 => (),
            DIVIDER_ADDR => { self.memory[addr as usize] = 0; self.divider_counter = 0; self.timer_counter = self.get_timer_frequency() },
            COUNTER_ADDR => { self.memory[addr as usize] = value; self.timer_counter = self.get_timer_frequency() },
            TIMER_CONTROL_ADDR => {
                let old = self.memory[addr as usize];
                self.memory[addr as usize] = value;
                if old != value {
                    self.timer_counter = self.get_timer_frequency()
                }
            }
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

    pub fn increment_counters(&mut self, cycles: i32) {
        self.divider_counter += cycles;
        if self.divider_counter >= 255 {
            self.divider_counter -= 255;
            self.memory[DIVIDER_ADDR as usize] += 1
        }
        if self.is_timer_running() {
            self.timer_counter -= cycles;
            if self.timer_counter <= 0 {
                self.timer_counter += self.get_timer_frequency();
                self.increment_timer()
            }
        }
    }

    fn dma_transfer(&mut self, value: u8) {
        let addr = (value as u16) << 8;
        for i in 0..0xa0 {
            self.write_byte(0xfe00+i, self.read_byte(addr+i));
        }
    }

    fn is_timer_running(&self) -> bool {
        self.memory[TIMER_CONTROL_ADDR as usize]&0x04 == 0x04
    }

    fn get_timer_frequency(&self) -> i32 {
        match self.memory[TIMER_CONTROL_ADDR as usize] & 0x03 {
            0x0 => 1024,
            0x1 => 16,
            0x2 => 64,
            0x3 | _ => 256
        }
    }

    fn increment_timer(&mut self) {
        match self.memory[COUNTER_ADDR as usize] {
            255 => {
                let modulo = self.memory[MODULO_ADDR as usize];
                self.write_byte(COUNTER_ADDR, modulo);
                write_interrupt(self, InterruptFlag::TIMER);
            },
            value => self.write_byte(COUNTER_ADDR, value+1)
        }
    }
}