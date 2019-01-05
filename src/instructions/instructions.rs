include!("extended_instructions.rs");

const CARRY: u8 = 0b0001_0000;
const HALF_CARRY: u8 = 0b0010_0000;
const SUB: u8 = 0b0100_0000;
const ZERO: u8 = 0b1000_0000;

const INSTRUCTIONS: [(&'static str, fn(&mut CPU, &mut MMU) -> u8); 256] = [
    ("NOP", nop),                     // 0x01
    ("LD BC, NN", ld_bc_nn),          // 0x02
    ("LD (BC), A", ld_bc_a),          // 0x02
    ("INC BC", inc_bc),               // 0x03
    ("INC B", inc_b),                 // 0x04
    ("DEC B", dec_b),                 // 0x05
    ("LD B, N", ld_b_n),              // 0x06
    ("RLCA", rlca),                   // 0x07
    ("LD (NN), SP", ld_nn_sp),        // 0x08
    ("ADD HL, BC", add_hl_bc),        // 0x09
    ("LD A, (BC)", ld_a_bc),          // 0x0a
    ("DEC BC", dec_bc),               // 0x0b
    ("INC C", inc_c),                 // 0x0c
    ("DEC C", dec_c),                 // 0x0d
    ("LD C, N", ld_c_n),              // 0x0e
    ("RRCA", rrca),                   // 0x0f
    ("STOP", unimplemented),          // 0x10
    ("LD DE, NN", ld_de_nn),          // 0x11
    ("LD (DE), A", ld_de_a),          // 0x12
    ("INC DE", inc_de),               // 0x13
    ("INC D", inc_d),                 // 0x14
    ("DEC D", dec_d),                 // 0x15
    ("LD D, N", ld_d_n),              // 0x16
    ("RLA", rla),                     // 0x17
    ("JR N", jr_n),                   // 0x18
    ("ADD HL, DE", add_hl_de),        // 0x19
    ("LD A, (DE)", ld_a_de),          // 0x1a
    ("DEC DE", dec_de),               // 0x1b
    ("INC E", inc_e),                 // 0x1c
    ("DEC E", dec_e),                 // 0x1d
    ("LD E, N", ld_e_n),              // 0x1e
    ("RRA", rra),                     // 0x1f
    ("JR NZ, N", jr_nz),              // 0x20
    ("LD HL, NN", ld_hl_nn),          // 0x21
    ("LDI (HL), A", ldi_hl_a),        // 0x22
    ("INC HL", inc_hl),               // 0x23
    ("INC H", inc_h),                 // 0x24
    ("DEC H", dec_h),                 // 0x25
    ("LD H, N", ld_h_n),              // 0x26
    ("DAA", daa),                     // 0x27
    ("JR Z, N", jr_z),                // 0x28
    ("ADD HL, HL", add_hl_hl),        // 0x29
    ("LDI A, (HL)", ldi_a_hl),        // 0x2a
    ("DEC HL", dec_hl),               // 0x2b
    ("INC L", inc_l),                 // 0x2c
    ("DEC L", dec_l),                 // 0x2d
    ("LD L, N", ld_l_n),              // 0x2e
    ("CPL", cpl),                     // 0x2f
    ("JR NC, N", jr_nc),              // 0x30
    ("LD SP, NN", ld_sp_nn),          // 0x31
    ("LDD (HL), A", ldd_hl_a),        // 0x32
    ("INC SP", inc_sp),               // 0x33
    ("INC (HL)", inc_at_hl),          // 0x34
    ("DEC (HL)", dec_at_hl),          // 0x35
    ("LD (HL), N", ld_hl_nn),         // 0x36
    ("SCF", scf),                     // 0x37
    ("JR C, N", jr_c),                // 0x38
    ("ADD HL, SP", add_hl_sp),        // 0x39
    ("LDD A, (HL)", ldd_a_hl),        // 0x3a
    ("DEC SP", dec_sp),               // 0x3b
    ("INC A", inc_a),                 // 0x3c
    ("DEC A", dec_a),                 // 0x3d
    ("LD A, N", ld_a_n),              // 0x3e
    ("CCF", ccf),                     // 0x3f
    ("LD B, B", ld_b_b),              // 0x40
    ("LD B, C", ld_b_c),              // 0x41
    ("LD B, D", ld_b_d),              // 0x42
    ("LD B, E", ld_b_e),              // 0x43
    ("LD B, H", ld_b_h),              // 0x44
    ("LD B, L", ld_b_l),              // 0x45
    ("LD B, (HL)", ld_b_hl),          // 0x46
    ("LD B, A", ld_b_a),              // 0x47
    ("LD C, B", ld_c_b),              // 0x48
    ("LD C, C", ld_c_c),              // 0x49
    ("LD C, D", ld_c_d),              // 0x4a
    ("LD C, E", ld_c_e),              // 0x4b
    ("LD C, H", ld_c_h),              // 0x4c
    ("LD C, L", ld_c_l),              // 0x4d
    ("LD C, (HL)", ld_c_hl),          // 0x4e
    ("LD C, A", ld_c_a),              // 0x4f
    ("LD D, B", ld_d_b),              // 0x50
    ("LD D, C", ld_d_c),              // 0x51
    ("LD D, D", ld_d_d),              // 0x52
    ("LD D, E", ld_d_e),              // 0x53
    ("LD D, H", ld_d_h),              // 0x54
    ("LD D, L", ld_d_l),              // 0x55
    ("LD D, (HL)", ld_d_hl),          // 0x56
    ("LD D, A", ld_d_a),              // 0x57
    ("LD E, B", ld_e_b),              // 0x58
    ("LD E, C", ld_e_c),              // 0x59
    ("LD E, D", ld_e_d),              // 0x5a
    ("LD E, E", ld_e_e),              // 0x5b
    ("LD E, H", ld_e_h),              // 0x5c
    ("LD E, L", ld_e_l),              // 0x5d
    ("LD E, (HL)", ld_e_hl),          // 0x5e
    ("LD E, A", ld_e_a),              // 0x5f
    ("LD H, B", ld_h_b),              // 0x60
    ("LD H, C", ld_h_c),              // 0x61
    ("LD H, D", ld_h_d),              // 0x62
    ("LD H, E", ld_h_e),              // 0x63
    ("LD H, H", ld_h_h),              // 0x64
    ("LD H, L", ld_h_l),              // 0x65
    ("LD H, (HL)", ld_h_hl),          // 0x66
    ("LD H, A", ld_h_a),              // 0x67
    ("LD L, B", ld_l_b),              // 0x68
    ("LD L, C", ld_l_c),              // 0x69
    ("LD L, D", ld_l_d),              // 0x6a
    ("LD L, E", ld_l_e),              // 0x6b
    ("LD L, H", ld_l_h),              // 0x6c
    ("LD L, L", ld_l_l),              // 0x6d
    ("LD L, (HL)", ld_l_hl),          // 0x6e
    ("LD L, A", ld_l_a),              // 0x6f
    ("LD (HL), B", ld_hl_b),          // 0x70
    ("LD (HL), C", ld_hl_c),          // 0x71
    ("LD (HL), D", ld_hl_d),          // 0x72
    ("LD (HL), E", ld_hl_e),          // 0x73
    ("LD (HL), H", ld_hl_h),          // 0x74
    ("LD (HL), L", ld_hl_l),          // 0x75
    ("HALT", halt),                   // 0x76
    ("LD (HL), A", ld_hl_a),          // 0x77
    ("LD A, B", ld_a_b),              // 0x78
    ("LD A, C", ld_a_c),              // 0x79
    ("LD A, D", ld_a_d),              // 0x7a
    ("LD A, E", ld_a_e),              // 0x7b
    ("LD A, H", ld_a_h),              // 0x7c
    ("LD A, L", ld_a_l),              // 0x7d
    ("LD A, (HL)", ld_a_hl),          // 0x7e
    ("LD A, A", ld_a_a),              // 0x7f
    ("ADD A, B", add_a_b),            // 0x80
    ("ADD A, C", add_a_c),            // 0x81
    ("ADD A, D", add_a_d),            // 0x82
    ("ADD A, E", add_a_e),            // 0x83
    ("ADD A, H", add_a_h),            // 0x84
    ("ADD A, L", add_a_l),            // 0x85
    ("ADD A, (HL)", add_a_hl),        // 0x86
    ("ADD A", add_a_a),               // 0x87
    ("ADC B", adc_a_b),               // 0x88
    ("ADC C", adc_a_c),               // 0x89
    ("ADC D", adc_a_d),               // 0x8a
    ("ADC E", adc_a_e),               // 0x8b
    ("ADC H", adc_a_h),               // 0x8c
    ("ADC L", adc_a_l),               // 0x8d
    ("ADC (HL)", adc_a_hl),           // 0x8e
    ("ADC A", adc_a_a),               // 0x8f
    ("SUB B", sub_a_b),               // 0x90
    ("SUB C", sub_a_c),               // 0x91
    ("SUB D", sub_a_d),               // 0x92
    ("SUB E", sub_a_e),               // 0x93
    ("SUB H", sub_a_h),               // 0x94
    ("SUB L", sub_a_l),               // 0x95
    ("SUB (HL)", sub_a_hl),           // 0x96
    ("SUB A", sub_a_a),               // 0x97
    ("SBC B", sbc_a_b),               // 0x98
    ("SBC C", sbc_a_c),               // 0x99
    ("SBC D", sbc_a_d),               // 0x9a
    ("SBC E", sbc_a_e),               // 0x9b
    ("SBC H", sbc_a_h),               // 0x9c
    ("SBC L", sbc_a_l),               // 0x9d
    ("SBC (HL)", sbc_a_hl),           // 0x9e
    ("SBC A", sbc_a_a),               // 0x9f
    ("AND B", and_a_b),               // 0xa0
    ("AND C", and_a_c),               // 0xa1
    ("AND D", and_a_d),               // 0xa2
    ("AND E", and_a_e),               // 0xa3
    ("AND H", and_a_h),               // 0xa4
    ("AND L", and_a_l),               // 0xa5
    ("AND (HL)", and_a_hl),           // 0xa6
    ("AND A", and_a_a),               // 0xa7
    ("XOR B", xor_a_b),               // 0xa8
    ("XOR C", xor_a_c),               // 0xa9
    ("XOR D", xor_a_d),               // 0xaa
    ("XOR E", xor_a_e),               // 0xab
    ("XOR H", xor_a_h),               // 0xac
    ("XOR L", xor_a_l),               // 0xad
    ("XOR (HL)", xor_a_hl),           // 0xae
    ("XOR A", xor_a_a),               // 0xaf
    ("OR B", or_a_b),                 // 0xb0
    ("OR C", or_a_c),                 // 0xb1
    ("OR D", or_a_d),                 // 0xb2
    ("OR E", or_a_e),                 // 0xb3
    ("OR H", or_a_h),                 // 0xb4
    ("OR L", or_a_l),                 // 0xb5
    ("OR (HL)", or_a_hl),             // 0xb6
    ("OR A", or_a_a),                 // 0xb7
    ("CP B", cp_a_b),                 // 0xb8
    ("CP C", cp_a_c),                 // 0xb9
    ("CP D", cp_a_d),                 // 0xba
    ("CP E", cp_a_e),                 // 0xbb
    ("CP H", cp_a_h),                 // 0xbc
    ("CP L", cp_a_l),                 // 0xbd
    ("CP (HL)", cp_a_hl),             // 0xbe
    ("CP A", cp_a_a),                 // 0xbf
    ("RET NZ", ret_nz),               // 0xc0
    ("POP BC", pop_bc),               // 0xc1
    ("JP N6, NN", jp_nz),             // 0xc2
    ("JP NN", jp_n),                  // 0xc3
    ("CALL NZ, NN", call_nz),         // 0xc4
    ("PUSH BC", push_bc),             // 0xc5
    ("ADD A, N", add_a_n),            // 0xc6
    ("RST 0x00", rst_0x00),           // 0xc7
    ("RET Z", ret_z),                 // 0xc8
    ("RET", ret_n),                   // 0xc9
    ("JP Z, NN", jp_z),               // 0xca
    ("CB N", execute_cb),             // 0xcb
    ("CALL Z, NN", call_z),           // 0xcc
    ("CALL NN", call_n),              // 0xcd
    ("ADC N", adc_a_n),               // 0xce
    ("RST 0x08", rst_0x08),           // 0xcf
    ("RET NC", ret_nc),               // 0xd0
    ("POP DE", pop_de),               // 0xd1
    ("JP NC, NN", jp_nc),             // 0xd2
    ("unimplemented", unimplemented), // 0xd3
    ("CALL NC, NN", call_nc),         // 0xd4
    ("PUSH DE", push_de),             // 0xd5
    ("SUB N", sub_a_n),               // 0xd6
    ("RST 0x10", rst_0x10),           // 0xd7
    ("RET C", ret_c),                 // 0xd8
    ("RETI", ret_i),                  // 0xd9
    ("JP C, NN", jp_c),               // 0xda
    ("unimplemented", unimplemented), // 0xdb
    ("CALL C, NN", call_c),           // 0xdc
    ("unimplemented", unimplemented), // 0xdd
    ("SBC N", sbc_a_n),               // 0xde
    ("RST 0x18", rst_0x18),           // 0xdf
    ("LD (0xFF00 + N), A", ld_n_a),   // 0xe0
    ("POP HL", pop_hl),               // 0xe1
    ("LD (0xFF00 + C), A", ld_c_a),   // 0xe2
    ("unimplemented", unimplemented), // 0xe3
    ("unimplemented", unimplemented), // 0xe4
    ("PUSH HL", push_hl),             // 0xe5
    ("AND N", and_a_n),               // 0xe6
    ("RST 0x20", rst_0x20),           // 0xe7
    ("ADD SP,N", add_sp_n),           // 0xe8
    ("JP HL", jp_hl),                 // 0xe9
    ("LD (NN), A", ld_nn_a),          // 0xea
    ("unimplemented", unimplemented), // 0xeb
    ("unimplemented", unimplemented), // 0xec
    ("unimplemented", unimplemented), // 0xed
    ("XOR N", xor_a_n),               // 0xee
    ("RST 0x28", rst_0x28),           // 0xef
    ("LD A, (0xFF00 + N)", ld_a_n),   // 0xf0
    ("POP AF", pop_af),               // 0xf1
    ("LD A, (0xFF00 + C)", ld_a_c),   // 0xf2
    ("DI", di),                       // 0xf3
    ("unimplemented", unimplemented), // 0xf4
    ("PUSH AF", push_af),             // 0xf5
    ("OR N", or_a_n),                 // 0xf6
    ("RST 0x30", rst_0x30),           // 0xf7
    ("LD HL, SP+N", ld_hl_sp_n),      // 0xf8
    ("LD SP, HL", ld_sp_hl),          // 0xf9
    ("LD A, (NN)", ld_a_nn),          // 0xfa
    ("EI", ei),                       // 0xfb
    ("unimplemented", unimplemented), // 0xfc
    ("unimplemented", unimplemented), // 0xfd
    ("CP N", cp_a_n),                 // 0xfe
    ("RST 0x38", rst_0x38),           // 0xff
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

fn ld_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.a = cpu.fetch_byte(mmu);
    8
}

fn ld_b_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.b = cpu.fetch_byte(mmu);
    8
}

