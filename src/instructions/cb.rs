use super::flags::CARRY;
use super::flags::HALF_CARRY;
use super::flags::ZERO;
use crate::cpu::CPU;
use crate::mmu::MMU;

pub fn execute_cb(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let op_code = cpu.fetch_byte(mmu);
    match op_code {
        0x00 => rlc(&mut cpu.regs.b, &mut cpu.regs.f),
        0x01 => rlc(&mut cpu.regs.c, &mut cpu.regs.f),
        0x02 => rlc(&mut cpu.regs.d, &mut cpu.regs.f),
        0x03 => rlc(&mut cpu.regs.e, &mut cpu.regs.f),
        0x04 => rlc(&mut cpu.regs.h, &mut cpu.regs.f),
        0x05 => rlc(&mut cpu.regs.l, &mut cpu.regs.f),
        0x06 => func_hl(cpu, mmu, rlc),
        0x07 => rlc(&mut cpu.regs.a, &mut cpu.regs.f),
        0x08 => rrc(&mut cpu.regs.b, &mut cpu.regs.f),
        0x09 => rrc(&mut cpu.regs.c, &mut cpu.regs.f),
        0x0a => rrc(&mut cpu.regs.d, &mut cpu.regs.f),
        0x0b => rrc(&mut cpu.regs.e, &mut cpu.regs.f),
        0x0c => rrc(&mut cpu.regs.h, &mut cpu.regs.f),
        0x0d => rrc(&mut cpu.regs.l, &mut cpu.regs.f),
        0x0e => func_hl(cpu, mmu, rrc),
        0x0f => rrc(&mut cpu.regs.a, &mut cpu.regs.f),
        0x10 => rl(&mut cpu.regs.b, &mut cpu.regs.f),
        0x11 => rl(&mut cpu.regs.c, &mut cpu.regs.f),
        0x12 => rl(&mut cpu.regs.d, &mut cpu.regs.f),
        0x13 => rl(&mut cpu.regs.e, &mut cpu.regs.f),
        0x14 => rl(&mut cpu.regs.h, &mut cpu.regs.f),
        0x15 => rl(&mut cpu.regs.l, &mut cpu.regs.f),
        0x16 => func_hl(cpu, mmu, rl),
        0x17 => rl(&mut cpu.regs.a, &mut cpu.regs.f),
        0x18 => rr(&mut cpu.regs.b, &mut cpu.regs.f),
        0x19 => rr(&mut cpu.regs.c, &mut cpu.regs.f),
        0x1a => rr(&mut cpu.regs.d, &mut cpu.regs.f),
        0x1b => rr(&mut cpu.regs.e, &mut cpu.regs.f),
        0x1c => rr(&mut cpu.regs.h, &mut cpu.regs.f),
        0x1d => rr(&mut cpu.regs.l, &mut cpu.regs.f),
        0x1e => func_hl(cpu, mmu, rr),
        0x1f => rr(&mut cpu.regs.a, &mut cpu.regs.f),
        0x20 => sla(&mut cpu.regs.b, &mut cpu.regs.f),
        0x21 => sla(&mut cpu.regs.c, &mut cpu.regs.f),
        0x22 => sla(&mut cpu.regs.d, &mut cpu.regs.f),
        0x23 => sla(&mut cpu.regs.e, &mut cpu.regs.f),
        0x24 => sla(&mut cpu.regs.h, &mut cpu.regs.f),
        0x25 => sla(&mut cpu.regs.l, &mut cpu.regs.f),
        0x26 => func_hl(cpu, mmu, sla),
        0x27 => sla(&mut cpu.regs.a, &mut cpu.regs.f),
        0x28 => sra(&mut cpu.regs.b, &mut cpu.regs.f),
        0x29 => sra(&mut cpu.regs.c, &mut cpu.regs.f),
        0x2a => sra(&mut cpu.regs.d, &mut cpu.regs.f),
        0x2b => sra(&mut cpu.regs.e, &mut cpu.regs.f),
        0x2c => sra(&mut cpu.regs.h, &mut cpu.regs.f),
        0x2d => sra(&mut cpu.regs.l, &mut cpu.regs.f),
        0x2e => func_hl(cpu, mmu, sra),
        0x2f => sra(&mut cpu.regs.a, &mut cpu.regs.f),
        0x30 => swap(&mut cpu.regs.b, &mut cpu.regs.f),
        0x31 => swap(&mut cpu.regs.c, &mut cpu.regs.f),
        0x32 => swap(&mut cpu.regs.d, &mut cpu.regs.f),
        0x33 => swap(&mut cpu.regs.e, &mut cpu.regs.f),
        0x34 => swap(&mut cpu.regs.h, &mut cpu.regs.f),
        0x35 => swap(&mut cpu.regs.l, &mut cpu.regs.f),
        0x36 => func_hl(cpu, mmu, swap),
        0x37 => swap(&mut cpu.regs.a, &mut cpu.regs.f),
        0x38 => srl(&mut cpu.regs.b, &mut cpu.regs.f),
        0x39 => srl(&mut cpu.regs.c, &mut cpu.regs.f),
        0x3a => srl(&mut cpu.regs.d, &mut cpu.regs.f),
        0x3b => srl(&mut cpu.regs.e, &mut cpu.regs.f),
        0x3c => srl(&mut cpu.regs.h, &mut cpu.regs.f),
        0x3d => srl(&mut cpu.regs.l, &mut cpu.regs.f),
        0x3e => func_hl(cpu, mmu, srl),
        0x3f => srl(&mut cpu.regs.a, &mut cpu.regs.f),
        0x40 => bit(&mut cpu.regs.b, &mut cpu.regs.f, 0),
        0x41 => bit(&mut cpu.regs.c, &mut cpu.regs.f, 0),
        0x42 => bit(&mut cpu.regs.d, &mut cpu.regs.f, 0),
        0x43 => bit(&mut cpu.regs.e, &mut cpu.regs.f, 0),
        0x44 => bit(&mut cpu.regs.h, &mut cpu.regs.f, 0),
        0x45 => bit(&mut cpu.regs.l, &mut cpu.regs.f, 0),
        0x46 => bit_hl(cpu, mmu, 0),
        0x47 => bit(&mut cpu.regs.a, &mut cpu.regs.f, 1),
        0x48 => bit(&mut cpu.regs.b, &mut cpu.regs.f, 1),
        0x49 => bit(&mut cpu.regs.c, &mut cpu.regs.f, 1),
        0x4a => bit(&mut cpu.regs.d, &mut cpu.regs.f, 1),
        0x4b => bit(&mut cpu.regs.e, &mut cpu.regs.f, 1),
        0x4c => bit(&mut cpu.regs.h, &mut cpu.regs.f, 1),
        0x4d => bit(&mut cpu.regs.l, &mut cpu.regs.f, 1),
        0x4e => bit_hl(cpu, mmu, 1),
        0x4f => bit(&mut cpu.regs.a, &mut cpu.regs.f, 1),
        0x50 => bit(&mut cpu.regs.b, &mut cpu.regs.f, 2),
        0x51 => bit(&mut cpu.regs.c, &mut cpu.regs.f, 2),
        0x52 => bit(&mut cpu.regs.d, &mut cpu.regs.f, 2),
        0x53 => bit(&mut cpu.regs.e, &mut cpu.regs.f, 2),
        0x54 => bit(&mut cpu.regs.h, &mut cpu.regs.f, 2),
        0x55 => bit(&mut cpu.regs.l, &mut cpu.regs.f, 2),
        0x56 => bit_hl(cpu, mmu, 2),
        0x57 => bit(&mut cpu.regs.a, &mut cpu.regs.f, 2),
        0x58 => bit(&mut cpu.regs.b, &mut cpu.regs.f, 3),
        0x59 => bit(&mut cpu.regs.c, &mut cpu.regs.f, 3),
        0x5a => bit(&mut cpu.regs.d, &mut cpu.regs.f, 3),
        0x5b => bit(&mut cpu.regs.e, &mut cpu.regs.f, 3),
        0x5c => bit(&mut cpu.regs.h, &mut cpu.regs.f, 3),
        0x5d => bit(&mut cpu.regs.l, &mut cpu.regs.f, 3),
        0x5e => bit_hl(cpu, mmu, 3),
        0x5f => bit(&mut cpu.regs.a, &mut cpu.regs.f, 3),
        0x60 => bit(&mut cpu.regs.b, &mut cpu.regs.f, 4),
        0x61 => bit(&mut cpu.regs.c, &mut cpu.regs.f, 4),
        0x62 => bit(&mut cpu.regs.d, &mut cpu.regs.f, 4),
        0x63 => bit(&mut cpu.regs.e, &mut cpu.regs.f, 4),
        0x64 => bit(&mut cpu.regs.h, &mut cpu.regs.f, 4),
        0x65 => bit(&mut cpu.regs.l, &mut cpu.regs.f, 4),
        0x66 => bit_hl(cpu, mmu, 4),
        0x67 => bit(&mut cpu.regs.a, &mut cpu.regs.f, 4),
        0x68 => bit(&mut cpu.regs.b, &mut cpu.regs.f, 5),
        0x69 => bit(&mut cpu.regs.c, &mut cpu.regs.f, 5),
        0x6a => bit(&mut cpu.regs.d, &mut cpu.regs.f, 5),
        0x6b => bit(&mut cpu.regs.e, &mut cpu.regs.f, 5),
        0x6c => bit(&mut cpu.regs.h, &mut cpu.regs.f, 5),
        0x6d => bit(&mut cpu.regs.l, &mut cpu.regs.f, 5),
        0x6e => bit_hl(cpu, mmu, 5),
        0x6f => bit(&mut cpu.regs.a, &mut cpu.regs.f, 5),
        0x70 => bit(&mut cpu.regs.b, &mut cpu.regs.f, 6),
        0x71 => bit(&mut cpu.regs.c, &mut cpu.regs.f, 6),
        0x72 => bit(&mut cpu.regs.d, &mut cpu.regs.f, 6),
        0x73 => bit(&mut cpu.regs.e, &mut cpu.regs.f, 6),
        0x74 => bit(&mut cpu.regs.h, &mut cpu.regs.f, 6),
        0x75 => bit(&mut cpu.regs.l, &mut cpu.regs.f, 6),
        0x76 => bit_hl(cpu, mmu, 6),
        0x77 => bit(&mut cpu.regs.a, &mut cpu.regs.f, 6),
        0x78 => bit(&mut cpu.regs.b, &mut cpu.regs.f, 7),
        0x79 => bit(&mut cpu.regs.c, &mut cpu.regs.f, 7),
        0x7a => bit(&mut cpu.regs.d, &mut cpu.regs.f, 7),
        0x7b => bit(&mut cpu.regs.e, &mut cpu.regs.f, 7),
        0x7c => bit(&mut cpu.regs.h, &mut cpu.regs.f, 7),
        0x7d => bit(&mut cpu.regs.l, &mut cpu.regs.f, 7),
        0x7e => bit_hl(cpu, mmu, 7),
        0x7f => bit(&mut cpu.regs.a, &mut cpu.regs.f, 7),
        0x80 => res(&mut cpu.regs.b, 0),
        0x81 => res(&mut cpu.regs.c, 0),
        0x82 => res(&mut cpu.regs.d, 0),
        0x83 => res(&mut cpu.regs.e, 0),
        0x84 => res(&mut cpu.regs.h, 0),
        0x85 => res(&mut cpu.regs.l, 0),
        0x86 => bitfunc_hl(cpu, mmu, 0, res),
        0x87 => res(&mut cpu.regs.a, 1),
        0x88 => res(&mut cpu.regs.b, 1),
        0x89 => res(&mut cpu.regs.c, 1),
        0x8a => res(&mut cpu.regs.d, 1),
        0x8b => res(&mut cpu.regs.e, 1),
        0x8c => res(&mut cpu.regs.h, 1),
        0x8d => res(&mut cpu.regs.l, 1),
        0x8e => bitfunc_hl(cpu, mmu, 1, res),
        0x8f => res(&mut cpu.regs.a, 1),
        0x90 => res(&mut cpu.regs.b, 2),
        0x91 => res(&mut cpu.regs.c, 2),
        0x92 => res(&mut cpu.regs.d, 2),
        0x93 => res(&mut cpu.regs.e, 2),
        0x94 => res(&mut cpu.regs.h, 2),
        0x95 => res(&mut cpu.regs.l, 2),
        0x96 => bitfunc_hl(cpu, mmu, 2, res),
        0x97 => res(&mut cpu.regs.a, 2),
        0x98 => res(&mut cpu.regs.b, 3),
        0x99 => res(&mut cpu.regs.c, 3),
        0x9a => res(&mut cpu.regs.d, 3),
        0x9b => res(&mut cpu.regs.e, 3),
        0x9c => res(&mut cpu.regs.h, 3),
        0x9d => res(&mut cpu.regs.l, 3),
        0x9e => bitfunc_hl(cpu, mmu, 3, res),
        0x9f => res(&mut cpu.regs.a, 3),
        0xa0 => res(&mut cpu.regs.b, 4),
        0xa1 => res(&mut cpu.regs.c, 4),
        0xa2 => res(&mut cpu.regs.d, 4),
        0xa3 => res(&mut cpu.regs.e, 4),
        0xa4 => res(&mut cpu.regs.h, 4),
        0xa5 => res(&mut cpu.regs.l, 4),
        0xa6 => bitfunc_hl(cpu, mmu, 4, res),
        0xa7 => res(&mut cpu.regs.a, 4),
        0xa8 => res(&mut cpu.regs.b, 5),
        0xa9 => res(&mut cpu.regs.c, 5),
        0xaa => res(&mut cpu.regs.d, 5),
        0xab => res(&mut cpu.regs.e, 5),
        0xac => res(&mut cpu.regs.h, 5),
        0xad => res(&mut cpu.regs.l, 5),
        0xae => bitfunc_hl(cpu, mmu, 5, res),
        0xaf => res(&mut cpu.regs.a, 5),
        0xb0 => res(&mut cpu.regs.b, 6),
        0xb1 => res(&mut cpu.regs.c, 6),
        0xb2 => res(&mut cpu.regs.d, 6),
        0xb3 => res(&mut cpu.regs.e, 6),
        0xb4 => res(&mut cpu.regs.h, 6),
        0xb5 => res(&mut cpu.regs.l, 6),
        0xb6 => bitfunc_hl(cpu, mmu, 6, res),
        0xb7 => res(&mut cpu.regs.a, 6),
        0xb8 => res(&mut cpu.regs.b, 7),
        0xb9 => res(&mut cpu.regs.c, 7),
        0xba => res(&mut cpu.regs.d, 7),
        0xbb => res(&mut cpu.regs.e, 7),
        0xbc => res(&mut cpu.regs.h, 7),
        0xbd => res(&mut cpu.regs.l, 7),
        0xbe => bitfunc_hl(cpu, mmu, 7, res),
        0xbf => res(&mut cpu.regs.a, 7),
        0xc0 => set(&mut cpu.regs.b, 0),
        0xc1 => set(&mut cpu.regs.c, 0),
        0xc2 => set(&mut cpu.regs.d, 0),
        0xc3 => set(&mut cpu.regs.e, 0),
        0xc4 => set(&mut cpu.regs.h, 0),
        0xc5 => set(&mut cpu.regs.l, 0),
        0xc6 => bitfunc_hl(cpu, mmu, 0, set),
        0xc7 => set(&mut cpu.regs.a, 1),
        0xc8 => set(&mut cpu.regs.b, 1),
        0xc9 => set(&mut cpu.regs.c, 1),
        0xca => set(&mut cpu.regs.d, 1),
        0xcb => set(&mut cpu.regs.e, 1),
        0xcc => set(&mut cpu.regs.h, 1),
        0xcd => set(&mut cpu.regs.l, 1),
        0xce => bitfunc_hl(cpu, mmu, 1, set),
        0xcf => set(&mut cpu.regs.a, 1),
        0xd0 => set(&mut cpu.regs.b, 2),
        0xd1 => set(&mut cpu.regs.c, 2),
        0xd2 => set(&mut cpu.regs.d, 2),
        0xd3 => set(&mut cpu.regs.e, 2),
        0xd4 => set(&mut cpu.regs.h, 2),
        0xd5 => set(&mut cpu.regs.l, 2),
        0xd6 => bitfunc_hl(cpu, mmu, 2, set),
        0xd7 => set(&mut cpu.regs.a, 2),
        0xd8 => set(&mut cpu.regs.b, 3),
        0xd9 => set(&mut cpu.regs.c, 3),
        0xda => set(&mut cpu.regs.d, 3),
        0xdb => set(&mut cpu.regs.e, 3),
        0xdc => set(&mut cpu.regs.h, 3),
        0xdd => set(&mut cpu.regs.l, 3),
        0xde => bitfunc_hl(cpu, mmu, 3, set),
        0xdf => set(&mut cpu.regs.a, 3),
        0xe0 => set(&mut cpu.regs.b, 4),
        0xe1 => set(&mut cpu.regs.c, 4),
        0xe2 => set(&mut cpu.regs.d, 4),
        0xe3 => set(&mut cpu.regs.e, 4),
        0xe4 => set(&mut cpu.regs.h, 4),
        0xe5 => set(&mut cpu.regs.l, 4),
        0xe6 => bitfunc_hl(cpu, mmu, 4, set),
        0xe7 => set(&mut cpu.regs.a, 4),
        0xe8 => set(&mut cpu.regs.b, 5),
        0xe9 => set(&mut cpu.regs.c, 5),
        0xea => set(&mut cpu.regs.d, 5),
        0xeb => set(&mut cpu.regs.e, 5),
        0xec => set(&mut cpu.regs.h, 5),
        0xed => set(&mut cpu.regs.l, 5),
        0xee => bitfunc_hl(cpu, mmu, 5, set),
        0xef => set(&mut cpu.regs.a, 5),
        0xf0 => set(&mut cpu.regs.b, 6),
        0xf1 => set(&mut cpu.regs.c, 6),
        0xf2 => set(&mut cpu.regs.d, 6),
        0xf3 => set(&mut cpu.regs.e, 6),
        0xf4 => set(&mut cpu.regs.h, 6),
        0xf5 => set(&mut cpu.regs.l, 6),
        0xf6 => bitfunc_hl(cpu, mmu, 6, set),
        0xf7 => set(&mut cpu.regs.a, 6),
        0xf8 => set(&mut cpu.regs.b, 7),
        0xf9 => set(&mut cpu.regs.c, 7),
        0xfa => set(&mut cpu.regs.d, 7),
        0xfb => set(&mut cpu.regs.e, 7),
        0xfc => set(&mut cpu.regs.h, 7),
        0xfd => set(&mut cpu.regs.l, 7),
        0xfe => bitfunc_hl(cpu, mmu, 7, set),
        0xff => set(&mut cpu.regs.a, 7),
        v => {
            println!("Impossible cb instruction: {}", v);
            0
        }
    }
}

