include!("interrupts.rs");
include!("instructions/instructions.rs");
include!("instructions/extended_instructions.rs");

use crate::cartridge;
use crate::gpu;
use crate::mmu;
use crate::registers;

const CYCLES_PER_SECOND: i32 = 4194304 / 60;

pub struct GameBoy {
    regs: registers::Registers,
    interrupts_enabled: bool,
    enabling_interrupts: bool,
    is_halted: bool,
    mmu: mmu::MMU,
    gpu: gpu::GPU,
}

pub fn new(file_name: &str) -> GameBoy {
    let mmu = mmu::new(Box::new(cartridge::new(file_name)));
    let regs = registers::new();
    let gpu = gpu::new();
    GameBoy {
        regs,
        interrupts_enabled: false,
        enabling_interrupts: false,
        is_halted: false,
        mmu,
        gpu,
    }
}

impl GameBoy {
    pub fn step(&mut self) {
        let mut cycles_ellapsed = 0u8;
        for _ in (0..CYCLES_PER_SECOND).step_by(cycles_ellapsed as usize) {
            cycles_ellapsed = self.step_cpu();
            self.mmu.increment_counters(cycles_ellapsed as i32);
            self.gpu.step(cycles_ellapsed as i32);
            self.isr();
        }
    }

    pub fn step_cpu(&mut self) -> u8 {
        if self.is_halted {
            4
        } else {
            let op_code = self.fetch_byte();
            self.execute(op_code)
        }
    }

    pub fn call(&mut self, addr: u16) {
        self.regs.sp -= 2;
        self.mmu.write_word(self.regs.sp, self.regs.pc);
        self.regs.pc = addr;
    }

    fn execute(&mut self, op_code: u8) -> u8 {
        (INSTRUCTIONS[op_code as usize].1)(self)
    }

    fn fetch_byte(&mut self) -> u8 {
        let r = self.mmu.read_byte(self.regs.pc);
        self.regs.pc += 1;
        r
    }

    fn fetch_word(&mut self) -> u16 {
        let r = self.mmu.read_word(self.regs.pc);
        self.regs.pc += 2;
        r
    }
}