fn ld_c_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.c = cpu.fetch_byte(mmu);
    8
}

fn ld_d_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.d = cpu.fetch_byte(mmu);
    8
}

fn ld_e_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.e = cpu.fetch_byte(mmu);
    8
}

fn ld_h_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.h = cpu.fetch_byte(mmu);
    8
}

fn ld_l_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.l = cpu.fetch_byte(mmu);
    8
}

fn ld_a_nn(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let addr = cpu.fetch_word(mmu);
    cpu.regs.a = mmu.read_byte(addr);
    16
}

fn ld_bc_nn(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_word(mmu);
    cpu.regs.set_bc(n);
    12
}

fn ld_a_a(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn ld_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.a = cpu.regs.b;
    4
}

fn ld_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.a = cpu.regs.c;
    4
}

fn ld_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.a = cpu.regs.d;
    4
}

fn ld_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.a = cpu.regs.e;
    4
}

fn ld_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.a = cpu.regs.h;
    4
}

fn ld_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.a = cpu.regs.l;
    4
}

fn ld_b_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.b = cpu.regs.a;
    4
}

fn ld_b_b(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn ld_b_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.b = cpu.regs.c;
    4
}

fn ld_b_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.b = cpu.regs.d;
    4
}

fn ld_b_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.b = cpu.regs.e;
    4
}