fn func_hl(cpu: &mut CPU, mmu: &mut MMU, func: fn(&mut u8, &mut u8) -> u8) -> u8 {
    let mut value = mmu.read_byte(cpu.regs.get_hl());
    let cycles = func(&mut value, &mut cpu.regs.f);
    mmu.write_byte(cpu.regs.get_hl(), value);
    8 + cycles
}

fn bitfunc_hl(cpu: &mut CPU, mmu: &mut MMU, n: u8, func: fn(&mut u8, u8) -> u8) -> u8 {
    let mut value = mmu.read_byte(cpu.regs.get_hl());
    let cycles = func(&mut value, n);
    mmu.write_byte(cpu.regs.get_hl(), value);
    8 + cycles
}

fn bit_hl(cpu: &mut CPU, mmu: &mut MMU, n: u8) -> u8 {
    let mut value = mmu.read_byte(cpu.regs.get_hl());
    let cycles = bit(&mut value, &mut cpu.regs.f, n);
    mmu.write_byte(cpu.regs.get_hl(), value);
    4 + cycles
}

fn rlc(register: &mut u8, f: &mut u8) -> u8 {
    let bit_7 = *register >> 7;
    *register = (*register << 1) | bit_7;
    *f = if bit_7 == 0 { 0 } else { CARRY } | if *register == 0 { ZERO } else { 0 };
    8
}

