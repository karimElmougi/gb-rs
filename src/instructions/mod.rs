mod alu;
mod cb;
mod flags;
mod load;

use crate::cpu::CPU;
use crate::mmu::MMU;

use self::flags::CARRY;
use self::flags::HALF_CARRY;
use self::flags::SUB;
use self::flags::ZERO;

pub const INSTRUCTIONS: [(&'static str, fn(&mut CPU, &mut MMU) -> u8); 256] = [
    ("NOP", nop),                         // 0x01
    ("LD BC, NN", load::ld_bc_nn),        // 0x02
    ("LD (BC), A", load::ld_bc_a),        // 0x02
    ("INC BC", alu::inc_bc),              // 0x03
    ("INC B", alu::inc_b),                // 0x04
    ("DEC B", alu::dec_b),                // 0x05
    ("LD B, N", load::ld_b_n),            // 0x06
    ("RLCA", alu::rlca),                  // 0x07
    ("LD (NN), SP", load::ld_nn_sp),      // 0x08
    ("ADD HL, BC", alu::add_hl_bc),       // 0x09
    ("LD A, (BC)", load::ld_a_bc),        // 0x0a
    ("DEC BC", alu::dec_bc),              // 0x0b
    ("INC C", alu::inc_c),                // 0x0c
    ("DEC C", alu::dec_c),                // 0x0d
    ("LD C, N", load::ld_c_n),            // 0x0e
    ("RRCA", alu::rrca),                  // 0x0f
    ("STOP", unimplemented),              // 0x10
    ("LD DE, NN", load::ld_de_nn),        // 0x11
    ("LD (DE), A", load::ld_de_a),        // 0x12
    ("INC DE", alu::inc_de),              // 0x13
    ("INC D", alu::inc_d),                // 0x14
    ("DEC D", alu::dec_d),                // 0x15
    ("LD D, N", load::ld_d_n),            // 0x16
    ("RLA", alu::rla),                    // 0x17
    ("JR N", jr_n),                       // 0x18
    ("ADD HL, DE", alu::add_hl_de),       // 0x19
    ("LD A, (DE)", load::ld_a_de),        // 0x1a
    ("DEC DE", alu::dec_de),              // 0x1b
    ("INC E", alu::inc_e),                // 0x1c
    ("DEC E", alu::dec_e),                // 0x1d
    ("LD E, N", load::ld_e_n),            // 0x1e
    ("RRA", alu::rra),                    // 0x1f
    ("JR NZ, N", jr_nz),                  // 0x20
    ("LD HL, NN", load::ld_hl_nn),        // 0x21
    ("LDI (HL), A", load::ldi_hl_a),      // 0x22
    ("INC HL", alu::inc_hl),              // 0x23
    ("INC H", alu::inc_h),                // 0x24
    ("DEC H", alu::dec_h),                // 0x25
    ("LD H, N", load::ld_h_n),            // 0x26
    ("DAA", daa),                         // 0x27
    ("JR Z, N", jr_z),                    // 0x28
    ("ADD HL, HL", alu::add_hl_hl),       // 0x29
    ("LDI A, (HL)", load::ldi_a_hl),      // 0x2a
    ("DEC HL", alu::dec_hl),              // 0x2b
    ("INC L", alu::inc_l),                // 0x2c
    ("DEC L", alu::dec_l),                // 0x2d
    ("LD L, N", load::ld_l_n),            // 0x2e
    ("CPL", alu::cpl),                    // 0x2f
    ("JR NC, N", jr_nc),                  // 0x30
    ("LD SP, NN", load::ld_sp_nn),        // 0x31
    ("LDD (HL), A", load::ldd_hl_a),      // 0x32
    ("INC SP", alu::inc_sp),              // 0x33
    ("INC (HL)", alu::inc_at_hl),         // 0x34
    ("DEC (HL)", alu::dec_at_hl),         // 0x35
    ("LD (HL), N", load::ld_hl_nn),       // 0x36
    ("SCF", scf),                         // 0x37
    ("JR C, N", jr_c),                    // 0x38
    ("ADD HL, SP", alu::add_hl_sp),       // 0x39
    ("LDD A, (HL)", load::ldd_a_hl),      // 0x3a
    ("DEC SP", alu::dec_sp),              // 0x3b
    ("INC A", alu::inc_a),                // 0x3c
    ("DEC A", alu::dec_a),                // 0x3d
    ("LD A, N", load::ld_a_n),            // 0x3e
    ("CCF", ccf),                         // 0x3f
    ("LD B, B", load::ld_b_b),            // 0x40
    ("LD B, C", load::ld_b_c),            // 0x41
    ("LD B, D", load::ld_b_d),            // 0x42
    ("LD B, E", load::ld_b_e),            // 0x43
    ("LD B, H", load::ld_b_h),            // 0x44
    ("LD B, L", load::ld_b_l),            // 0x45
    ("LD B, (HL)", load::ld_b_hl),        // 0x46
    ("LD B, A", load::ld_b_a),            // 0x47
    ("LD C, B", load::ld_c_b),            // 0x48
    ("LD C, C", load::ld_c_c),            // 0x49
    ("LD C, D", load::ld_c_d),            // 0x4a
    ("LD C, E", load::ld_c_e),            // 0x4b
    ("LD C, H", load::ld_c_h),            // 0x4c
    ("LD C, L", load::ld_c_l),            // 0x4d
    ("LD C, (HL)", load::ld_c_hl),        // 0x4e
    ("LD C, A", load::ld_c_a),            // 0x4f
    ("LD D, B", load::ld_d_b),            // 0x50
    ("LD D, C", load::ld_d_c),            // 0x51
    ("LD D, D", load::ld_d_d),            // 0x52
    ("LD D, E", load::ld_d_e),            // 0x53
    ("LD D, H", load::ld_d_h),            // 0x54
    ("LD D, L", load::ld_d_l),            // 0x55
    ("LD D, (HL)", load::ld_d_hl),        // 0x56
    ("LD D, A", load::ld_d_a),            // 0x57
    ("LD E, B", load::ld_e_b),            // 0x58
    ("LD E, C", load::ld_e_c),            // 0x59
    ("LD E, D", load::ld_e_d),            // 0x5a
    ("LD E, E", load::ld_e_e),            // 0x5b
    ("LD E, H", load::ld_e_h),            // 0x5c
    ("LD E, L", load::ld_e_l),            // 0x5d
    ("LD E, (HL)", load::ld_e_hl),        // 0x5e
    ("LD E, A", load::ld_e_a),            // 0x5f
    ("LD H, B", load::ld_h_b),            // 0x60
    ("LD H, C", load::ld_h_c),            // 0x61
    ("LD H, D", load::ld_h_d),            // 0x62
    ("LD H, E", load::ld_h_e),            // 0x63
    ("LD H, H", load::ld_h_h),            // 0x64
    ("LD H, L", load::ld_h_l),            // 0x65
    ("LD H, (HL)", load::ld_h_hl),        // 0x66
    ("LD H, A", load::ld_h_a),            // 0x67
    ("LD L, B", load::ld_l_b),            // 0x68
    ("LD L, C", load::ld_l_c),            // 0x69
    ("LD L, D", load::ld_l_d),            // 0x6a
    ("LD L, E", load::ld_l_e),            // 0x6b
    ("LD L, H", load::ld_l_h),            // 0x6c
    ("LD L, L", load::ld_l_l),            // 0x6d
    ("LD L, (HL)", load::ld_l_hl),        // 0x6e
    ("LD L, A", load::ld_l_a),            // 0x6f
    ("LD (HL), B", load::ld_hl_b),        // 0x70
    ("LD (HL), C", load::ld_hl_c),        // 0x71
    ("LD (HL), D", load::ld_hl_d),        // 0x72
    ("LD (HL), E", load::ld_hl_e),        // 0x73
    ("LD (HL), H", load::ld_hl_h),        // 0x74
    ("LD (HL), L", load::ld_hl_l),        // 0x75
    ("HALT", halt),                       // 0x76
    ("LD (HL), A", load::ld_hl_a),        // 0x77
    ("LD A, B", load::ld_a_b),            // 0x78
    ("LD A, C", load::ld_a_c),            // 0x79
    ("LD A, D", load::ld_a_d),            // 0x7a
    ("LD A, E", load::ld_a_e),            // 0x7b
    ("LD A, H", load::ld_a_h),            // 0x7c
    ("LD A, L", load::ld_a_l),            // 0x7d
    ("LD A, (HL)", load::ld_a_hl),        // 0x7e
    ("LD A, A", load::ld_a_a),            // 0x7f
    ("ADD A, B", alu::add_a_b),           // 0x80
    ("ADD A, C", alu::add_a_c),           // 0x81
    ("ADD A, D", alu::add_a_d),           // 0x82
    ("ADD A, E", alu::add_a_e),           // 0x83
    ("ADD A, H", alu::add_a_h),           // 0x84
    ("ADD A, L", alu::add_a_l),           // 0x85
    ("ADD A, (HL)", alu::add_a_hl),       // 0x86
    ("ADD A", alu::add_a_a),              // 0x87
    ("ADC B", alu::adc_a_b),              // 0x88
    ("ADC C", alu::adc_a_c),              // 0x89
    ("ADC D", alu::adc_a_d),              // 0x8a
    ("ADC E", alu::adc_a_e),              // 0x8b
    ("ADC H", alu::adc_a_h),              // 0x8c
    ("ADC L", alu::adc_a_l),              // 0x8d
    ("ADC (HL)", alu::adc_a_hl),          // 0x8e
    ("ADC A", alu::adc_a_a),              // 0x8f
    ("SUB B", alu::sub_a_b),              // 0x90
    ("SUB C", alu::sub_a_c),              // 0x91
    ("SUB D", alu::sub_a_d),              // 0x92
    ("SUB E", alu::sub_a_e),              // 0x93
    ("SUB H", alu::sub_a_h),              // 0x94
    ("SUB L", alu::sub_a_l),              // 0x95
    ("SUB (HL)", alu::sub_a_hl),          // 0x96
    ("SUB A", alu::sub_a_a),              // 0x97
    ("SBC B", alu::sbc_a_b),              // 0x98
    ("SBC C", alu::sbc_a_c),              // 0x99
    ("SBC D", alu::sbc_a_d),              // 0x9a
    ("SBC E", alu::sbc_a_e),              // 0x9b
    ("SBC H", alu::sbc_a_h),              // 0x9c
    ("SBC L", alu::sbc_a_l),              // 0x9d
    ("SBC (HL)", alu::sbc_a_hl),          // 0x9e
    ("SBC A", alu::sbc_a_a),              // 0x9f
    ("AND B", alu::and_a_b),              // 0xa0
    ("AND C", alu::and_a_c),              // 0xa1
    ("AND D", alu::and_a_d),              // 0xa2
    ("AND E", alu::and_a_e),              // 0xa3
    ("AND H", alu::and_a_h),              // 0xa4
    ("AND L", alu::and_a_l),              // 0xa5
    ("AND (HL)", alu::and_a_hl),          // 0xa6
    ("AND A", alu::and_a_a),              // 0xa7
    ("XOR B", alu::xor_a_b),              // 0xa8
    ("XOR C", alu::xor_a_c),              // 0xa9
    ("XOR D", alu::xor_a_d),              // 0xaa
    ("XOR E", alu::xor_a_e),              // 0xab
    ("XOR H", alu::xor_a_h),              // 0xac
    ("XOR L", alu::xor_a_l),              // 0xad
    ("XOR (HL)", alu::xor_a_hl),          // 0xae
    ("XOR A", alu::xor_a_a),              // 0xaf
    ("OR B", alu::or_a_b),                // 0xb0
    ("OR C", alu::or_a_c),                // 0xb1
    ("OR D", alu::or_a_d),                // 0xb2
    ("OR E", alu::or_a_e),                // 0xb3
    ("OR H", alu::or_a_h),                // 0xb4
    ("OR L", alu::or_a_l),                // 0xb5
    ("OR (HL)", alu::or_a_hl),            // 0xb6
    ("OR A", alu::or_a_a),                // 0xb7
    ("CP B", alu::cp_a_b),                // 0xb8
    ("CP C", alu::cp_a_c),                // 0xb9
    ("CP D", alu::cp_a_d),                // 0xba
    ("CP E", alu::cp_a_e),                // 0xbb
    ("CP H", alu::cp_a_h),                // 0xbc
    ("CP L", alu::cp_a_l),                // 0xbd
    ("CP (HL)", alu::cp_a_hl),            // 0xbe
    ("CP A", alu::cp_a_a),                // 0xbf
    ("RET NZ", ret_nz),                   // 0xc0
    ("POP BC", pop_bc),                   // 0xc1
    ("JP N6, NN", jp_nz),                 // 0xc2
    ("JP NN", jp_n),                      // 0xc3
    ("CALL NZ, NN", call_nz),             // 0xc4
    ("PUSH BC", push_bc),                 // 0xc5
    ("ADD A, N", alu::add_a_n),           // 0xc6
    ("RST 0x00", rst_0x00),               // 0xc7
    ("RET Z", ret_z),                     // 0xc8
    ("RET", ret_n),                       // 0xc9
    ("JP Z, NN", jp_z),                   // 0xca
    ("CB N", cb::execute_cb),             // 0xcb
    ("CALL Z, NN", call_z),               // 0xcc
    ("CALL NN", call_n),                  // 0xcd
    ("ADC N", alu::adc_a_n),              // 0xce
    ("RST 0x08", rst_0x08),               // 0xcf
    ("RET NC", ret_nc),                   // 0xd0
    ("POP DE", pop_de),                   // 0xd1
    ("JP NC, NN", jp_nc),                 // 0xd2
    ("unimplemented", unimplemented),     // 0xd3
    ("CALL NC, NN", call_nc),             // 0xd4
    ("PUSH DE", push_de),                 // 0xd5
    ("SUB N", alu::sub_a_n),              // 0xd6
    ("RST 0x10", rst_0x10),               // 0xd7
    ("RET C", ret_c),                     // 0xd8
    ("RETI", ret_i),                      // 0xd9
    ("JP C, NN", jp_c),                   // 0xda
    ("unimplemented", unimplemented),     // 0xdb
    ("CALL C, NN", call_c),               // 0xdc
    ("unimplemented", unimplemented),     // 0xdd
    ("SBC N", alu::sbc_a_n),              // 0xde
    ("RST 0x18", rst_0x18),               // 0xdf
    ("LD (0xFF00 + N), A", load::ld_n_a), // 0xe0
    ("POP HL", pop_hl),                   // 0xe1
    ("LD (0xFF00 + C), A", load::ld_c_a), // 0xe2
    ("unimplemented", unimplemented),     // 0xe3
    ("unimplemented", unimplemented),     // 0xe4
    ("PUSH HL", push_hl),                 // 0xe5
    ("AND N", alu::and_a_n),              // 0xe6
    ("RST 0x20", rst_0x20),               // 0xe7
    ("ADD SP,N", alu::add_sp_n),          // 0xe8
    ("JP HL", jp_hl),                     // 0xe9
    ("LD (NN), A", load::ld_nn_a),        // 0xea
    ("unimplemented", unimplemented),     // 0xeb
    ("unimplemented", unimplemented),     // 0xec
    ("unimplemented", unimplemented),     // 0xed
    ("XOR N", alu::xor_a_n),              // 0xee
    ("RST 0x28", rst_0x28),               // 0xef
    ("LD A, (0xFF00 + N)", load::ld_a_n), // 0xf0
    ("POP AF", pop_af),                   // 0xf1
    ("LD A, (0xFF00 + C)", load::ld_a_c), // 0xf2
    ("DI", di),                           // 0xf3
    ("unimplemented", unimplemented),     // 0xf4
    ("PUSH AF", push_af),                 // 0xf5
    ("OR N", alu::or_a_n),                // 0xf6
    ("RST 0x30", rst_0x30),               // 0xf7
    ("LD HL, SP+N", load::ld_hl_sp_n),    // 0xf8
    ("LD SP, HL", load::ld_sp_hl),        // 0xf9
    ("LD A, (NN)", load::ld_a_nn),        // 0xfa
    ("EI", ei),                           // 0xfb
    ("unimplemented", unimplemented),     // 0xfc
    ("unimplemented", unimplemented),     // 0xfd
    ("CP N", alu::cp_a_n),                // 0xfe
    ("RST 0x38", rst_0x38),               // 0xff
];

fn nop(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn halt(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.is_halted = true;
    4
}

fn unimplemented(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    println!(
        "Unimplemented instruction: 0x{:x}",
        mmu.read_byte(cpu.regs.pc - 1)
    );
    4
}

fn scf(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.f |= CARRY;
    cpu.regs.f &= !HALF_CARRY;
    cpu.regs.f &= !SUB;
    4
}

fn ccf(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.f ^= CARRY;
    cpu.regs.f &= !HALF_CARRY;
    cpu.regs.f &= !SUB;
    4
}

fn jr_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    jr_cc(cpu, mmu, true)
}

fn jr_z(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    jr_cc(cpu, mmu, cpu.regs.f & ZERO == ZERO)
}

fn jr_nz(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    jr_cc(cpu, mmu, cpu.regs.f & ZERO != ZERO)
}

fn jr_c(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    jr_cc(cpu, mmu, cpu.regs.f & CARRY == CARRY)
}

fn jr_nc(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    jr_cc(cpu, mmu, cpu.regs.f & CARRY != CARRY)
}

fn jr_cc(cpu: &mut CPU, mmu: &mut MMU, condition: bool) -> u8 {
    if condition {
        cpu.regs.pc += cpu.fetch_byte(mmu) as i8 as u16;
        return 12;
    }
    8
}

fn jp_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    jp_cc(cpu, mmu, true)
}