fn ld_b_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.b = cpu.regs.h;
    4
}

fn ld_b_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.b = cpu.regs.l;
    4
}

fn ld_c_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.c = cpu.regs.a;
    4
}

fn ld_c_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.c = cpu.regs.b;
    4
}

fn ld_c_c(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn ld_c_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.c = cpu.regs.d;
    4
}

fn ld_c_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.c = cpu.regs.e;
    4
}

fn ld_c_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.c = cpu.regs.h;
    4
}

fn ld_c_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.c = cpu.regs.l;
    4
}

fn ld_d_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.d = cpu.regs.a;
    4
}

fn ld_d_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.d = cpu.regs.b;
    4
}

fn ld_d_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.d = cpu.regs.c;
    4
}

fn ld_d_d(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn ld_d_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.d = cpu.regs.e;
    4
}

fn ld_d_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.d = cpu.regs.h;
    4
}

fn ld_d_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.d = cpu.regs.l;
    4
}

fn ld_e_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.e = cpu.regs.a;
    4
}

fn ld_e_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.e = cpu.regs.b;
    4
}

fn ld_e_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.e = cpu.regs.c;
    4
}

fn ld_e_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.e = cpu.regs.d;
    4
}

fn ld_e_e(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn ld_e_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.e = cpu.regs.h;
    4
}