fn rrc(register: &mut u8, f: &mut u8) -> u8 {
    let bit_0 = *register & 1;
    *register = (*register >> 1) | (bit_0 << 7);
    *f = if bit_0 == 0 { 0 } else { CARRY } | if *register == 0 { ZERO } else { 0 };
    8
}

fn rl(register: &mut u8, f: &mut u8) -> u8 {
    let bit_7 = *register >> 7;
    *register = (*register << 1) | if *f & CARRY == CARRY { 1 } else { 0 };
    *f = if bit_7 == 0 { 0 } else { CARRY } | if *register == 0 { ZERO } else { 0 };
    8
}

fn rr(register: &mut u8, f: &mut u8) -> u8 {
    let old_carry = *f & CARRY;
    *f = if *register & 1 == 1 { CARRY } else { 0 };
    *register >>= 1;
    *register |= if old_carry == CARRY { 0x80 } else { 0 };
    *f |= if *register == 0 { ZERO } else { 0 };
    8
}

fn sla(register: &mut u8, f: &mut u8) -> u8 {
    let bit_7 = *register >> 7;
    *register <<= 1;
    *f = if bit_7 == 0 { 0 } else { CARRY } | if *register == 0 { ZERO } else { 0 };
    8
}

fn sra(register: &mut u8, f: &mut u8) -> u8 {
    let bit_0 = *register & 1;
    let bit_7 = *register & 0x80;
    *register = (*register >> 1) | bit_7;
    *f = if bit_0 == 0 { 0 } else { CARRY } | if *register == 0 { ZERO } else { 0 };
    8
}

fn swap(register: &mut u8, f: &mut u8) -> u8 {
    let lo = *register & 0xf;
    *register = (*register >> 4) | (lo << 4);
    *f = if *register == 0 { ZERO } else { 0 };
    8
}

fn srl(register: &mut u8, f: &mut u8) -> u8 {
    *f = if *register & 1 == 1 { CARRY } else { 0 };
    *register >>= 1;
    *f |= if *register == 0 { ZERO } else { 0 };
    8
}

fn bit(register: &mut u8, f: &mut u8, n: u8) -> u8 {
    *f = (*f & CARRY) | HALF_CARRY | if ((*register >> n) & 1) == 0 { ZERO } else { 0 };
    8
}

fn res(register: &mut u8, n: u8) -> u8 {
    *register &= !(1 << n);
    8
}

fn set(register: &mut u8, n: u8) -> u8 {
    *register |= 1 << n;
    8
}
