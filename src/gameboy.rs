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
        (INSTRUCTIONS[op_code as usize].f)(self)
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

const CARRY: u8 = 0x10;
const HALF_CARRY: u8 = 0x20;
const SUB: u8 = 0x40;
const ZERO: u8 = 0x80;

fn add(op1: u8, op2: u8, f: u8) -> (u8, u8) {
    add_impl(op1, op2, f, false)
}

fn addc(op1: u8, op2: u8, f: u8) -> (u8, u8) {
    add_impl(op1, op2, f, true)
}

fn add_impl(op1: u8, op2: u8, f: u8, carrying: bool) -> (u8, u8) {
    let carry = if carrying {
        ((f & CARRY) >> 4) as u16
    } else {
        0
    };
    let r16 = op1 as u16 + op2 as u16 + carry;
    let r = r16 as u8;
    let f = 0
        | if r == 0 { ZERO } else { 0 }
        | if r16 > 0xff { CARRY } else { 0 }
        | if (op1 & 0xf) + (op2 & 0xf) + (carry as u8) > 0xf {
            HALF_CARRY
        } else {
            0
        };
    (r, f)
}