fn ld_e_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.e = cpu.regs.l;
    4
}

fn ld_h_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.h = cpu.regs.a;
    4
}

fn ld_h_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.h = cpu.regs.b;
    4
}

fn ld_h_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.h = cpu.regs.c;
    4
}

fn ld_h_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.h = cpu.regs.d;
    4
}

fn ld_h_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.h = cpu.regs.e;
    4
}

fn ld_h_h(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn ld_h_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.h = cpu.regs.l;
    4
}

fn ld_l_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.l = cpu.regs.a;
    4
}

fn ld_l_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.l = cpu.regs.b;
    4
}

fn ld_l_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.l = cpu.regs.c;
    4
}

fn ld_l_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.l = cpu.regs.d;
    4
}

fn ld_l_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.l = cpu.regs.e;
    4
}

fn ld_l_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.l = cpu.regs.h;
    4
}

fn ld_l_l(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn ld_a_bc(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.a = mmu.read_byte(cpu.regs.get_bc());
    8
}

fn ld_a_de(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.a = mmu.read_byte(cpu.regs.get_de());
    8
}

fn ld_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.a = mmu.read_byte(cpu.regs.get_hl());
    8
}

fn ld_b_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.b = mmu.read_byte(cpu.regs.get_hl());
    8
}

fn ld_c_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.c = mmu.read_byte(cpu.regs.get_hl());
    8
}

