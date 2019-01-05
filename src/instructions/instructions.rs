const CARRY: u8 = 0b0001_0000;
const HALF_CARRY: u8 = 0b0010_0000;
const SUB: u8 = 0b0100_0000;
const ZERO: u8 = 0b1000_0000;

const INSTRUCTIONS: [(&'static str, fn(&mut GameBoy) -> u8); 256] = [
    ("NOP", GameBoy::nop),                     // 0x01
    ("LD BC, NN", GameBoy::ld_bc_nn),          // 0x02
    ("LD (BC), A", GameBoy::ld_bc_a),          // 0x02
    ("INC BC", GameBoy::inc_bc),               // 0x03
    ("INC B", GameBoy::inc_b),                 // 0x04
    ("DEC B", GameBoy::dec_b),                 // 0x05
    ("LD B, N", GameBoy::ld_b_n),              // 0x06
    ("RLCA", GameBoy::rlca),                   // 0x07
    ("LD (NN), SP", GameBoy::ld_nn_sp),        // 0x08
    ("ADD HL, BC", GameBoy::add_hl_bc),        // 0x09
    ("LD A, (BC)", GameBoy::ld_a_bc),          // 0x0a
    ("DEC BC", GameBoy::dec_bc),               // 0x0b
    ("INC C", GameBoy::inc_c),                 // 0x0c
    ("DEC C", GameBoy::dec_c),                 // 0x0d
    ("LD C, N", GameBoy::ld_c_n),              // 0x0e
    ("RRCA", GameBoy::rrca),                   // 0x0f
    ("STOP", GameBoy::unimplemented),          // 0x10
    ("LD DE, NN", GameBoy::ld_de_nn),          // 0x11
    ("LD (DE), A", GameBoy::ld_de_a),          // 0x12
    ("INC DE", GameBoy::inc_de),               // 0x13
    ("INC D", GameBoy::inc_d),                 // 0x14
    ("DEC D", GameBoy::dec_d),                 // 0x15
    ("LD D, N", GameBoy::ld_d_n),              // 0x16
    ("RLA", GameBoy::rla),                     // 0x17
    ("JR N", GameBoy::jr_n),                   // 0x18
    ("ADD HL, DE", GameBoy::add_hl_de),        // 0x19
    ("LD A, (DE)", GameBoy::ld_a_de),          // 0x1a
    ("DEC DE", GameBoy::dec_de),               // 0x1b
    ("INC E", GameBoy::inc_e),                 // 0x1c
    ("DEC E", GameBoy::dec_e),                 // 0x1d
    ("LD E, N", GameBoy::ld_e_n),              // 0x1e
    ("RRA", GameBoy::rra),                     // 0x1f
    ("JR NZ, N", GameBoy::jr_nz),              // 0x20
    ("LD HL, NN", GameBoy::ld_hl_nn),          // 0x21
    ("LDI (HL), A", GameBoy::ldi_hl_a),        // 0x22
    ("INC HL", GameBoy::inc_hl),               // 0x23
    ("INC H", GameBoy::inc_h),                 // 0x24
    ("DEC H", GameBoy::dec_h),                 // 0x25
    ("LD H, N", GameBoy::ld_h_n),              // 0x26
    ("DAA", GameBoy::daa),                     // 0x27
    ("JR Z, N", GameBoy::jr_z),                // 0x28
    ("ADD HL, HL", GameBoy::add_hl_hl),        // 0x29
    ("LDI A, (HL)", GameBoy::ldi_a_hl),        // 0x2a
    ("DEC HL", GameBoy::dec_hl),               // 0x2b
    ("INC L", GameBoy::inc_l),                 // 0x2c
    ("DEC L", GameBoy::dec_l),                 // 0x2d
    ("LD L, N", GameBoy::ld_l_n),              // 0x2e
    ("CPL", GameBoy::cpl),                     // 0x2f
    ("JR NC, N", GameBoy::jr_nc),              // 0x30
    ("LD SP, NN", GameBoy::ld_sp_nn),          // 0x31
    ("LDD (HL), A", GameBoy::ldd_hl_a),        // 0x32
    ("INC SP", GameBoy::inc_sp),               // 0x33
    ("INC (HL)", GameBoy::inc_at_hl),          // 0x34
    ("DEC (HL)", GameBoy::dec_at_hl),          // 0x35
    ("LD (HL), N", GameBoy::ld_hl_nn),         // 0x36
    ("SCF", GameBoy::scf),                     // 0x37
    ("JR C, N", GameBoy::jr_c),                // 0x38
    ("ADD HL, SP", GameBoy::add_hl_sp),        // 0x39
    ("LDD A, (HL)", GameBoy::ldd_a_hl),        // 0x3a
    ("DEC SP", GameBoy::dec_sp),               // 0x3b
    ("INC A", GameBoy::inc_a),                 // 0x3c
    ("DEC A", GameBoy::dec_a),                 // 0x3d
    ("LD A, N", GameBoy::ld_a_n),              // 0x3e
    ("CCF", GameBoy::ccf),                     // 0x3f
    ("LD B, B", GameBoy::ld_b_b),              // 0x40
    ("LD B, C", GameBoy::ld_b_c),              // 0x41
    ("LD B, D", GameBoy::ld_b_d),              // 0x42
    ("LD B, E", GameBoy::ld_b_e),              // 0x43
    ("LD B, H", GameBoy::ld_b_h),              // 0x44
    ("LD B, L", GameBoy::ld_b_l),              // 0x45
    ("LD B, (HL)", GameBoy::ld_b_hl),          // 0x46
    ("LD B, A", GameBoy::ld_b_a),              // 0x47
    ("LD C, B", GameBoy::ld_c_b),              // 0x48
    ("LD C, C", GameBoy::ld_c_c),              // 0x49
    ("LD C, D", GameBoy::ld_c_d),              // 0x4a
    ("LD C, E", GameBoy::ld_c_e),              // 0x4b
    ("LD C, H", GameBoy::ld_c_h),              // 0x4c
    ("LD C, L", GameBoy::ld_c_l),              // 0x4d
    ("LD C, (HL)", GameBoy::ld_c_hl),          // 0x4e
    ("LD C, A", GameBoy::ld_c_a),              // 0x4f
    ("LD D, B", GameBoy::ld_d_b),              // 0x50
    ("LD D, C", GameBoy::ld_d_c),              // 0x51
    ("LD D, D", GameBoy::ld_d_d),              // 0x52
    ("LD D, E", GameBoy::ld_d_e),              // 0x53
    ("LD D, H", GameBoy::ld_d_h),              // 0x54
    ("LD D, L", GameBoy::ld_d_l),              // 0x55
    ("LD D, (HL)", GameBoy::ld_d_hl),          // 0x56
    ("LD D, A", GameBoy::ld_d_a),              // 0x57
    ("LD E, B", GameBoy::ld_e_b),              // 0x58
    ("LD E, C", GameBoy::ld_e_c),              // 0x59
    ("LD E, D", GameBoy::ld_e_d),              // 0x5a
    ("LD E, E", GameBoy::ld_e_e),              // 0x5b
    ("LD E, H", GameBoy::ld_e_h),              // 0x5c
    ("LD E, L", GameBoy::ld_e_l),              // 0x5d
    ("LD E, (HL)", GameBoy::ld_e_hl),          // 0x5e
    ("LD E, A", GameBoy::ld_e_a),              // 0x5f
    ("LD H, B", GameBoy::ld_h_b),              // 0x60
    ("LD H, C", GameBoy::ld_h_c),              // 0x61
    ("LD H, D", GameBoy::ld_h_d),              // 0x62
    ("LD H, E", GameBoy::ld_h_e),              // 0x63
    ("LD H, H", GameBoy::ld_h_h),              // 0x64
    ("LD H, L", GameBoy::ld_h_l),              // 0x65
    ("LD H, (HL)", GameBoy::ld_h_hl),          // 0x66
    ("LD H, A", GameBoy::ld_h_a),              // 0x67
    ("LD L, B", GameBoy::ld_l_b),              // 0x68
    ("LD L, C", GameBoy::ld_l_c),              // 0x69
    ("LD L, D", GameBoy::ld_l_d),              // 0x6a
    ("LD L, E", GameBoy::ld_l_e),              // 0x6b
    ("LD L, H", GameBoy::ld_l_h),              // 0x6c
    ("LD L, L", GameBoy::ld_l_l),              // 0x6d
    ("LD L, (HL)", GameBoy::ld_l_hl),          // 0x6e
    ("LD L, A", GameBoy::ld_l_a),              // 0x6f
    ("LD (HL), B", GameBoy::ld_hl_b),          // 0x70
    ("LD (HL), C", GameBoy::ld_hl_c),          // 0x71
    ("LD (HL), D", GameBoy::ld_hl_d),          // 0x72
    ("LD (HL), E", GameBoy::ld_hl_e),          // 0x73
    ("LD (HL), H", GameBoy::ld_hl_h),          // 0x74
    ("LD (HL), L", GameBoy::ld_hl_l),          // 0x75
    ("HALT", GameBoy::halt),                   // 0x76
    ("LD (HL), A", GameBoy::ld_hl_a),          // 0x77
    ("LD A, B", GameBoy::ld_a_b),              // 0x78
    ("LD A, C", GameBoy::ld_a_c),              // 0x79
    ("LD A, D", GameBoy::ld_a_d),              // 0x7a
    ("LD A, E", GameBoy::ld_a_e),              // 0x7b
    ("LD A, H", GameBoy::ld_a_h),              // 0x7c
    ("LD A, L", GameBoy::ld_a_l),              // 0x7d
    ("LD A, (HL)", GameBoy::ld_a_hl),          // 0x7e
    ("LD A, A", GameBoy::ld_a_a),              // 0x7f
    ("ADD A, B", GameBoy::add_a_b),            // 0x80
    ("ADD A, C", GameBoy::add_a_c),            // 0x81
    ("ADD A, D", GameBoy::add_a_d),            // 0x82
    ("ADD A, E", GameBoy::add_a_e),            // 0x83
    ("ADD A, H", GameBoy::add_a_h),            // 0x84
    ("ADD A, L", GameBoy::add_a_l),            // 0x85
    ("ADD A, (HL)", GameBoy::add_a_hl),        // 0x86
    ("ADD A", GameBoy::add_a_a),               // 0x87
    ("ADC B", GameBoy::adc_a_b),               // 0x88
    ("ADC C", GameBoy::adc_a_c),               // 0x89
    ("ADC D", GameBoy::adc_a_d),               // 0x8a
    ("ADC E", GameBoy::adc_a_e),               // 0x8b
    ("ADC H", GameBoy::adc_a_h),               // 0x8c
    ("ADC L", GameBoy::adc_a_l),               // 0x8d
    ("ADC (HL)", GameBoy::adc_a_hl),           // 0x8e
    ("ADC A", GameBoy::adc_a_a),               // 0x8f
    ("SUB B", GameBoy::sub_a_b),               // 0x90
    ("SUB C", GameBoy::sub_a_c),               // 0x91
    ("SUB D", GameBoy::sub_a_d),               // 0x92
    ("SUB E", GameBoy::sub_a_e),               // 0x93
    ("SUB H", GameBoy::sub_a_h),               // 0x94
    ("SUB L", GameBoy::sub_a_l),               // 0x95
    ("SUB (HL)", GameBoy::sub_a_hl),           // 0x96
    ("SUB A", GameBoy::sub_a_a),               // 0x97
    ("SBC B", GameBoy::sbc_a_b),               // 0x98
    ("SBC C", GameBoy::sbc_a_c),               // 0x99
    ("SBC D", GameBoy::sbc_a_d),               // 0x9a
    ("SBC E", GameBoy::sbc_a_e),               // 0x9b
    ("SBC H", GameBoy::sbc_a_h),               // 0x9c
    ("SBC L", GameBoy::sbc_a_l),               // 0x9d
    ("SBC (HL)", GameBoy::sbc_a_hl),           // 0x9e
    ("SBC A", GameBoy::sbc_a_a),               // 0x9f
    ("AND B", GameBoy::and_a_b),               // 0xa0
    ("AND C", GameBoy::and_a_c),               // 0xa1
    ("AND D", GameBoy::and_a_d),               // 0xa2
    ("AND E", GameBoy::and_a_e),               // 0xa3
    ("AND H", GameBoy::and_a_h),               // 0xa4
    ("AND L", GameBoy::and_a_l),               // 0xa5
    ("AND (HL)", GameBoy::and_a_hl),           // 0xa6
    ("AND A", GameBoy::and_a_a),               // 0xa7
    ("XOR B", GameBoy::xor_a_b),               // 0xa8
    ("XOR C", GameBoy::xor_a_c),               // 0xa9
    ("XOR D", GameBoy::xor_a_d),               // 0xaa
    ("XOR E", GameBoy::xor_a_e),               // 0xab
    ("XOR H", GameBoy::xor_a_h),               // 0xac
    ("XOR L", GameBoy::xor_a_l),               // 0xad
    ("XOR (HL)", GameBoy::xor_a_hl),           // 0xae
    ("XOR A", GameBoy::xor_a_a),               // 0xaf
    ("OR B", GameBoy::or_a_b),                 // 0xb0
    ("OR C", GameBoy::or_a_c),                 // 0xb1
    ("OR D", GameBoy::or_a_d),                 // 0xb2
    ("OR E", GameBoy::or_a_e),                 // 0xb3
    ("OR H", GameBoy::or_a_h),                 // 0xb4
    ("OR L", GameBoy::or_a_l),                 // 0xb5
    ("OR (HL)", GameBoy::or_a_hl),             // 0xb6
    ("OR A", GameBoy::or_a_a),                 // 0xb7
    ("CP B", GameBoy::cp_a_b),                 // 0xb8
    ("CP C", GameBoy::cp_a_c),                 // 0xb9
    ("CP D", GameBoy::cp_a_d),                 // 0xba
    ("CP E", GameBoy::cp_a_e),                 // 0xbb
    ("CP H", GameBoy::cp_a_h),                 // 0xbc
    ("CP L", GameBoy::cp_a_l),                 // 0xbd
    ("CP (HL)", GameBoy::cp_a_hl),             // 0xbe
    ("CP A", GameBoy::cp_a_a),                 // 0xbf
    ("RET NZ", GameBoy::ret_nz),               // 0xc0
    ("POP BC", GameBoy::pop_bc),               // 0xc1
    ("JP N6, NN", GameBoy::jp_nz),             // 0xc2
    ("JP NN", GameBoy::jp_n),                  // 0xc3
    ("CALL NZ, NN", GameBoy::call_nz),         // 0xc4
    ("PUSH BC", GameBoy::push_bc),             // 0xc5
    ("ADD A, N", GameBoy::add_a_n),            // 0xc6
    ("RST 0x00", GameBoy::rst_0x00),           // 0xc7
    ("RET Z", GameBoy::ret_z),                 // 0xc8
    ("RET", GameBoy::ret_n),                   // 0xc9
    ("JP Z, NN", GameBoy::jp_z),               // 0xca
    ("CB N", GameBoy::execute_cb),             // 0xcb
    ("CALL Z, NN", GameBoy::call_z),           // 0xcc
    ("CALL NN", GameBoy::call_n),              // 0xcd
    ("ADC N", GameBoy::adc_a_n),               // 0xce
    ("RST 0x08", GameBoy::rst_0x08),           // 0xcf
    ("RET NC", GameBoy::ret_nc),               // 0xd0
    ("POP DE", GameBoy::pop_de),               // 0xd1
    ("JP NC, NN", GameBoy::jp_nc),             // 0xd2
    ("unimplemented", GameBoy::unimplemented), // 0xd3
    ("CALL NC, NN", GameBoy::call_nc),         // 0xd4
    ("PUSH DE", GameBoy::push_de),             // 0xd5
    ("SUB N", GameBoy::sub_a_n),               // 0xd6
    ("RST 0x10", GameBoy::rst_0x10),           // 0xd7
    ("RET C", GameBoy::ret_c),                 // 0xd8
    ("RETI", GameBoy::ret_i),                  // 0xd9
    ("JP C, NN", GameBoy::jp_c),               // 0xda
    ("unimplemented", GameBoy::unimplemented), // 0xdb
    ("CALL C, NN", GameBoy::call_c),           // 0xdc
    ("unimplemented", GameBoy::unimplemented), // 0xdd
    ("SBC N", GameBoy::sbc_a_n),               // 0xde
    ("RST 0x18", GameBoy::rst_0x18),           // 0xdf
    ("LD (0xFF00 + N), A", GameBoy::ld_n_a),   // 0xe0
    ("POP HL", GameBoy::pop_hl),               // 0xe1
    ("LD (0xFF00 + C), A", GameBoy::ld_c_a),   // 0xe2
    ("unimplemented", GameBoy::unimplemented), // 0xe3
    ("unimplemented", GameBoy::unimplemented), // 0xe4
    ("PUSH HL", GameBoy::push_hl),             // 0xe5
    ("AND N", GameBoy::and_a_n),               // 0xe6
    ("RST 0x20", GameBoy::rst_0x20),           // 0xe7
    ("ADD SP,N", GameBoy::add_sp_n),           // 0xe8
    ("JP HL", GameBoy::jp_hl),                 // 0xe9
    ("LD (NN), A", GameBoy::ld_nn_a),          // 0xea
    ("unimplemented", GameBoy::unimplemented), // 0xeb
    ("unimplemented", GameBoy::unimplemented), // 0xec
    ("unimplemented", GameBoy::unimplemented), // 0xed
    ("XOR N", GameBoy::xor_a_n),               // 0xee
    ("RST 0x28", GameBoy::rst_0x28),           // 0xef
    ("LD A, (0xFF00 + N)", GameBoy::ld_a_n),   // 0xf0
    ("POP AF", GameBoy::pop_af),               // 0xf1
    ("LD A, (0xFF00 + C)", GameBoy::ld_a_c),   // 0xf2
    ("DI", GameBoy::di),                       // 0xf3
    ("unimplemented", GameBoy::unimplemented), // 0xf4
    ("PUSH AF", GameBoy::push_af),             // 0xf5
    ("OR N", GameBoy::or_a_n),                 // 0xf6
    ("RST 0x30", GameBoy::rst_0x30),           // 0xf7
    ("LD HL, SP+N", GameBoy::ld_hl_sp_n),      // 0xf8
    ("LD SP, HL", GameBoy::ld_sp_hl),          // 0xf9
    ("LD A, (NN)", GameBoy::ld_a_nn),          // 0xfa
    ("EI", GameBoy::ei),                       // 0xfb
    ("unimplemented", GameBoy::unimplemented), // 0xfc
    ("unimplemented", GameBoy::unimplemented), // 0xfd
    ("CP N", GameBoy::cp_a_n),                 // 0xfe
    ("RST 0x38", GameBoy::rst_0x38),           // 0xff
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
        println!(
            "Unimplemented instruction: 0x{:x}",
            self.mmu.read_byte(self.regs.pc - 1)
        );
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

    fn ld_a_nn(&mut self) -> u8 {
        let addr = self.fetch_word();
        self.regs.a = self.mmu.read_byte(addr);
        16
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

    fn ld_n_a(&mut self) -> u8 {
        let addr = 0xff00 + (self.fetch_byte() as u16);
        self.mmu.write_byte(addr, self.regs.a);
        12
    }

    fn ld_nn_a(&mut self) -> u8 {
        let addr = self.fetch_word();
        self.mmu.write_byte(addr, self.regs.a);
        16
    }

    fn ld_nn_sp(&mut self) -> u8 {
        let n = self.fetch_word();
        self.mmu.write_word(n, self.regs.sp);
        20
    }

    fn ld_hl_sp_n(&mut self) -> u8 {
        let n = self.fetch_byte();
        let sp_lo = (self.regs.sp & 0xff) as u8;
        let (_, f) = add(n, sp_lo);
        self.regs.f = f & !ZERO & !SUB;
        self.regs.set_hl(self.regs.sp + ((n as i8) as u16));
        12
    }

    fn ld_sp_hl(&mut self) -> u8 {
        self.regs.sp = self.regs.get_hl();
        8
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
        let (r, mut f) = add(self.mmu.read_byte(addr), 1);
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
        let (r, mut f) = sub(self.mmu.read_byte(addr), 1);
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

    fn add_a_a(&mut self) -> u8 {
        self.add_a(self.regs.a)
    }

    fn add_a_b(&mut self) -> u8 {
        self.add_a(self.regs.b)
    }

    fn add_a_c(&mut self) -> u8 {
        self.add_a(self.regs.c)
    }

    fn add_a_d(&mut self) -> u8 {
        self.add_a(self.regs.d)
    }

    fn add_a_e(&mut self) -> u8 {
        self.add_a(self.regs.e)
    }

    fn add_a_h(&mut self) -> u8 {
        self.add_a(self.regs.h)
    }

    fn add_a_l(&mut self) -> u8 {
        self.add_a(self.regs.l)
    }

    fn add_a_hl(&mut self) -> u8 {
        4 + self.add_a(self.mmu.read_byte(self.regs.get_hl()))
    }

    fn add_a_n(&mut self) -> u8 {
        let n = self.fetch_byte();
        4 + self.add_a(n)
    }

    fn add_a(&mut self, operand: u8) -> u8 {
        let (a, f) = add(self.regs.a, operand);
        self.regs.a = a;
        self.regs.f = f;
        4
    }

    fn add_hl_bc(&mut self) -> u8 {
        self.add_hl(self.regs.get_bc())
    }

    fn add_hl_de(&mut self) -> u8 {
        self.add_hl(self.regs.get_de())
    }

    fn add_hl_hl(&mut self) -> u8 {
        self.add_hl(self.regs.get_hl())
    }

    fn add_hl_sp(&mut self) -> u8 {
        self.add_hl(self.regs.sp)
    }

    fn add_hl(&mut self, operand: u16) -> u8 {
        let (hl, f) = add_16(self.regs.get_hl(), operand, self.regs.f);
        self.regs.set_hl(hl);
        self.regs.f = f;
        8
    }

    fn add_sp_n(&mut self) -> u8 {
        let n = self.fetch_byte();
        let sp_lo = (self.regs.sp & 0xff) as u8;
        let (_, f) = add(n, sp_lo);
        self.regs.f = f & !ZERO & !SUB;
        self.regs.sp += (n as i8) as u16;
        16
    }

    fn adc_a_a(&mut self) -> u8 {
        self.adc_a(self.regs.a)
    }

    fn adc_a_b(&mut self) -> u8 {
        self.adc_a(self.regs.b)
    }

    fn adc_a_c(&mut self) -> u8 {
        self.adc_a(self.regs.c)
    }

    fn adc_a_d(&mut self) -> u8 {
        self.adc_a(self.regs.d)
    }

    fn adc_a_e(&mut self) -> u8 {
        self.adc_a(self.regs.e)
    }

    fn adc_a_h(&mut self) -> u8 {
        self.adc_a(self.regs.h)
    }

    fn adc_a_l(&mut self) -> u8 {
        self.adc_a(self.regs.l)
    }

    fn adc_a_hl(&mut self) -> u8 {
        4 + self.adc_a(self.mmu.read_byte(self.regs.get_hl()))
    }

    fn adc_a_n(&mut self) -> u8 {
        let n = self.fetch_byte();
        4 + self.adc_a(n)
    }

    fn adc_a(&mut self, operand: u8) -> u8 {
        let (a, f) = addc(self.regs.a, operand, self.regs.f & CARRY == CARRY);
        self.regs.a = a;
        self.regs.f = f;
        4
    }

    fn sub_a_a(&mut self) -> u8 {
        self.sub_a(self.regs.a)
    }

    fn sub_a_b(&mut self) -> u8 {
        self.sub_a(self.regs.b)
    }

    fn sub_a_c(&mut self) -> u8 {
        self.sub_a(self.regs.c)
    }

    fn sub_a_d(&mut self) -> u8 {
        self.sub_a(self.regs.d)
    }

    fn sub_a_e(&mut self) -> u8 {
        self.sub_a(self.regs.e)
    }

    fn sub_a_h(&mut self) -> u8 {
        self.sub_a(self.regs.h)
    }

    fn sub_a_l(&mut self) -> u8 {
        self.sub_a(self.regs.l)
    }

    fn sub_a_hl(&mut self) -> u8 {
        4 + self.sub_a(self.mmu.read_byte(self.regs.get_hl()))
    }

    fn sub_a_n(&mut self) -> u8 {
        let n = self.fetch_byte();
        4 + self.sub_a(n)
    }

    fn sub_a(&mut self, operand: u8) -> u8 {
        let (a, f) = sub(self.regs.a, operand);
        self.regs.a = a;
        self.regs.f = f;
        4
    }

    fn sbc_a_a(&mut self) -> u8 {
        self.sbc_a(self.regs.a)
    }

    fn sbc_a_b(&mut self) -> u8 {
        self.sbc_a(self.regs.b)
    }

    fn sbc_a_c(&mut self) -> u8 {
        self.sbc_a(self.regs.c)
    }

    fn sbc_a_d(&mut self) -> u8 {
        self.sbc_a(self.regs.d)
    }

    fn sbc_a_e(&mut self) -> u8 {
        self.sbc_a(self.regs.e)
    }

    fn sbc_a_h(&mut self) -> u8 {
        self.sbc_a(self.regs.h)
    }

    fn sbc_a_l(&mut self) -> u8 {
        self.sbc_a(self.regs.l)
    }

    fn sbc_a_hl(&mut self) -> u8 {
        4 + self.sbc_a(self.mmu.read_byte(self.regs.get_hl()))
    }

    fn sbc_a_n(&mut self) -> u8 {
        let n = self.fetch_byte();
        4 + self.sbc_a(n)
    }

    fn sbc_a(&mut self, operand: u8) -> u8 {
        let (a, f) = subc(self.regs.a, operand, self.regs.f & CARRY == CARRY);
        self.regs.a = a;
        self.regs.f = f;
        4
    }

    fn and_a_a(&mut self) -> u8 {
        self.and_a(self.regs.a)
    }

    fn and_a_b(&mut self) -> u8 {
        self.and_a(self.regs.b)
    }

    fn and_a_c(&mut self) -> u8 {
        self.and_a(self.regs.c)
    }

    fn and_a_d(&mut self) -> u8 {
        self.and_a(self.regs.d)
    }

    fn and_a_e(&mut self) -> u8 {
        self.and_a(self.regs.e)
    }

    fn and_a_h(&mut self) -> u8 {
        self.and_a(self.regs.h)
    }

    fn and_a_l(&mut self) -> u8 {
        self.and_a(self.regs.l)
    }

    fn and_a_hl(&mut self) -> u8 {
        4 + self.and_a(self.mmu.read_byte(self.regs.get_hl()))
    }

    fn and_a_n(&mut self) -> u8 {
        let n = self.fetch_byte();
        4 + self.and_a(n)
    }

    fn and_a(&mut self, operand: u8) -> u8 {
        self.regs.a &= operand;
        self.regs.f = HALF_CARRY | if self.regs.a == 0 { ZERO } else { 0 };
        4
    }

    fn xor_a_a(&mut self) -> u8 {
        self.xor_a(self.regs.a)
    }

    fn xor_a_b(&mut self) -> u8 {
        self.xor_a(self.regs.b)
    }

    fn xor_a_c(&mut self) -> u8 {
        self.xor_a(self.regs.c)
    }

    fn xor_a_d(&mut self) -> u8 {
        self.xor_a(self.regs.d)
    }

    fn xor_a_e(&mut self) -> u8 {
        self.xor_a(self.regs.e)
    }

    fn xor_a_h(&mut self) -> u8 {
        self.xor_a(self.regs.h)
    }

    fn xor_a_l(&mut self) -> u8 {
        self.xor_a(self.regs.l)
    }

    fn xor_a_hl(&mut self) -> u8 {
        4 + self.xor_a(self.mmu.read_byte(self.regs.get_hl()))
    }

    fn xor_a_n(&mut self) -> u8 {
        let n = self.fetch_byte();
        4 + self.xor_a(n)
    }

    fn xor_a(&mut self, operand: u8) -> u8 {
        self.regs.a ^= operand;
        self.regs.f = if self.regs.a == 0 { ZERO } else { 0 };
        4
    }

    fn or_a_a(&mut self) -> u8 {
        self.or_a(self.regs.a)
    }

    fn or_a_b(&mut self) -> u8 {
        self.or_a(self.regs.b)
    }

    fn or_a_c(&mut self) -> u8 {
        self.or_a(self.regs.c)
    }

    fn or_a_d(&mut self) -> u8 {
        self.or_a(self.regs.d)
    }

    fn or_a_e(&mut self) -> u8 {
        self.or_a(self.regs.e)
    }

    fn or_a_h(&mut self) -> u8 {
        self.or_a(self.regs.h)
    }

    fn or_a_l(&mut self) -> u8 {
        self.or_a(self.regs.l)
    }

    fn or_a_hl(&mut self) -> u8 {
        4 + self.or_a(self.mmu.read_byte(self.regs.get_hl()))
    }

    fn or_a_n(&mut self) -> u8 {
        let n = self.fetch_byte();
        4 + self.or_a(n)
    }

    fn or_a(&mut self, operand: u8) -> u8 {
        self.regs.a |= operand;
        self.regs.f = if self.regs.a == 0 { ZERO } else { 0 };
        4
    }

    fn cp_a_a(&mut self) -> u8 {
        self.cp_a(self.regs.a)
    }

    fn cp_a_b(&mut self) -> u8 {
        self.cp_a(self.regs.b)
    }

    fn cp_a_c(&mut self) -> u8 {
        self.cp_a(self.regs.c)
    }

    fn cp_a_d(&mut self) -> u8 {
        self.cp_a(self.regs.d)
    }

    fn cp_a_e(&mut self) -> u8 {
        self.cp_a(self.regs.e)
    }

    fn cp_a_h(&mut self) -> u8 {
        self.cp_a(self.regs.h)
    }

    fn cp_a_l(&mut self) -> u8 {
        self.cp_a(self.regs.l)
    }

    fn cp_a_hl(&mut self) -> u8 {
        4 + self.cp_a(self.mmu.read_byte(self.regs.get_hl()))
    }

    fn cp_a_n(&mut self) -> u8 {
        let n = self.fetch_byte();
        4 + self.cp_a(n)
    }

    fn cp_a(&mut self, operand: u8) -> u8 {
        let (_, f) = sub(self.regs.a, operand);
        self.regs.f = f;
        4
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

    fn jp_n(&mut self) -> u8 {
        self.jp_cc(true)
    }

    fn jp_z(&mut self) -> u8 {
        self.jp_cc(self.regs.f & ZERO == ZERO)
    }

    fn jp_nz(&mut self) -> u8 {
        self.jp_cc(self.regs.f & ZERO != ZERO)
    }

    fn jp_c(&mut self) -> u8 {
        self.jp_cc(self.regs.f & CARRY == CARRY)
    }

    fn jp_nc(&mut self) -> u8 {
        self.jp_cc(self.regs.f & CARRY != CARRY)
    }

    fn jp_cc(&mut self, condition: bool) -> u8 {
        if condition {
            self.regs.pc = self.mmu.read_word(self.regs.pc);
            return 16;
        }
        12
    }

    fn jp_hl(&mut self) -> u8 {
        self.regs.pc = self.regs.get_hl();
        4
    }

    fn call_n(&mut self) -> u8 {
        self.call_cc(true)
    }

    fn call_z(&mut self) -> u8 {
        self.call_cc(self.regs.f & ZERO == ZERO)
    }

    fn call_nz(&mut self) -> u8 {
        self.call_cc(self.regs.f & ZERO != ZERO)
    }

    fn call_c(&mut self) -> u8 {
        self.call_cc(self.regs.f & CARRY == CARRY)
    }

    fn call_nc(&mut self) -> u8 {
        self.call_cc(self.regs.f & CARRY != CARRY)
    }

    fn call_cc(&mut self, condition: bool) -> u8 {
        if condition {
            self.regs.sp -= 2;
            self.mmu.write_word(self.regs.sp, self.regs.pc + 2);
            self.regs.pc = self.mmu.read_word(self.regs.pc);
            return 24;
        }
        12
    }

    fn rst_0x00(&mut self) -> u8 {
        self.rst(0x00)
    }

    fn rst_0x08(&mut self) -> u8 {
        self.rst(0x08)
    }

    fn rst_0x10(&mut self) -> u8 {
        self.rst(0x10)
    }

    fn rst_0x18(&mut self) -> u8 {
        self.rst(0x18)
    }

    fn rst_0x20(&mut self) -> u8 {
        self.rst(0x00)
    }

    fn rst_0x28(&mut self) -> u8 {
        self.rst(0x28)
    }

    fn rst_0x30(&mut self) -> u8 {
        self.rst(0x30)
    }

    fn rst_0x38(&mut self) -> u8 {
        self.rst(0x38)
    }

    fn ret_n(&mut self) -> u8 {
        self.ret_cc(true);
        16
    }

    fn ret_z(&mut self) -> u8 {
        self.ret_cc(self.regs.f & ZERO == ZERO)
    }

    fn ret_nz(&mut self) -> u8 {
        self.ret_cc(self.regs.f & ZERO != ZERO)
    }

    fn ret_c(&mut self) -> u8 {
        self.ret_cc(self.regs.f & CARRY == CARRY)
    }

    fn ret_nc(&mut self) -> u8 {
        self.ret_cc(self.regs.f & CARRY != CARRY)
    }

    fn ret_cc(&mut self, condition: bool) -> u8 {
        if condition {
            self.regs.pc = self.mmu.read_word(self.regs.sp);
            self.regs.sp += 2;
            return 20;
        }
        8
    }

    fn ret_i(&mut self) -> u8 {
        self.enabling_interrupts = true;
        self.ret_n()
    }

    fn rst(&mut self, addr: u16) -> u8 {
        self.regs.sp -= 2;
        self.mmu.write_word(self.regs.pc, self.regs.pc + 1);
        self.regs.pc = addr;
        32
    }

    fn push_af(&mut self) -> u8 {
        self.regs.sp -= 2;
        self.mmu.write_word(self.regs.sp, self.regs.get_af());
        16
    }

    fn push_bc(&mut self) -> u8 {
        self.regs.sp -= 2;
        self.mmu.write_word(self.regs.sp, self.regs.get_bc());
        16
    }

    fn push_de(&mut self) -> u8 {
        self.regs.sp -= 2;
        self.mmu.write_word(self.regs.sp, self.regs.get_de());
        16
    }

    fn push_hl(&mut self) -> u8 {
        self.regs.sp -= 2;
        self.mmu.write_word(self.regs.sp, self.regs.get_hl());
        16
    }

    fn pop_af(&mut self) -> u8 {
        self.regs.set_af(self.mmu.read_word(self.regs.sp));
        12
    }

    fn pop_bc(&mut self) -> u8 {
        self.regs.set_bc(self.mmu.read_word(self.regs.sp));
        12
    }

    fn pop_de(&mut self) -> u8 {
        self.regs.set_de(self.mmu.read_word(self.regs.sp));
        12
    }

    fn pop_hl(&mut self) -> u8 {
        self.regs.set_hl(self.mmu.read_word(self.regs.sp));
        12
    }

    fn ei(&mut self) -> u8 {
        self.enabling_interrupts = true;
        4
    }

    fn di(&mut self) -> u8 {
        self.interrupts_enabled = false;
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