fn jp_z(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    jp_cc(cpu, mmu, cpu.regs.f & ZERO == ZERO)
}

fn jp_nz(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    jp_cc(cpu, mmu, cpu.regs.f & ZERO != ZERO)
}

fn jp_c(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    jp_cc(cpu, mmu, cpu.regs.f & CARRY == CARRY)
}

fn jp_nc(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    jp_cc(cpu, mmu, cpu.regs.f & CARRY != CARRY)
}

fn jp_cc(cpu: &mut CPU, mmu: &mut MMU, condition: bool) -> u8 {
    if condition {
        cpu.regs.pc = mmu.read_word(cpu.regs.pc);
        return 16;
    }
    12
}

fn jp_hl(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.pc = cpu.regs.get_hl();
    4
}

fn call_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    call_cc(cpu, mmu, true)
}

fn call_z(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    call_cc(cpu, mmu, cpu.regs.f & ZERO == ZERO)
}

fn call_nz(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    call_cc(cpu, mmu, cpu.regs.f & ZERO != ZERO)
}

fn call_c(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    call_cc(cpu, mmu, cpu.regs.f & CARRY == CARRY)
}

fn call_nc(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    call_cc(cpu, mmu, cpu.regs.f & CARRY != CARRY)
}

fn call_cc(cpu: &mut CPU, mmu: &mut MMU, condition: bool) -> u8 {
    if condition {
        cpu.regs.sp -= 2;
        mmu.write_word(cpu.regs.sp, cpu.regs.pc + 2);
        cpu.regs.pc = mmu.read_word(cpu.regs.pc);
        return 24;
    }
    12
}