fn ld_d_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.d = mmu.read_byte(cpu.regs.get_hl());
    8
}

fn ld_e_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.e = mmu.read_byte(cpu.regs.get_hl());
    8
}

fn ld_h_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.h = mmu.read_byte(cpu.regs.get_hl());
    8
}

fn ld_l_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.l = mmu.read_byte(cpu.regs.get_hl());
    8
}

fn ld_de_nn(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_word(mmu);
    cpu.regs.set_de(n);
    12
}

fn ld_hl_nn(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_word(mmu);
    cpu.regs.set_hl(n);
    12
}

fn ld_sp_nn(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.sp = cpu.fetch_word(mmu);
    12
}

fn ld_bc_a(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_bc(), cpu.regs.a);
    8
}

fn ld_de_a(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_de(), cpu.regs.a);
    8
}

fn ld_hl_a(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_hl(), cpu.regs.a);
    8
}

fn ld_hl_b(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_hl(), cpu.regs.b);
    8
}

fn ld_hl_c(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_hl(), cpu.regs.c);
    8
}

fn ld_hl_d(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_hl(), cpu.regs.d);
    8
}

fn ld_hl_e(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_hl(), cpu.regs.e);
    8
}

fn ld_hl_h(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_hl(), cpu.regs.h);
    8
}

fn ld_hl_l(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_hl(), cpu.regs.l);
    8
}

fn ld_n_a(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let addr = 0xff00 + (cpu.fetch_byte(mmu) as u16);
    mmu.write_byte(addr, cpu.regs.a);
    12
}

fn ld_nn_a(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let addr = cpu.fetch_word(mmu);
    mmu.write_byte(addr, cpu.regs.a);
    16
}

fn ld_nn_sp(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_word(mmu);
    mmu.write_word(n, cpu.regs.sp);
    20
}

fn ld_hl_sp_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    let sp_lo = (cpu.regs.sp & 0xff) as u8;
    let (_, f) = add(n, sp_lo);
    cpu.regs.f = f & !ZERO & !SUB;
    cpu.regs.set_hl(cpu.regs.sp + ((n as i8) as u16));
    12
}

fn ld_sp_hl(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.sp = cpu.regs.get_hl();
    8
}

fn ldi_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    ld_a_hl(cpu, mmu);
    inc_hl(cpu, mmu);
    8
}

fn ldi_hl_a(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    ld_hl_a(cpu, mmu);
    inc_hl(cpu, mmu);
    8
}

fn ldd_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    ld_a_hl(cpu, mmu);
    dec_hl(cpu, mmu);
    8
}

fn ldd_hl_a(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    ld_hl_a(cpu, mmu);
    dec_hl(cpu, mmu);
    8
}

