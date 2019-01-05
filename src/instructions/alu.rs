use super::flags::CARRY;
use super::flags::HALF_CARRY;
use super::flags::SUB;
use super::flags::ZERO;
use crate::cpu::CPU;
use crate::mmu::MMU;

pub fn inc_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (a, f) = inc_r(cpu.regs.a, cpu.regs.f);
    cpu.regs.a = a;
    cpu.regs.f = f;
    4
}

pub fn inc_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (b, f) = inc_r(cpu.regs.b, cpu.regs.f);
    cpu.regs.b = b;
    cpu.regs.f = f;
    4
}

pub fn inc_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (c, f) = inc_r(cpu.regs.c, cpu.regs.f);
    cpu.regs.c = c;
    cpu.regs.f = f;
    4
}

pub fn inc_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (d, f) = inc_r(cpu.regs.d, cpu.regs.f);
    cpu.regs.d = d;
    cpu.regs.f = f;
    4
}

pub fn inc_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (e, f) = inc_r(cpu.regs.e, cpu.regs.f);
    cpu.regs.e = e;
    cpu.regs.f = f;
    4
}

pub fn inc_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (h, f) = inc_r(cpu.regs.h, cpu.regs.f);
    cpu.regs.h = h;
    cpu.regs.f = f;
    4
}

pub fn inc_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (l, f) = inc_r(cpu.regs.l, cpu.regs.f);
    cpu.regs.l = l;
    cpu.regs.f = f;
    4
}

pub fn dec_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (a, f) = dec_r(cpu.regs.a, cpu.regs.f);
    cpu.regs.a = a;
    cpu.regs.f = f;
    4
}

pub fn dec_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (b, f) = dec_r(cpu.regs.b, cpu.regs.f);
    cpu.regs.b = b;
    cpu.regs.f = f;
    4
}

pub fn dec_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (c, f) = dec_r(cpu.regs.c, cpu.regs.f);
    cpu.regs.a = c;
    cpu.regs.f = f;
    4
}

pub fn dec_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (d, f) = dec_r(cpu.regs.d, cpu.regs.f);
    cpu.regs.a = d;
    cpu.regs.f = f;
    4
}

pub fn dec_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (e, f) = dec_r(cpu.regs.e, cpu.regs.f);
    cpu.regs.a = e;
    cpu.regs.f = f;
    4
}

pub fn dec_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (h, f) = dec_r(cpu.regs.h, cpu.regs.f);
    cpu.regs.a = h;
    cpu.regs.f = f;
    4
}

pub fn dec_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (l, f) = dec_r(cpu.regs.l, cpu.regs.f);
    cpu.regs.a = l;
    cpu.regs.f = f;
    4
}

pub fn inc_bc(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.set_bc(cpu.regs.get_bc() + 1);
    8
}

pub fn inc_de(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.set_de(cpu.regs.get_de() + 1);
    8
}

pub fn inc_hl(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.set_hl(cpu.regs.get_hl() + 1);
    8
}

pub fn inc_sp(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.sp += 1;
    8
}

pub fn dec_bc(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.set_bc(cpu.regs.get_bc() - 1);
    8
}

pub fn dec_de(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.set_de(cpu.regs.get_de() - 1);
    8
}

pub fn dec_hl(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.set_hl(cpu.regs.get_hl() - 1);
    8
}

pub fn dec_sp(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.sp -= 1;
    8
}

pub fn inc_at_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let addr = cpu.regs.get_hl();
    let old_carry = cpu.regs.f & CARRY;
    let (r, mut f) = add(mmu.read_byte(addr), 1);
    if old_carry == CARRY {
        f |= CARRY;
    } else {
        f &= !CARRY;
    }
    cpu.regs.f = f;
    mmu.write_byte(addr, r);
    12
}