fn rst_0x00(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    rst(cpu, mmu, 0x00)
}

fn rst_0x08(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    rst(cpu, mmu, 0x08)
}

fn rst_0x10(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    rst(cpu, mmu, 0x10)
}

fn rst_0x18(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    rst(cpu, mmu, 0x18)
}

fn rst_0x20(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    rst(cpu, mmu, 0x00)
}

fn rst_0x28(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    rst(cpu, mmu, 0x28)
}

fn rst_0x30(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    rst(cpu, mmu, 0x30)
}

fn rst_0x38(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    rst(cpu, mmu, 0x38)
}

fn ret_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    ret_cc(cpu, mmu, true);
    16
}

fn ret_z(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    ret_cc(cpu, mmu, cpu.regs.f & ZERO == ZERO)
}

fn ret_nz(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    ret_cc(cpu, mmu, cpu.regs.f & ZERO != ZERO)
}

fn ret_c(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    ret_cc(cpu, mmu, cpu.regs.f & CARRY == CARRY)
}

fn ret_nc(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    ret_cc(cpu, mmu, cpu.regs.f & CARRY != CARRY)
}

fn ret_cc(cpu: &mut CPU, mmu: &mut MMU, condition: bool) -> u8 {
    if condition {
        cpu.regs.pc = mmu.read_word(cpu.regs.sp);
        cpu.regs.sp += 2;
        return 20;
    }
    8
}