fn inc_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (a, f) = inc_r(cpu.regs.a, cpu.regs.f);
    cpu.regs.a = a;
    cpu.regs.f = f;
    4
}

fn inc_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (b, f) = inc_r(cpu.regs.b, cpu.regs.f);
    cpu.regs.b = b;
    cpu.regs.f = f;
    4
}

fn inc_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (c, f) = inc_r(cpu.regs.c, cpu.regs.f);
    cpu.regs.c = c;
    cpu.regs.f = f;
    4
}

fn inc_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (d, f) = inc_r(cpu.regs.d, cpu.regs.f);
    cpu.regs.d = d;
    cpu.regs.f = f;
    4
}

fn inc_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (e, f) = inc_r(cpu.regs.e, cpu.regs.f);
    cpu.regs.e = e;
    cpu.regs.f = f;
    4
}

fn inc_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (h, f) = inc_r(cpu.regs.h, cpu.regs.f);
    cpu.regs.h = h;
    cpu.regs.f = f;
    4
}

fn inc_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (l, f) = inc_r(cpu.regs.l, cpu.regs.f);
    cpu.regs.l = l;
    cpu.regs.f = f;
    4
}

fn dec_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (a, f) = dec_r(cpu.regs.a, cpu.regs.f);
    cpu.regs.a = a;
    cpu.regs.f = f;
    4
}

fn dec_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (b, f) = dec_r(cpu.regs.b, cpu.regs.f);
    cpu.regs.b = b;
    cpu.regs.f = f;
    4
}

fn dec_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (c, f) = dec_r(cpu.regs.c, cpu.regs.f);
    cpu.regs.a = c;
    cpu.regs.f = f;
    4
}

fn dec_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (d, f) = dec_r(cpu.regs.d, cpu.regs.f);
    cpu.regs.a = d;
    cpu.regs.f = f;
    4
}

fn dec_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (e, f) = dec_r(cpu.regs.e, cpu.regs.f);
    cpu.regs.a = e;
    cpu.regs.f = f;
    4
}

fn dec_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (h, f) = dec_r(cpu.regs.h, cpu.regs.f);
    cpu.regs.a = h;
    cpu.regs.f = f;
    4
}

fn dec_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let (l, f) = dec_r(cpu.regs.l, cpu.regs.f);
    cpu.regs.a = l;
    cpu.regs.f = f;
    4
}

fn inc_bc(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.set_bc(cpu.regs.get_bc() + 1);
    8
}

fn inc_de(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.set_de(cpu.regs.get_de() + 1);
    8
}

fn inc_hl(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.set_hl(cpu.regs.get_hl() + 1);
    8
}

fn inc_sp(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.sp += 1;
    8
}

fn dec_bc(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.set_bc(cpu.regs.get_bc() - 1);
    8
}

fn dec_de(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.set_de(cpu.regs.get_de() - 1);
    8
}

fn dec_hl(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.set_hl(cpu.regs.get_hl() - 1);
    8
}

fn dec_sp(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.sp -= 1;
    8
}

fn inc_at_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
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

fn dec_at_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
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

fn add_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_a(cpu, cpu.regs.a)
}

fn add_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_a(cpu, cpu.regs.b)
}

fn add_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_a(cpu, cpu.regs.c)
}

fn add_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_a(cpu, cpu.regs.d)
}

fn add_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_a(cpu, cpu.regs.e)
}

fn add_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_a(cpu, cpu.regs.h)
}

fn add_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_a(cpu, cpu.regs.l)
}

fn add_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + add_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

fn add_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + add_a(cpu, n)
}

fn add_a(cpu: &mut CPU, operand: u8) -> u8 {
    let (a, f) = add(cpu.regs.a, operand);
    cpu.regs.a = a;
    cpu.regs.f = f;
    4
}

fn add_hl_bc(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_hl(cpu, cpu.regs.get_bc())
}

fn add_hl_de(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_hl(cpu, cpu.regs.get_de())
}

fn add_hl_hl(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_hl(cpu, cpu.regs.get_hl())
}

fn add_hl_sp(cpu: &mut CPU, _: &mut MMU) -> u8 {
    add_hl(cpu, cpu.regs.sp)
}

fn add_hl(cpu: &mut CPU, operand: u16) -> u8 {
    let (hl, f) = add_16(cpu.regs.get_hl(), operand, cpu.regs.f);
    cpu.regs.set_hl(hl);
    cpu.regs.f = f;
    8
}