pub fn dec_at_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let addr = cpu.regs.get_hl();
    let old_carry = cpu.regs.f & CARRY;
    let (r, mut f) = sub(mmu.read_byte(addr), 1);
    if old_carry == CARRY {
        f |= CARRY;
    } else {
        f &= !CARRY;
    }
    if r & 0xf == 0xf {
        f |= HALF_CARRY;
    } else {
        f &= !HALF_CARRY;
    }
    cpu.regs.f = f;
    mmu.write_byte(addr, r);
    12
}

pub fn add_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_a(cpu, cpu.regs.a)
}

pub fn add_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_a(cpu, cpu.regs.b)
}

pub fn add_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_a(cpu, cpu.regs.c)
}

pub fn add_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_a(cpu, cpu.regs.d)
}

pub fn add_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_a(cpu, cpu.regs.e)
}

pub fn add_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_a(cpu, cpu.regs.h)
}

pub fn add_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_a(cpu, cpu.regs.l)
}

pub fn add_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + add_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

pub fn add_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + add_a(cpu, n)
}

pub fn add_a(cpu: &mut CPU, operand: u8) -> u8 {
    let (a, f) = add(cpu.regs.a, operand);
    cpu.regs.a = a;
    cpu.regs.f = f;
    4
}

pub fn add_hl_bc(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_hl(cpu, cpu.regs.get_bc())
}

pub fn add_hl_de(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_hl(cpu, cpu.regs.get_de())
}

pub fn add_hl_hl(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_hl(cpu, cpu.regs.get_hl())
}

pub fn add_hl_sp(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_hl(cpu, cpu.regs.sp)
}

pub fn add_hl(cpu: &mut CPU, operand: u16) -> u8 {
    let (hl, f) = add_16(cpu.regs.get_hl(), operand, cpu.regs.f);
    cpu.regs.set_hl(hl);
    cpu.regs.f = f;
    8
}

pub fn add_sp_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    let sp_lo = (cpu.regs.sp & 0xff) as u8;
    let (_, f) = add(n, sp_lo);
    cpu.regs.f = f & !ZERO & !SUB;
    cpu.regs.sp += (n as i8) as u16;
    16
}

pub fn adc_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    adc_a(cpu, cpu.regs.a)
}

pub fn adc_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    adc_a(cpu, cpu.regs.b)
}

pub fn adc_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    adc_a(cpu, cpu.regs.c)
}

pub fn adc_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    adc_a(cpu, cpu.regs.d)
}

pub fn adc_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    adc_a(cpu, cpu.regs.e)
}

pub fn adc_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    adc_a(cpu, cpu.regs.h)
}

pub fn adc_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    adc_a(cpu, cpu.regs.l)
}

pub fn adc_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + adc_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

pub fn adc_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + adc_a(cpu, n)
}

pub fn adc_a(cpu: &mut CPU, operand: u8) -> u8 {
    let (a, f) = addc(cpu.regs.a, operand, cpu.regs.f & CARRY == CARRY);
    cpu.regs.a = a;
    cpu.regs.f = f;
    4
}

pub fn sub_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sub_a(cpu, cpu.regs.a)
}

pub fn sub_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sub_a(cpu, cpu.regs.b)
}

pub fn sub_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sub_a(cpu, cpu.regs.c)
}

pub fn sub_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sub_a(cpu, cpu.regs.d)
}

pub fn sub_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sub_a(cpu, cpu.regs.e)
}

pub fn sub_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sub_a(cpu, cpu.regs.h)
}

pub fn sub_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sub_a(cpu, cpu.regs.l)
}

pub fn sub_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + sub_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

pub fn sub_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + sub_a(cpu, n)
}

pub fn sub_a(cpu: &mut CPU, operand: u8) -> u8 {
    let (a, f) = sub(cpu.regs.a, operand);
    cpu.regs.a = a;
    cpu.regs.f = f;
    4
}

pub fn sbc_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sbc_a(cpu, cpu.regs.a)
}

pub fn sbc_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sbc_a(cpu, cpu.regs.b)
}

pub fn sbc_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sbc_a(cpu, cpu.regs.c)
}