fn ret_i(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.enabling_interrupts = true;
    ret_n(cpu, mmu)
}

fn rst(cpu: &mut CPU, mmu: &mut MMU, addr: u16) -> u8 {
    cpu.regs.sp -= 2;
    mmu.write_word(cpu.regs.pc, cpu.regs.pc + 1);
    cpu.regs.pc = addr;
    32
}

fn push_af(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.sp -= 2;
    mmu.write_word(cpu.regs.sp, cpu.regs.get_af());
    16
}

fn push_bc(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.sp -= 2;
    mmu.write_word(cpu.regs.sp, cpu.regs.get_bc());
    16
}

fn push_de(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.sp -= 2;
    mmu.write_word(cpu.regs.sp, cpu.regs.get_de());
    16
}

fn push_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.sp -= 2;
    mmu.write_word(cpu.regs.sp, cpu.regs.get_hl());
    16
}

fn pop_af(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.set_af(mmu.read_word(cpu.regs.sp));
    cpu.regs.sp += 2;
    12
}

fn pop_bc(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.set_bc(mmu.read_word(cpu.regs.sp));
    cpu.regs.sp += 2;
    12
}

fn pop_de(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.set_de(mmu.read_word(cpu.regs.sp));
    cpu.regs.sp += 2;
    12
}

