use crate::cpu::CPU;
use crate::mmu::MMU;

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

pub fn isr(cpu: &mut CPU, mmu: &mut MMU) {
    if cpu.enabling_interrupts {
        cpu.interrupts_enabled = true;
        cpu.enabling_interrupts = false;
        return;
    }
    if !cpu.interrupts_enabled && !cpu.is_halted {
        return;
    }
    if !is_ready(mmu) {
        return;
    }
    for flag in INTERRUPT_FLAGS.iter() {
        if is_enabled(mmu, *flag) && has_occured(mmu, *flag) {
            service_interrupt(cpu, mmu, *flag);
            return;
        }
    }
}

pub fn write_interrupt(mmu: &mut MMU, interrupt_signal: InterruptFlag) {
    let flags = mmu.read_byte(INTERRUPT_FLAG_ADDR);
    mmu.write_byte(INTERRUPT_FLAG_ADDR, flags | (interrupt_signal as u8));
}

fn service_interrupt(cpu: &mut CPU, mmu: &mut MMU, interrupt_signal: InterruptFlag) {
    cpu.is_halted = false;
    if !cpu.interrupts_enabled && cpu.is_halted {
        return;
    }
    cpu.interrupts_enabled = false;
    let flags = mmu.read_byte(INTERRUPT_FLAG_ADDR) & !(interrupt_signal as u8);
    mmu.write_byte(INTERRUPT_FLAG_ADDR, flags);
    let addr = match interrupt_signal {
        InterruptFlag::VBLANK => 0x40,
        InterruptFlag::LCD => 0x48,
        InterruptFlag::TIMER => 0x50,
        InterruptFlag::SERIAL => 0x58,
        InterruptFlag::JOYPAD => 0x60,
    };
    cpu.call(mmu, addr);
}

fn is_enabled(mmu: &MMU, interrupt_type: InterruptFlag) -> bool {
    let flag = interrupt_type as u8;
    mmu.read_byte(INTERRUPTS_ENABLED_ADDR) & flag == flag
}

fn has_occured(mmu: &MMU, interrupt_type: InterruptFlag) -> bool {
    let flag = interrupt_type as u8;
    mmu.read_byte(INTERRUPT_FLAG_ADDR) & flag == flag
}

fn is_ready(mmu: &MMU) -> bool {
    (mmu.read_byte(INTERRUPT_FLAG_ADDR) & mmu.read_byte(INTERRUPTS_ENABLED_ADDR)) != 0x00
}
