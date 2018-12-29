use super::mmu::MMU;
use super::gpu::GPU;

pub struct CPU {
    interrupts_enabled: bool,
    enabling_interrupts: bool,
    is_halted: bool,
    a: u8, 
    b: u8, 
    c: u8, 
    d: u8, 
    e: u8, 
    h: u8, 
    l: u8, 
    f: u8,
    pc: u16, 
    sp: u16,
    mmu: MMU
}

pub fn new(mmu: MMU) -> CPU {
    CPU {
        a: 1,
        b: 0, 
        c: 19,
        d: 0,
        e: 216,
        h: 1,
        l: 77,
        f: 176,
        pc: 0x100,
        sp: 0xfffe,
        interrupts_enabled: false,
        enabling_interrupts: false,
        is_halted: false,
        mmu
    }
}

impl CPU {
    pub fn step(&self) -> u64 {
        if self.is_halted {
            4
        } else{
            let op_code = self.fetch_instruction();
            self.execute(op_code)
        }
    }

    fn fetch_instruction(&self) -> u8 {
        self.mmu.read_byte(self.pc)
    }

    fn execute(&self, op_code: u8) -> u64 {
        unimplemented!();
    }
}
