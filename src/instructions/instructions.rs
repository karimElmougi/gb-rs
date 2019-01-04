const CARRY: u8 = 0b0001_0000;
const HALF_CARRY: u8 = 0b0010_0000;
const SUB: u8 = 0b0100_0000;
const ZERO: u8 = 0b1000_0000;

const INSTRUCTIONS: [(&'static str, fn(&mut GameBoy) -> u8); 128] = [
    ("NOP", GameBoy::nop),              // 0x01
    ("LD BC, NN", GameBoy::ld_bc_nn),   // 0x02
    ("LD (BC), A", GameBoy::ld_bc_a),   // 0x02
    ("INC BC", GameBoy::inc_bc),        // 0x03
    ("INC B", GameBoy::inc_b),          // 0x04
    ("DEC B", GameBoy::dec_b),          // 0x05
    ("LD B, N", GameBoy::ld_b_n),       // 0x06
    ("RLCA", GameBoy::rlca),            // 0x07
    ("LD (NN), SP", GameBoy::ld_nn_sp), // 0x08
    ("ADD HL, BC", GameBoy::add_hl_bc), // 0x09
    ("LD A, (BC)", GameBoy::ld_a_bc),   // 0x0a
    ("DEC BC", GameBoy::dec_bc),        // 0x0b
    ("INC C", GameBoy::inc_c),          // 0x0c
    ("DEC C", GameBoy::dec_c),          // 0x0d
    ("LD C, N", GameBoy::ld_c_n),       // 0x0e
    ("RRCA", GameBoy::rrca),            // 0x0f
    ("STOP", GameBoy::unimplemented),   // 0x10
    ("LD DE, NN", GameBoy::ld_de_nn),   // 0x11
    ("LD (DE), A", GameBoy::ld_de_a),   // 0x12
    ("INC DE", GameBoy::inc_de),        // 0x13
    ("INC D", GameBoy::inc_d),          // 0x14
    ("DEC D", GameBoy::dec_d),          // 0x15
    ("LD D, N", GameBoy::ld_d_n),       // 0x16
    ("RLA", GameBoy::rla),              // 0x17
    ("JR N", GameBoy::jr_n),            // 0x18
    ("ADD HL, DE", GameBoy::add_hl_de), // 0x19
    ("LD A, (DE)", GameBoy::ld_a_de),   // 0x1a
    ("DEC DE", GameBoy::dec_de),        // 0x1b
    ("INC E", GameBoy::inc_e),          // 0x1c
    ("DEC E", GameBoy::dec_e),          // 0x1d
    ("LD E, N", GameBoy::ld_e_n),       // 0x1e
    ("RRA", GameBoy::rra),              // 0x1f
    ("JR NZ, N", GameBoy::jr_nz),       // 0x20
    ("LD HL, NN", GameBoy::ld_hl_nn),   // 0x21
    ("LDI (HL), A", GameBoy::ldi_hl_a), // 0x22
    ("INC HL", GameBoy::inc_hl),        // 0x23
    ("INC H", GameBoy::inc_h),          // 0x24
    ("DEC H", GameBoy::dec_h),          // 0x25
    ("LD H, N", GameBoy::ld_h_n),       // 0x26
    ("DAA", GameBoy::daa),              // 0x27
    ("JR Z, N", GameBoy::jr_z),         // 0x28
    ("ADD HL, HL", GameBoy::add_hl_hl), // 0x29
    ("LDI A, (HL)", GameBoy::ldi_a_hl), // 0x2a
    ("DEC HL", GameBoy::dec_hl),        // 0x2b
    ("INC L", GameBoy::inc_l),          // 0x2c
    ("DEC L", GameBoy::dec_l),          // 0x2d
    ("LD L, N", GameBoy::ld_l_n),       // 0x2e
    ("CPL", GameBoy::cpl),              // 0x2f
    ("JR NC, N", GameBoy::jr_nc),       // 0x30
    ("LD SP, NN", GameBoy::ld_sp_nn),   // 0x31
    ("LDD (HL), A", GameBoy::ldd_hl_a), // 0x32
    ("INC SP", GameBoy::inc_sp),        // 0x33
    ("INC (HL)", GameBoy::inc_at_hl),   // 0x34
    ("DEC (HL)", GameBoy::dec_at_hl),   // 0x35
    ("LD (HL), N", GameBoy::ld_hl_nn),  // 0x36
    ("SCF", GameBoy::scf),              // 0x37
    ("JR C, N", GameBoy::jr_c),         // 0x38
    ("ADD HL, SP", GameBoy::add_hl_sp), // 0x39
    ("LDD A, (HL)", GameBoy::ldd_a_hl), // 0x3a
    ("DEC SP", GameBoy::dec_sp),        // 0x3b
    ("INC A", GameBoy::inc_a),          // 0x3c
    ("DEC A", GameBoy::dec_a),          // 0x3d
    ("LD A, N", GameBoy::ld_a_n),       // 0x3e
    ("CCF", GameBoy::ccf),              // 0x3f
    ("LD B, B", GameBoy::ld_b_b),       // 0x40
    ("LD B, C", GameBoy::ld_b_c),       // 0x41
    ("LD B, D", GameBoy::ld_b_d),       // 0x42
    ("LD B, E", GameBoy::ld_b_e),       // 0x43
    ("LD B, H", GameBoy::ld_b_h),       // 0x44
    ("LD B, L", GameBoy::ld_b_l),       // 0x45
    ("LD B, (HL)", GameBoy::ld_b_hl),   // 0x46
    ("LD B, A", GameBoy::ld_b_a),       // 0x47
    ("LD C, B", GameBoy::ld_c_b),       // 0x48
    ("LD C, C", GameBoy::ld_c_c),       // 0x49
    ("LD C, D", GameBoy::ld_c_d),       // 0x4a
    ("LD C, E", GameBoy::ld_c_e),       // 0x4b
    ("LD C, H", GameBoy::ld_c_h),       // 0x4c
    ("LD C, L", GameBoy::ld_c_l),       // 0x4d
    ("LD C, (HL)", GameBoy::ld_c_hl),   // 0x4e
    ("LD C, A", GameBoy::ld_c_a),       // 0x4f
    ("LD D, B", GameBoy::ld_d_b),       // 0x50
    ("LD D, C", GameBoy::ld_d_c),       // 0x51
    ("LD D, D", GameBoy::ld_d_d),       // 0x52
    ("LD D, E", GameBoy::ld_d_e),       // 0x53
    ("LD D, H", GameBoy::ld_d_h),       // 0x54
    ("LD D, L", GameBoy::ld_d_l),       // 0x55
    ("LD D, (HL)", GameBoy::ld_d_hl),   // 0x56
    ("LD D, A", GameBoy::ld_d_a),       // 0x57
    ("LD E, B", GameBoy::ld_e_b),       // 0x58
    ("LD E, C", GameBoy::ld_e_c),       // 0x59
    ("LD E, D", GameBoy::ld_e_d),       // 0x5a
    ("LD E, E", GameBoy::ld_e_e),       // 0x5b
    ("LD E, H", GameBoy::ld_e_h),       // 0x5c
    ("LD E, L", GameBoy::ld_e_l),       // 0x5d
    ("LD E, (HL)", GameBoy::ld_e_hl),   // 0x5e
    ("LD E, A", GameBoy::ld_e_a),       // 0x5f
    ("LD H, B", GameBoy::ld_h_b),       // 0x60
    ("LD H, C", GameBoy::ld_h_c),       // 0x61
    ("LD H, D", GameBoy::ld_h_d),       // 0x62
    ("LD H, E", GameBoy::ld_h_e),       // 0x63
    ("LD H, H", GameBoy::ld_h_h),       // 0x64
    ("LD H, L", GameBoy::ld_h_l),       // 0x65
    ("LD H, (HL)", GameBoy::ld_h_hl),   // 0x66
    ("LD H, A", GameBoy::ld_h_a),       // 0x67
    ("LD L, B", GameBoy::ld_l_b),       // 0x68
    ("LD L, C", GameBoy::ld_l_c),       // 0x69
    ("LD L, D", GameBoy::ld_l_d),       // 0x6a
    ("LD L, E", GameBoy::ld_l_e),       // 0x6b
    ("LD L, H", GameBoy::ld_l_h),       // 0x6c
    ("LD L, L", GameBoy::ld_l_l),       // 0x6d
    ("LD L, (HL)", GameBoy::ld_l_hl),   // 0x6e
    ("LD L, A", GameBoy::ld_l_a),       // 0x6f
    ("LD (HL), B", GameBoy::ld_hl_b),   // 0x70
    ("LD (HL), C", GameBoy::ld_hl_c),   // 0x71
    ("LD (HL), D", GameBoy::ld_hl_d),   // 0x72
    ("LD (HL), E", GameBoy::ld_hl_e),   // 0x73
    ("LD (HL), H", GameBoy::ld_hl_h),   // 0x74
    ("LD (HL), L", GameBoy::ld_hl_l),   // 0x75
    ("HALT", GameBoy::halt),            // 0x76
    ("LD (HL), A", GameBoy::ld_hl_a),   // 0x77
    ("LD A, B", GameBoy::ld_a_b),       // 0x78
    ("LD A, C", GameBoy::ld_a_c),       // 0x79
    ("LD A, D", GameBoy::ld_a_d),       // 0x7a
    ("LD A, E", GameBoy::ld_a_e),       // 0x7b
    ("LD A, H", GameBoy::ld_a_h),       // 0x7c
    ("LD A, L", GameBoy::ld_a_l),       // 0x7d
    ("LD A, (HL)", GameBoy::ld_a_hl),   // 0x7e
    ("LD A, A", GameBoy::ld_a_a),       // 0x7f
    // ("ADD A, B", add_a_r, 4),          // 0x80
    // ("ADD A, C", add_a_r, 4),          // 0x81
    // ("ADD A, D", add_a_r, 4),          // 0x82
    // ("ADD A, E", add_a_r, 4),          // 0x83
    // ("ADD A, H", add_a_r, 4),          // 0x84
    // ("ADD A, L", add_a_r, 4),          // 0x85
    // ("ADD A, (HL)", add_a_hl, 8),        // 0x86
    // ("ADD A", add_a_r, 4),             // 0x87
    // ("ADC B", adc_a_r, 4),             // 0x88
    // ("ADC C", adc_a_r, 4),             // 0x89
    // ("ADC D", adc_a_r, 4),             // 0x8a
    // ("ADC E", adc_a_r, 4),             // 0x8b
    // ("ADC H", adc_a_r, 4),             // 0x8c
    // ("ADC L", adc_a_r, 4),             // 0x8d
    // ("ADC (HL)", adc_a_hl, 8),           // 0x8e
    // ("ADC A", adc_a_r, 4),             // 0x8f
    // ("SUB B", sub_a_r, 4),             // 0x90
    // ("SUB C", sub_a_r, 4),             // 0x91
    // ("SUB D", sub_a_r, 4),             // 0x92
    // ("SUB E", sub_a_r, 4),             // 0x93
    // ("SUB H", sub_a_r, 4),             // 0x94
    // ("SUB L", sub_a_r, 4),             // 0x95
    // ("SUB (HL)", sub_a_hl, 8),           // 0x96
    // ("SUB A", sub_a_r, 4),             // 0x97
    // ("SBC B", sbc_a_r, 4),             // 0x98
    // ("SBC C", sbc_a_r, 4),             // 0x99
    // ("SBC D", sbc_a_r, 4),             // 0x9a
    // ("SBC E", sbc_a_r, 4),             // 0x9b
    // ("SBC H", sbc_a_r, 4),             // 0x9c
    // ("SBC L", sbc_a_r, 4),             // 0x9d
    // ("SBC (HL)", sbc_a_hl, 8),           // 0x9e
    // ("SBC A", sbc_a_r, 4),             // 0x9f
    // ("AND B", and_a_r, 4),             // 0xa0
    // ("AND C", and_a_r, 4),             // 0xa1
    // ("AND D", and_a_r, 4),             // 0xa2
    // ("AND E", and_a_r, 4),             // 0xa3
    // ("AND H", and_a_r, 4),             // 0xa4
    // ("AND L", and_a_r, 4),             // 0xa5
    // ("AND (HL)", and_a_hl, 8),           // 0xa6
    // ("AND A", and_a_r, 4),             // 0xa7
    // ("XOR B", xor_a_r, 4),             // 0xa8
    // ("XOR C", xor_a_r, 4),             // 0xa9
    // ("XOR D", xor_a_r, 4),             // 0xaa
    // ("XOR E", xor_a_r, 4),             // 0xab
    // ("XOR H", xor_a_r, 4),             // 0xac
    // ("XOR L", xor_a_r, 4),             // 0xad
    // ("XOR (HL)", xor_a_hl, 8),           // 0xae
    // ("XOR A", xor_a_r, 4),             // 0xaf
    // ("OR B", or_a_r, 4),               // 0xb0
    // ("OR C", or_a_r, 4),               // 0xb1
    // ("OR D", or_a_r, 4),               // 0xb2
    // ("OR E", or_a_r, 4),               // 0xb3
    // ("OR H", or_a_r, 4),               // 0xb4
    // ("OR L", or_a_r, 4),               // 0xb5
    // ("OR (HL)", or_a_hl, 8),             // 0xb6
    // ("OR A", or_a_r, 4),               // 0xb7
    // ("CP B", cp_a_r, 4),               // 0xb8
    // ("CP C", cp_a_r, 4),               // 0xb9
    // ("CP D", cp_a_r, 4),               // 0xba
    // ("CP E", cp_a_r, 4),               // 0xbb
    // ("CP H", cp_a_r, 4),               // 0xbc
    // ("CP L", cp_a_r, 4),               // 0xbd
    // ("CP (HL)", cp_a_hl, 8),             // 0xbe
    // ("CP A", cp_a_r, 4),               // 0xbf
    // ("RET NZ", retnz, 8),                // 0xc0
    // ("POP BC", pop, 12),           // 0xc1
    // ("JP NZ, NN", 2, jpnz, 12),             // 0xc2
    // ("JP NN", 2, jpnn, 12),                 // 0xc3
    // ("CALL NZ, NN", 2, callnz, 12),         // 0xc4
    // ("PUSH BC", push, 16),         // 0xc5
    // ("ADD A, N", add_a_n, 8),            // 0xc6
    // ("RST 0x00", rst0x00, 32),           // 0xc7
    // ("RET Z", retz, 8),                  // 0xc8
    // ("RET", ret, 8),                     // 0xc9
    // ("JP Z, NN", 2, jpz, 12),               // 0xca
    // ("CB N", cb),                     // 0xcb
    // ("CALL Z, NN", 2, callz, 12),           // 0xcc
    // ("CALL NN", 2, call, 12),               // 0xcd
    // ("ADC N", adc_a_n, 8),               // 0xce
    // ("RST 0x08", rst0x08, 32),           // 0xcf
    // ("RET NC", retnc, 8),                // 0xd0
    // ("POP DE", pop, 12),           // 0xd1
    // ("JP NC, NN", 2, jpnc, 12),             // 0xd2
    // ("unimplemented", unimplemented, 4), // 0xd3
    // ("CALL NC, NN", 2, callnc, 12),         // 0xd4
    // ("PUSH DE", push, 16),         // 0xd5
    // ("SUB N", sub_a_n, 8),               // 0xd6
    // ("RST 0x10", rst0x10, 32),           // 0xd7
    // ("RET C", retc, 8),                  // 0xd8
    // ("RETI", reti, 8),                   // 0xd9
    // ("JP C, NN", 2, jpc, 12),               // 0xda
    // ("unimplemented", unimplemented, 4), // 0xdb
    // ("CALL C, NN", 2, callc, 12),           // 0xdc
    // ("unimplemented", unimplemented, 4), // 0xdd
    // ("SBC N", sbc_a_n, 8),               // 0xde
    // ("RST 0x18", rst0x18, 32),           // 0xdf
    // ("LD (0xFF00 + N), A", ld_n_a, 12),  // 0xe0
    // ("POP HL", pop, 12),           // 0xe1
    // ("LD (0xFF00 + C), A", ld_c_a, 8),   // 0xe2
    // ("unimplemented", unimplemented, 4), // 0xe3
    // ("unimplemented", unimplemented, 4), // 0xe4
    // ("PUSH HL", push, 16),         // 0xe5
    // ("AND N", and_a_n, 8),               // 0xe6
    // ("RST 0x20", rst0x20, 32),           // 0xe7
    // ("ADD SP,N", add_sp_n, 16),          // 0xe8
    // ("JP HL", jp_hl, 4),                 // 0xe9
    // ("LD (NN), A", 2, ld_nn_a, 16),         // 0xea
    // ("unimplemented", unimplemented, 4), // 0xeb
    // ("unimplemented", unimplemented, 4), // 0xec
    // ("unimplemented", unimplemented, 4), // 0xed
    // ("XOR N", xor_n, 8),                 // 0xee
    // ("RST 0x28", rst0x28, 32),           // 0xef
    // ("LD A, (0xFF00 + N)", ld_a_n, 12),  // 0xf0
    // ("POP AF", pop_af, 12),              // 0xf1
    // ("LD A, (0xFF00 + C)", ld_a_c, 8),   // 0xf2
    // ("DI", di, 4),                       // 0xf3
    // ("unimplemented", unimplemented, 4), // 0xf4
    // ("PUSH AF", push, 16),         // 0xf5
    // ("OR N", or_n, 8),                   // 0xf6
    // ("RST 0x30", rst0x30, 32),           // 0xf7
    // ("LD HL, SP+N", ld_hl_sp_n, 12),     // 0xf8
    // ("LD SP, HL", ld_sp_hl, 8),          // 0xf9
    // ("LD A, (NN)", 2, ld_a_nn, 16),         // 0xfa
    // ("EI", ei, 4),                       // 0xfb
    // ("unimplemented", unimplemented, 4), // 0xfc
    // ("unimplemented", unimplemented, 4), // 0xfd
    // ("CP N", cp_a_n, 8),                 // 0xfe
    // ("RST 0x38", rst0x38, 32),           // 0xff
];

impl GameBoy {
    fn nop(&mut self) -> u8 {
        4
    }

    fn halt(&mut self) -> u8 {
        self.is_halted = true;
        4
    }

    fn unimplemented(&mut self) -> u8 {
        println!("Unimplemented instruction");
        4
    }

    fn ld_a_n(&mut self) -> u8 {
        self.regs.a = self.fetch_byte();
        8
    }

    fn ld_b_n(&mut self) -> u8 {
        self.regs.b = self.fetch_byte();
        8
    }

    fn ld_c_n(&mut self) -> u8 {
        self.regs.c = self.fetch_byte();
        8
    }

    fn ld_d_n(&mut self) -> u8 {
        self.regs.d = self.fetch_byte();
        8
    }

    fn ld_e_n(&mut self) -> u8 {
        self.regs.e = self.fetch_byte();
        8
    }

    fn ld_h_n(&mut self) -> u8 {
        self.regs.h = self.fetch_byte();
        8
    }

    fn ld_l_n(&mut self) -> u8 {
        self.regs.l = self.fetch_byte();
        8
    }

    fn ld_bc_nn(&mut self) -> u8 {
        let n = self.fetch_word();
        self.regs.set_bc(n);
        12
    }

    fn ld_a_a(&mut self) -> u8 {
        4
    }

    fn ld_a_b(&mut self) -> u8 {
        self.regs.a = self.regs.b;
        4
    }

    fn ld_a_c(&mut self) -> u8 {
        self.regs.a = self.regs.c;
        4
    }

    fn ld_a_d(&mut self) -> u8 {
        self.regs.a = self.regs.d;
        4
    }

    fn ld_a_e(&mut self) -> u8 {
        self.regs.a = self.regs.e;
        4
    }

    fn ld_a_h(&mut self) -> u8 {
        self.regs.a = self.regs.h;
        4
    }

    fn ld_a_l(&mut self) -> u8 {
        self.regs.a = self.regs.l;
        4
    }

    fn ld_b_a(&mut self) -> u8 {
        self.regs.b = self.regs.a;
        4
    }

    fn ld_b_b(&mut self) -> u8 {
        4
    }

    fn ld_b_c(&mut self) -> u8 {
        self.regs.b = self.regs.c;
        4
    }

    fn ld_b_d(&mut self) -> u8 {
        self.regs.b = self.regs.d;
        4
    }

    fn ld_b_e(&mut self) -> u8 {
        self.regs.b = self.regs.e;
        4
    }

    fn ld_b_h(&mut self) -> u8 {
        self.regs.b = self.regs.h;
        4
    }

    fn ld_b_l(&mut self) -> u8 {
        self.regs.b = self.regs.l;
        4
    }

    fn ld_c_a(&mut self) -> u8 {
        self.regs.c = self.regs.a;
        4
    }

    fn ld_c_b(&mut self) -> u8 {
        self.regs.c = self.regs.b;
        4
    }

    fn ld_c_c(&mut self) -> u8 {
        4
    }

    fn ld_c_d(&mut self) -> u8 {
        self.regs.c = self.regs.d;
        4
    }

    fn ld_c_e(&mut self) -> u8 {
        self.regs.c = self.regs.e;
        4
    }

    fn ld_c_h(&mut self) -> u8 {
        self.regs.c = self.regs.h;
        4
    }

    fn ld_c_l(&mut self) -> u8 {
        self.regs.c = self.regs.l;
        4
    }

    fn ld_d_a(&mut self) -> u8 {
        self.regs.d = self.regs.a;
        4
    }

    fn ld_d_b(&mut self) -> u8 {
        self.regs.d = self.regs.b;
        4
    }

    fn ld_d_c(&mut self) -> u8 {
        self.regs.d = self.regs.c;
        4
    }

    fn ld_d_d(&mut self) -> u8 {
        4
    }

    fn ld_d_e(&mut self) -> u8 {
        self.regs.d = self.regs.e;
        4
    }

    fn ld_d_h(&mut self) -> u8 {
        self.regs.d = self.regs.h;
        4
    }

    fn ld_d_l(&mut self) -> u8 {
        self.regs.d = self.regs.l;
        4
    }

    fn ld_e_a(&mut self) -> u8 {
        self.regs.e = self.regs.a;
        4
    }

    fn ld_e_b(&mut self) -> u8 {
        self.regs.e = self.regs.b;
        4
    }

    fn ld_e_c(&mut self) -> u8 {
        self.regs.e = self.regs.c;
        4
    }

    fn ld_e_d(&mut self) -> u8 {
        self.regs.e = self.regs.d;
        4
    }

    fn ld_e_e(&mut self) -> u8 {
        4
    }

    fn ld_e_h(&mut self) -> u8 {
        self.regs.e = self.regs.h;
        4
    }

    fn ld_e_l(&mut self) -> u8 {
        self.regs.e = self.regs.l;
        4
    }

    fn ld_h_a(&mut self) -> u8 {
        self.regs.h = self.regs.a;
        4
    }

    fn ld_h_b(&mut self) -> u8 {
        self.regs.h = self.regs.b;
        4
    }

    fn ld_h_c(&mut self) -> u8 {
        self.regs.h = self.regs.c;
        4
    }

    fn ld_h_d(&mut self) -> u8 {
        self.regs.h = self.regs.d;
        4
    }

    fn ld_h_e(&mut self) -> u8 {
        self.regs.h = self.regs.e;
        4
    }

    fn ld_h_h(&mut self) -> u8 {
        4
    }

    fn ld_h_l(&mut self) -> u8 {
        self.regs.h = self.regs.l;
        4
    }

    fn ld_l_a(&mut self) -> u8 {
        self.regs.l = self.regs.a;
        4
    }

    fn ld_l_b(&mut self) -> u8 {
        self.regs.l = self.regs.b;
        4
    }

    fn ld_l_c(&mut self) -> u8 {
        self.regs.l = self.regs.c;
        4
    }

    fn ld_l_d(&mut self) -> u8 {
        self.regs.l = self.regs.d;
        4
    }

    fn ld_l_e(&mut self) -> u8 {
        self.regs.l = self.regs.e;
        4
    }

    fn ld_l_h(&mut self) -> u8 {
        self.regs.l = self.regs.h;
        4
    }

    fn ld_l_l(&mut self) -> u8 {
        4
    }

    fn ld_a_bc(&mut self) -> u8 {
        self.regs.a = self.mmu.read_byte(self.regs.get_bc());
        8
    }

    fn ld_a_de(&mut self) -> u8 {
        self.regs.a = self.mmu.read_byte(self.regs.get_de());
        8
    }

    fn ld_a_hl(&mut self) -> u8 {
        self.regs.a = self.mmu.read_byte(self.regs.get_hl());
        8
    }

    fn ld_b_hl(&mut self) -> u8 {
        self.regs.b = self.mmu.read_byte(self.regs.get_hl());
        8
    }

    fn ld_c_hl(&mut self) -> u8 {
        self.regs.c = self.mmu.read_byte(self.regs.get_hl());
        8
    }

    fn ld_d_hl(&mut self) -> u8 {
        self.regs.d = self.mmu.read_byte(self.regs.get_hl());
        8
    }

    fn ld_e_hl(&mut self) -> u8 {
        self.regs.e = self.mmu.read_byte(self.regs.get_hl());
        8
    }

    fn ld_h_hl(&mut self) -> u8 {
        self.regs.h = self.mmu.read_byte(self.regs.get_hl());
        8
    }

    fn ld_l_hl(&mut self) -> u8 {
        self.regs.l = self.mmu.read_byte(self.regs.get_hl());
        8
    }

    fn ld_de_nn(&mut self) -> u8 {
        let n = self.fetch_word();
        self.regs.set_de(n);
        12
    }

    fn ld_hl_nn(&mut self) -> u8 {
        let n = self.fetch_word();
        self.regs.set_hl(n);
        12
    }

    fn ld_sp_nn(&mut self) -> u8 {
        self.regs.sp = self.fetch_word();
        12
    }

    fn ld_bc_a(&mut self) -> u8 {
        self.mmu.write_byte(self.regs.get_bc(), self.regs.a);
        8
    }

    fn ld_de_a(&mut self) -> u8 {
        self.mmu.write_byte(self.regs.get_de(), self.regs.a);
        8
    }

    fn ld_hl_a(&mut self) -> u8 {
        self.mmu.write_byte(self.regs.get_hl(), self.regs.a);
        8
    }

    fn ld_hl_b(&mut self) -> u8 {
        self.mmu.write_byte(self.regs.get_hl(), self.regs.b);
        8
    }

    fn ld_hl_c(&mut self) -> u8 {
        self.mmu.write_byte(self.regs.get_hl(), self.regs.c);
        8
    }

    fn ld_hl_d(&mut self) -> u8 {
        self.mmu.write_byte(self.regs.get_hl(), self.regs.d);
        8
    }

    fn ld_hl_e(&mut self) -> u8 {
        self.mmu.write_byte(self.regs.get_hl(), self.regs.e);
        8
    }

    fn ld_hl_h(&mut self) -> u8 {
        self.mmu.write_byte(self.regs.get_hl(), self.regs.h);
        8
    }

    fn ld_hl_l(&mut self) -> u8 {
        self.mmu.write_byte(self.regs.get_hl(), self.regs.l);
        8
    }

    fn ld_nn_sp(&mut self) -> u8 {
        let n = self.fetch_word();
        self.mmu.write_word(n, self.regs.sp);
        20
    }

    fn ldi_a_hl(&mut self) -> u8 {
        self.ld_a_hl();
        self.inc_hl();
        8
    }

    fn ldi_hl_a(&mut self) -> u8 {
        self.ld_hl_a();
        self.inc_hl();
        8
    }

    fn ldd_a_hl(&mut self) -> u8 {
        self.ld_a_hl();
        self.dec_hl();
        8
    }

    fn ldd_hl_a(&mut self) -> u8 {
        self.ld_hl_a();
        self.dec_hl();
        8
    }

    fn inc_a(&mut self) -> u8 {
        let (a, f) = inc_r(self.regs.a, self.regs.f);
        self.regs.a = a;
        self.regs.f = f;
        4
    }

    fn inc_b(&mut self) -> u8 {
        let (b, f) = inc_r(self.regs.b, self.regs.f);
        self.regs.b = b;
        self.regs.f = f;
        4
    }

    fn inc_c(&mut self) -> u8 {
        let (c, f) = inc_r(self.regs.c, self.regs.f);
        self.regs.c = c;
        self.regs.f = f;
        4
    }

    fn inc_d(&mut self) -> u8 {
        let (d, f) = inc_r(self.regs.d, self.regs.f);
        self.regs.d = d;
        self.regs.f = f;
        4
    }

    fn inc_e(&mut self) -> u8 {
        let (e, f) = inc_r(self.regs.e, self.regs.f);
        self.regs.e = e;
        self.regs.f = f;
        4
    }

    fn inc_h(&mut self) -> u8 {
        let (h, f) = inc_r(self.regs.h, self.regs.f);
        self.regs.h = h;
        self.regs.f = f;
        4
    }

    fn inc_l(&mut self) -> u8 {
        let (l, f) = inc_r(self.regs.l, self.regs.f);
        self.regs.l = l;
        self.regs.f = f;
        4
    }

    fn dec_a(&mut self) -> u8 {
        let (a, f) = dec_r(self.regs.a, self.regs.f);
        self.regs.a = a;
        self.regs.f = f;
        4
    }

    fn dec_b(&mut self) -> u8 {
        let (b, f) = dec_r(self.regs.b, self.regs.f);
        self.regs.b = b;
        self.regs.f = f;
        4
    }

    fn dec_c(&mut self) -> u8 {
        let (c, f) = dec_r(self.regs.c, self.regs.f);
        self.regs.a = c;
        self.regs.f = f;
        4
    }

    fn dec_d(&mut self) -> u8 {
        let (d, f) = dec_r(self.regs.d, self.regs.f);
        self.regs.a = d;
        self.regs.f = f;
        4
    }

    fn dec_e(&mut self) -> u8 {
        let (e, f) = dec_r(self.regs.e, self.regs.f);
        self.regs.a = e;
        self.regs.f = f;
        4
    }

    fn dec_h(&mut self) -> u8 {
        let (h, f) = dec_r(self.regs.h, self.regs.f);
        self.regs.a = h;
        self.regs.f = f;
        4
    }

    fn dec_l(&mut self) -> u8 {
        let (l, f) = dec_r(self.regs.l, self.regs.f);
        self.regs.a = l;
        self.regs.f = f;
        4
    }

    fn inc_bc(&mut self) -> u8 {
        self.regs.set_bc(self.regs.get_bc() + 1);
        8
    }

    fn inc_de(&mut self) -> u8 {
        self.regs.set_de(self.regs.get_de() + 1);
        8
    }

    fn inc_hl(&mut self) -> u8 {
        self.regs.set_hl(self.regs.get_hl() + 1);
        8
    }

    fn inc_sp(&mut self) -> u8 {
        self.regs.sp += 1;
        8
    }

    fn dec_bc(&mut self) -> u8 {
        self.regs.set_bc(self.regs.get_bc() - 1);
        8
    }

    fn dec_de(&mut self) -> u8 {
        self.regs.set_de(self.regs.get_de() - 1);
        8
    }

    fn dec_hl(&mut self) -> u8 {
        self.regs.set_hl(self.regs.get_hl() - 1);
        8
    }

    fn dec_sp(&mut self) -> u8 {
        self.regs.sp -= 1;
        8
    }

    fn inc_at_hl(&mut self) -> u8 {
        let addr = self.regs.get_hl();
        let old_carry = self.regs.f & CARRY;
        let (r, mut f) = add(self.mmu.read_byte(addr), 1, self.regs.f);
        if old_carry == CARRY {
            f |= CARRY;
        } else {
            f &= !CARRY;
        }
        self.regs.f = f;
        self.mmu.write_byte(addr, r);
        12
    }

    fn dec_at_hl(&mut self) -> u8 {
        let addr = self.regs.get_hl();
        let old_carry = self.regs.f & CARRY;
        let (r, mut f) = sub(self.mmu.read_byte(addr), 1, self.regs.f);
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
        self.regs.f = f;
        self.mmu.write_byte(addr, r);
        12
    }

    fn add_hl_bc(&mut self) -> u8 {
        let (hl, f) = add_16(self.regs.get_hl(), self.regs.get_bc(), self.regs.f);
        self.regs.set_hl(hl);
        self.regs.f = f;
        8
    }

    fn add_hl_de(&mut self) -> u8 {
        let (hl, f) = add_16(self.regs.get_hl(), self.regs.get_de(), self.regs.f);
        self.regs.set_hl(hl);
        self.regs.f = f;
        8
    }

    fn add_hl_hl(&mut self) -> u8 {
        let (hl, f) = add_16(self.regs.get_hl(), self.regs.get_de(), self.regs.f);
        self.regs.set_hl(hl);
        self.regs.f = f;
        8
    }

    fn add_hl_sp(&mut self) -> u8 {
        let (hl, f) = add_16(self.regs.get_hl(), self.regs.sp, self.regs.f);
        self.regs.set_hl(hl);
        self.regs.f = f;
        8
    }

    fn jr_n(&mut self) -> u8 {
        self.jr_cc(true)
    }

    fn jr_z(&mut self) -> u8 {
        self.jr_cc(self.regs.f & ZERO == ZERO)
    }

    fn jr_nz(&mut self) -> u8 {
        self.jr_cc(self.regs.f & ZERO != ZERO)
    }

    fn jr_c(&mut self) -> u8 {
        self.jr_cc(self.regs.f & CARRY == CARRY)
    }

    fn jr_nc(&mut self) -> u8 {
        self.jr_cc(self.regs.f & CARRY != CARRY)
    }

    fn jr_cc(&mut self, condition: bool) -> u8 {
        if condition {
            self.regs.pc += self.fetch_byte() as i8 as u16;
            return 12;
        }
        8
    }

    fn rlca(&mut self) -> u8 {
        let bit_7 = self.regs.a >> 7;
        self.regs.a = (self.regs.a << 1) | bit_7;
        self.regs.f = if bit_7 == 1 { CARRY } else { 0 };
        4
    }

    fn rla(&mut self) -> u8 {
        let bit_7 = self.regs.a >> 7;
        self.regs.a <<= 1;
        self.regs.a |= if self.regs.f & CARRY == CARRY { 1 } else { 0 };
        self.regs.f = if bit_7 == 1 { CARRY } else { 0 };
        4
    }

    fn rrca(&mut self) -> u8 {
        let bit_0 = self.regs.a & 1;
        self.regs.a = (self.regs.a >> 1) | (bit_0 << 7);
        self.regs.f = if bit_0 == 1 { CARRY } else { 0 };
        4
    }

    fn rra(&mut self) -> u8 {
        let old_carry = (self.regs.f & CARRY) << 3;
        self.regs.f = if self.regs.a & 1 == 1 { CARRY } else { 0 };
        self.regs.a = (self.regs.a >> 1) | old_carry;
        4
    }

    fn cpl(&mut self) -> u8 {
        self.regs.a = !self.regs.a;
        self.regs.f |= SUB | HALF_CARRY;
        4
    }

    fn scf(&mut self) -> u8 {
        self.regs.f |= CARRY;
        self.regs.f &= !HALF_CARRY;
        self.regs.f &= !SUB;
        4
    }

    fn ccf(&mut self) -> u8 {
        self.regs.f ^= CARRY;
        self.regs.f &= !HALF_CARRY;
        self.regs.f &= !SUB;
        4
    }

    fn daa(&mut self) -> u8 {
        if self.regs.f & SUB == SUB {
            self.regs.a += if self.regs.f & HALF_CARRY == HALF_CARRY {
                self.regs.f &= !HALF_CARRY;
                if self.regs.f & CARRY == CARRY {
                    0x9a
                } else {
                    0xfa
                }
            } else if self.regs.f & CARRY == CARRY {
                0xa0
            } else {
                0
            };
        } else {
            if self.regs.a > 0x99 || self.regs.f & CARRY == CARRY {
                self.regs.a += 0x60;
                self.regs.f |= CARRY;
            }
            if (self.regs.a & 0xf) > 0x09 || self.regs.f & HALF_CARRY == HALF_CARRY {
                self.regs.a += 0x06;
                self.regs.f &= !HALF_CARRY;
            }
        }
        if self.regs.a == 0 {
            self.regs.f |= ZERO;
        } else {
            self.regs.f &= !ZERO;
        }
        4
    }
}

fn inc_r(reg: u8, f: u8) -> (u8, u8) {
    let (r, mut new_f) = add(reg, 1, f);
    new_f = if f & CARRY == CARRY {
        new_f | CARRY
    } else {
        new_f & !CARRY
    };
    (r, new_f)
}

fn dec_r(reg: u8, f: u8) -> (u8, u8) {
    let (r, mut new_f) = sub(reg, 1, f);
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

fn add(op1: u8, op2: u8, f: u8) -> (u8, u8) {
    add_impl(op1, op2, f, false)
}

fn addc(op1: u8, op2: u8, f: u8) -> (u8, u8) {
    add_impl(op1, op2, f, true)
}

fn add_impl(op1: u8, op2: u8, f: u8, use_carry: bool) -> (u8, u8) {
    let carry = if use_carry { ((f & CARRY) >> 4) } else { 0 };
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

fn sub(op1: u8, op2: u8, f: u8) -> (u8, u8) {
    sub_impl(op1, op2, f, false)
}

fn subc(op1: u8, op2: u8, f: u8) -> (u8, u8) {
    sub_impl(op1, op2, f, true)
}

fn sub_impl(op1: u8, op2: u8, f: u8, use_borrow: bool) -> (u8, u8) {
    let borrow = if use_borrow { ((f & CARRY) >> 4) } else { 0 };
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
