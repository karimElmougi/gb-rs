pub const INTERRUPT_FLAG_ADDR: u16 = 0xff0f;
pub const INTERRUPTS_ENABLED_ADDR: u16 = 0xffff;

#[derive(Copy, Clone)]
pub enum InterruptFlag {
    VBLANK = 0x01,
    LCD = 0x02,
    TIMER = 0x04,
    SERIAL = 0x08,
    JOYPAD = 0x10,
}

const INTERRUPT_FLAGS: [InterruptFlag; 5] = [
    InterruptFlag::VBLANK,
    InterruptFlag::LCD,
    InterruptFlag::TIMER,
    InterruptFlag::SERIAL,
    InterruptFlag::JOYPAD,
];

impl GameBoy {
    pub fn isr(&mut self) {
        if self.enabling_interrupts {
            self.interrupts_enabled = true;
            self.enabling_interrupts = false;
            return;
        }
        if !self.interrupts_enabled && !self.is_halted {
            return;
        }
        if !self.is_ready() {
            return;
        }
        for flag in INTERRUPT_FLAGS.iter() {
            if self.is_enabled(*flag) && self.has_occured(*flag) {
                self.service_interrupt(*flag);
                return;
            }
        }
    }

    fn service_interrupt(&mut self, interrupt_signal: InterruptFlag) {
        self.is_halted = false;
        if !self.interrupts_enabled && self.is_halted {
            return;
        }
        self.interrupts_enabled = false;
        let mut flags = self.mmu.read_byte(INTERRUPT_FLAG_ADDR);
        flags &= !(interrupt_signal as u8);
        self.mmu.write_byte(INTERRUPT_FLAG_ADDR, flags);
        let addr = match interrupt_signal {
            InterruptFlag::VBLANK => 0x40,
            InterruptFlag::LCD => 0x48,
            InterruptFlag::TIMER => 0x50,
            InterruptFlag::SERIAL => 0x58,
            InterruptFlag::JOYPAD => 0x60,
        };
        self.call(addr);
    }

    fn is_enabled(&self, interrupt_type: InterruptFlag) -> bool {
        let flag = interrupt_type as u8;
        self.mmu.read_byte(INTERRUPTS_ENABLED_ADDR) & flag == flag
    }

    fn has_occured(&self, interrupt_type: InterruptFlag) -> bool {
        let flag = interrupt_type as u8;
        self.mmu.read_byte(INTERRUPT_FLAG_ADDR) & flag == flag
    }

    fn is_ready(&self) -> bool {
        (self.mmu.read_byte(INTERRUPT_FLAG_ADDR) & self.mmu.read_byte(INTERRUPTS_ENABLED_ADDR))
            != 0x00
    }
}