fn pop_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.set_hl(mmu.read_word(cpu.regs.sp));
    cpu.regs.sp += 2;
    12
}

fn ei(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.enabling_interrupts = true;
    4
}

fn di(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.interrupts_enabled = false;
    4
}

fn daa(cpu: &mut CPU, _: &mut MMU) -> u8 {
    if cpu.regs.f & SUB == SUB {
        cpu.regs.a += if cpu.regs.f & HALF_CARRY == HALF_CARRY {
            cpu.regs.f &= !HALF_CARRY;
            if cpu.regs.f & CARRY == CARRY {
                0x9a
            } else {
                0xfa
            }
        } else if cpu.regs.f & CARRY == CARRY {
            0xa0
        } else {
            0
        };
    } else {
        if cpu.regs.a > 0x99 || cpu.regs.f & CARRY == CARRY {
            cpu.regs.a += 0x60;
            cpu.regs.f |= CARRY;
        }
        if (cpu.regs.a & 0xf) > 0x09 || cpu.regs.f & HALF_CARRY == HALF_CARRY {
            cpu.regs.a += 0x06;
            cpu.regs.f &= !HALF_CARRY;
        }
    }
    if cpu.regs.a == 0 {
        cpu.regs.f |= ZERO;
    } else {
        cpu.regs.f &= !ZERO;
    }
    4
}