fn add_sp_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    let sp_lo = (cpu.regs.sp & 0xff) as u8;
    let (_, f) = add(n, sp_lo);
    cpu.regs.f = f & !ZERO & !SUB;
    cpu.regs.sp += (n as i8) as u16;
    16
}

fn adc_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    adc_a(cpu, cpu.regs.a)
}

fn adc_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    adc_a(cpu, cpu.regs.b)
}

fn adc_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    adc_a(cpu, cpu.regs.c)
}

fn adc_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    adc_a(cpu, cpu.regs.d)
}

fn adc_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    adc_a(cpu, cpu.regs.e)
}

fn adc_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    adc_a(cpu, cpu.regs.h)
}

fn adc_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    adc_a(cpu, cpu.regs.l)
}

fn adc_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + adc_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

fn adc_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + adc_a(cpu, n)
}

fn adc_a(cpu: &mut CPU, operand: u8) -> u8 {
    let (a, f) = addc(cpu.regs.a, operand, cpu.regs.f & CARRY == CARRY);
    cpu.regs.a = a;
    cpu.regs.f = f;
    4
}

fn sub_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sub_a(cpu, cpu.regs.a)
}

fn sub_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sub_a(cpu, cpu.regs.b)
}

fn sub_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sub_a(cpu, cpu.regs.c)
}

fn sub_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sub_a(cpu, cpu.regs.d)
}

fn sub_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sub_a(cpu, cpu.regs.e)
}

fn sub_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sub_a(cpu, cpu.regs.h)
}

fn sub_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sub_a(cpu, cpu.regs.l)
}

fn sub_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + sub_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

fn sub_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + sub_a(cpu, n)
}

fn sub_a(cpu: &mut CPU, operand: u8) -> u8 {
    let (a, f) = sub(cpu.regs.a, operand);
    cpu.regs.a = a;
    cpu.regs.f = f;
    4
}

fn sbc_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sbc_a(cpu, cpu.regs.a)
}

fn sbc_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sbc_a(cpu, cpu.regs.b)
}

fn sbc_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sbc_a(cpu, cpu.regs.c)
}

fn sbc_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sbc_a(cpu, cpu.regs.d)
}

fn sbc_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sbc_a(cpu, cpu.regs.e)
}

fn sbc_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sbc_a(cpu, cpu.regs.h)
}

fn sbc_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    sbc_a(cpu, cpu.regs.l)
}

fn sbc_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + sbc_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

fn sbc_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + sbc_a(cpu, n)
}

fn sbc_a(cpu: &mut CPU, operand: u8) -> u8 {
    let (a, f) = subc(cpu.regs.a, operand, cpu.regs.f & CARRY == CARRY);
    cpu.regs.a = a;
    cpu.regs.f = f;
    4
}

fn and_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    and_a(cpu, cpu.regs.a)
}

fn and_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    and_a(cpu, cpu.regs.b)
}

fn and_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    and_a(cpu, cpu.regs.c)
}

fn and_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    and_a(cpu, cpu.regs.d)
}

fn and_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    and_a(cpu, cpu.regs.e)
}

fn and_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    and_a(cpu, cpu.regs.h)
}

fn and_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    and_a(cpu, cpu.regs.l)
}

fn and_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + and_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

fn and_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + and_a(cpu, n)
}

fn and_a(cpu: &mut CPU, operand: u8) -> u8 {
    cpu.regs.a &= operand;
    cpu.regs.f = HALF_CARRY | if cpu.regs.a == 0 { ZERO } else { 0 };
    4
}

fn xor_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    xor_a(cpu, cpu.regs.a)
}

fn xor_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    xor_a(cpu, cpu.regs.b)
}

fn xor_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    xor_a(cpu, cpu.regs.c)
}

fn xor_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    xor_a(cpu, cpu.regs.d)
}

fn xor_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    xor_a(cpu, cpu.regs.e)
}

fn xor_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    xor_a(cpu, cpu.regs.h)
}

fn xor_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    xor_a(cpu, cpu.regs.l)
}

fn xor_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + xor_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

fn xor_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + xor_a(cpu, n)
}

fn xor_a(cpu: &mut CPU, operand: u8) -> u8 {
    cpu.regs.a ^= operand;
    cpu.regs.f = if cpu.regs.a == 0 { ZERO } else { 0 };
    4
}

fn or_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    or_a(cpu, cpu.regs.a)
}