pub fn sbc_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sbc_a(cpu, cpu.regs.d)
}

pub fn sbc_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sbc_a(cpu, cpu.regs.e)
}

pub fn sbc_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sbc_a(cpu, cpu.regs.h)
}

pub fn sbc_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sbc_a(cpu, cpu.regs.l)
}

pub fn sbc_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + sbc_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

pub fn sbc_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + sbc_a(cpu, n)
}

pub fn sbc_a(cpu: &mut CPU, operand: u8) -> u8 {
    let (a, f) = subc(cpu.regs.a, operand, cpu.regs.f & CARRY == CARRY);
    cpu.regs.a = a;
    cpu.regs.f = f;
    4
}

pub fn and_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    and_a(cpu, cpu.regs.a)
}

pub fn and_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    and_a(cpu, cpu.regs.b)
}

pub fn and_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    and_a(cpu, cpu.regs.c)
}

pub fn and_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    and_a(cpu, cpu.regs.d)
}

pub fn and_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    and_a(cpu, cpu.regs.e)
}

pub fn and_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    and_a(cpu, cpu.regs.h)
}

pub fn and_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    and_a(cpu, cpu.regs.l)
}

pub fn and_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + and_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

pub fn and_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + and_a(cpu, n)
}

pub fn and_a(cpu: &mut CPU, operand: u8) -> u8 {
    cpu.regs.a &= operand;
    cpu.regs.f = HALF_CARRY | if cpu.regs.a == 0 { ZERO } else { 0 };
    4
}

pub fn xor_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    xor_a(cpu, cpu.regs.a)
}

pub fn xor_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    xor_a(cpu, cpu.regs.b)
}

pub fn xor_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    xor_a(cpu, cpu.regs.c)
}

pub fn xor_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    xor_a(cpu, cpu.regs.d)
}

pub fn xor_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    xor_a(cpu, cpu.regs.e)
}

pub fn xor_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    xor_a(cpu, cpu.regs.h)
}

pub fn xor_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    xor_a(cpu, cpu.regs.l)
}

pub fn xor_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + xor_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

pub fn xor_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + xor_a(cpu, n)
}

pub fn xor_a(cpu: &mut CPU, operand: u8) -> u8 {
    cpu.regs.a ^= operand;
    cpu.regs.f = if cpu.regs.a == 0 { ZERO } else { 0 };
    4
}

pub fn or_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    or_a(cpu, cpu.regs.a)
}

pub fn or_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    or_a(cpu, cpu.regs.b)
}

pub fn or_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    or_a(cpu, cpu.regs.c)
}

pub fn or_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    or_a(cpu, cpu.regs.d)
}

pub fn or_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    or_a(cpu, cpu.regs.e)
}

pub fn or_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    or_a(cpu, cpu.regs.h)
}

pub fn or_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    or_a(cpu, cpu.regs.l)
}

pub fn or_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + or_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

pub fn or_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + or_a(cpu, n)
}

pub fn or_a(cpu: &mut CPU, operand: u8) -> u8 {
    cpu.regs.a |= operand;
    cpu.regs.f = if cpu.regs.a == 0 { ZERO } else { 0 };
    4
}

pub fn cp_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cp_a(cpu, cpu.regs.a)
}

pub fn cp_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cp_a(cpu, cpu.regs.b)
}

pub fn cp_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cp_a(cpu, cpu.regs.c)
}

pub fn cp_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cp_a(cpu, cpu.regs.d)
}

pub fn cp_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cp_a(cpu, cpu.regs.e)
}

pub fn cp_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cp_a(cpu, cpu.regs.h)
}

pub fn cp_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cp_a(cpu, cpu.regs.l)
}

pub fn cp_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + cp_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

pub fn cp_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + cp_a(cpu, n)
}

pub fn cp_a(cpu: &mut CPU, operand: u8) -> u8 {
    let (_, f) = sub(cpu.regs.a, operand);
    cpu.regs.f = f;
    4
}

