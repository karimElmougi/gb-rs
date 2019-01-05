include!("instructions/instructions.rs");

use crate::mmu::MMU;
use crate::registers;

pub struct CPU {
    pub regs: registers::Registers,
    pub interrupts_enabled: bool,
    pub enabling_interrupts: bool,
    pub is_halted: bool,
}

pub fn new() -> CPU {
    let regs = registers::new();
    CPU {
        regs,
        interrupts_enabled: false,
        enabling_interrupts: false,
        is_halted: false,
    }
}

impl CPU {
    pub fn step(&mut self, mmu: &mut MMU) -> u8 {
        if self.is_halted {
            4
        } else {
            let op_code = self.fetch_byte(mmu);
            self.execute(mmu, op_code)
        }
    }

    pub fn call(&mut self, mmu: &mut MMU, addr: u16) {
        self.regs.sp -= 2;
        mmu.write_word(self.regs.sp, self.regs.pc);
        self.regs.pc = addr;
    }

    pub fn execute(&mut self, mmu: &mut MMU, op_code: u8) -> u8 {
        (INSTRUCTIONS[op_code as usize].1)(self, mmu)
    }

    pub fn fetch_byte(&mut self, mmu: &MMU) -> u8 {
        let r = mmu.read_byte(self.regs.pc);
        self.regs.pc += 1;
        r
    }

    pub fn fetch_word(&mut self, mmu: &MMU) -> u16 {
        let r = mmu.read_word(self.regs.pc);
        self.regs.pc += 2;
        r
    }
}