fn or_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    or_a(cpu, cpu.regs.b)
}

fn or_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    or_a(cpu, cpu.regs.c)
}

fn or_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    or_a(cpu, cpu.regs.d)
}

fn or_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    or_a(cpu, cpu.regs.e)
}

fn or_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    or_a(cpu, cpu.regs.h)
}

fn or_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    or_a(cpu, cpu.regs.l)
}

fn or_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + or_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

fn or_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + or_a(cpu, n)
}

fn or_a(cpu: &mut CPU, operand: u8) -> u8 {
    cpu.regs.a |= operand;
    cpu.regs.f = if cpu.regs.a == 0 { ZERO } else { 0 };
    4
}

fn cp_a_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cp_a(cpu, cpu.regs.a)
}

fn cp_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cp_a(cpu, cpu.regs.b)
}

fn cp_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cp_a(cpu, cpu.regs.c)
}

fn cp_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cp_a(cpu, cpu.regs.d)
}

fn cp_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cp_a(cpu, cpu.regs.e)
}

fn cp_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cp_a(cpu, cpu.regs.h)
}

fn cp_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cp_a(cpu, cpu.regs.l)
}

fn cp_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    4 + cp_a(cpu, mmu.read_byte(cpu.regs.get_hl()))
}

fn cp_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    4 + cp_a(cpu, n)
}

fn cp_a(cpu: &mut CPU, operand: u8) -> u8 {
    let (_, f) = sub(cpu.regs.a, operand);
    cpu.regs.f = f;
    4
}

fn rlca(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let bit_7 = cpu.regs.a >> 7;
    cpu.regs.a = (cpu.regs.a << 1) | bit_7;
    cpu.regs.f = if bit_7 == 1 { CARRY } else { 0 };
    4
}

fn rla(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let bit_7 = cpu.regs.a >> 7;
    cpu.regs.a <<= 1;
    cpu.regs.a |= if cpu.regs.f & CARRY == CARRY { 1 } else { 0 };
    cpu.regs.f = if bit_7 == 1 { CARRY } else { 0 };
    4
}

fn rrca(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let bit_0 = cpu.regs.a & 1;
    cpu.regs.a = (cpu.regs.a >> 1) | (bit_0 << 7);
    cpu.regs.f = if bit_0 == 1 { CARRY } else { 0 };
    4
}

fn rra(cpu: &mut CPU, _: &mut MMU) -> u8 {
    let old_carry = (cpu.regs.f & CARRY) << 3;
    cpu.regs.f = if cpu.regs.a & 1 == 1 { CARRY } else { 0 };
    cpu.regs.a = (cpu.regs.a >> 1) | old_carry;
    4
}

fn cpl(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.a = !cpu.regs.a;
    cpu.regs.f |= SUB | HALF_CARRY;
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
    12
}

fn pop_bc(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.set_bc(mmu.read_word(cpu.regs.sp));
    12
}

fn pop_de(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.set_de(mmu.read_word(cpu.regs.sp));
    12
}

fn pop_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.set_hl(mmu.read_word(cpu.regs.sp));
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

fn inc_r(reg: u8, f: u8) -> (u8, u8) {
    let (r, mut new_f) = add(reg, 1);
    new_f = if f & CARRY == CARRY {
        new_f | CARRY
    } else {
        new_f & !CARRY
    };
    (r, new_f)
}

fn dec_r(reg: u8, f: u8) -> (u8, u8) {
    let (r, mut new_f) = sub(reg, 1);
    new_f = if f & CARRY == CARRY {
        new_f | CARRY
    } else {
        new_f & !CARRY
    };
    (r, new_f)
}

fn add_16(op1: u16, op2: u16, f: u8) -> (u16, u8) {
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

fn add(op1: u8, op2: u8) -> (u8, u8) {
    add_impl(op1, op2, false, false)
}

fn addc(op1: u8, op2: u8, carry_on: bool) -> (u8, u8) {
    add_impl(op1, op2, true, carry_on)
}

fn add_impl(op1: u8, op2: u8, use_carry: bool, carry_on: bool) -> (u8, u8) {
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

fn sub(op1: u8, op2: u8) -> (u8, u8) {
    sub_impl(op1, op2, false, false)
}

fn subc(op1: u8, op2: u8, carry_on: bool) -> (u8, u8) {
    sub_impl(op1, op2, true, carry_on)
}

fn sub_impl(op1: u8, op2: u8, use_borrow: bool, carry_on: bool) -> (u8, u8) {
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