pub fn rlca(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let bit_7 = cpu.regs.a >> 7;
    cpu.regs.a = (cpu.regs.a << 1) | bit_7;
    cpu.regs.f = if bit_7 == 1 { CARRY } else { 0 };
    4
}

pub fn rla(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let bit_7 = cpu.regs.a >> 7;
    cpu.regs.a <<= 1;
    cpu.regs.a |= if cpu.regs.f & CARRY == CARRY { 1 } else { 0 };
    cpu.regs.f = if bit_7 == 1 { CARRY } else { 0 };
    4
}

pub fn rrca(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let bit_0 = cpu.regs.a & 1;
    cpu.regs.a = (cpu.regs.a >> 1) | (bit_0 << 7);
    cpu.regs.f = if bit_0 == 1 { CARRY } else { 0 };
    4
}

pub fn rra(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let old_carry = (cpu.regs.f & CARRY) << 3;
    cpu.regs.f = if cpu.regs.a & 1 == 1 { CARRY } else { 0 };
    cpu.regs.a = (cpu.regs.a >> 1) | old_carry;
    4
}

pub fn cpl(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.a = !cpu.regs.a;
    cpu.regs.f |= SUB | HALF_CARRY;
    4
}

pub fn inc_r(reg: u8, f: u8) -> (u8, u8) {
    let (r, mut new_f) = add(reg, 1);
    new_f = if f & CARRY == CARRY {
        new_f | CARRY
    } else {
        new_f & !CARRY
    };
    (r, new_f)
}

pub fn dec_r(reg: u8, f: u8) -> (u8, u8) {
    let (r, mut new_f) = sub(reg, 1);
    new_f = if f & CARRY == CARRY {
        new_f | CARRY
    } else {
        new_f & !CARRY
    };
    (r, new_f)
}

pub fn add_16(op1: u16, op2: u16, f: u8) -> (u16, u8) {
    let r = op1 as u32 + op2 as u32;
    let f = f & ZERO
        | if r > 0xffff { CARRY } else { 0 }
        | if ((op1 & 0xfff) + (op2 & 0xfff)) > 0xfff {
            HALF_CARRY
        } else {
            0
        };
    (r as u16, f)
}

pub fn add(op1: u8, op2: u8) -> (u8, u8) {
    add_impl(op1, op2, false, false)
}

pub fn addc(op1: u8, op2: u8, carry_on: bool) -> (u8, u8) {
    add_impl(op1, op2, true, carry_on)
}

pub fn add_impl(op1: u8, op2: u8, use_carry: bool, carry_on: bool) -> (u8, u8) {
    let carry = if use_carry && carry_on { 1 } else { 0 };
    let r16 = op1 as u16 + op2 as u16 + carry as u16;
    let r = r16 as u8;
    let f = 0
        | if r == 0 { ZERO } else { 0 }
        | if r16 > 0xff { CARRY } else { 0 }
        | if (op1 & 0xf) + (op2 & 0xf) + carry > 0xf {
            HALF_CARRY
        } else {
            0
        };
    (r, f)
}

pub fn sub(op1: u8, op2: u8) -> (u8, u8) {
    sub_impl(op1, op2, false, false)
}

pub fn subc(op1: u8, op2: u8, carry_on: bool) -> (u8, u8) {
    sub_impl(op1, op2, true, carry_on)
}

pub fn sub_impl(op1: u8, op2: u8, use_borrow: bool, carry_on: bool) -> (u8, u8) {
    let borrow = if use_borrow && carry_on { 1 } else { 0 };
    let r16 = op1 as u16 - op2 as u16 - borrow as u16;
    let r = r16 as u8;
    let f = SUB
        | if r == 0 { ZERO } else { 0 }
        | if r16 > 0xff { CARRY } else { 0 }
        | if (op1 & 0xf) - (op2 & 0xf) - borrow > 0xf {
            HALF_CARRY
        } else {
            0
        };
    (r, f)
}
