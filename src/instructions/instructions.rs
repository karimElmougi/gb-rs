const CARRY: u8 = 0b0001_0000;
const HALF_CARRY: u8 = 0b0010_0000;
const SUB: u8 = 0b0100_0000;
const ZERO: u8 = 0b1000_0000;

const INSTRUCTIONS: [(&'static str, fn(&mut GameBoy)->u8); 10] = [
    ("NOP", GameBoy::nop),                         // 0x01
    ("LD BC, NN", GameBoy::ld_bc_nn),              // 0x02
    ("LD (BC), A", GameBoy::ld_bc_a),              // 0x02
    ("INC BC", GameBoy::inc_bc),                   // 0x03
    ("INC B", GameBoy::inc_b),                   // 0x04
    ("DEC B", GameBoy::dec_b),                   // 0x05
    ("LD B, N", GameBoy::ld_b_n),                   // 0x06
    ("RLCA", GameBoy::rlca),                   // 0x07
    ("LD (NN), SP", GameBoy::ld_nn_sp),                   // 0x08
    ("ADD HL, BC", GameBoy::add_hl_bc),                   // 0x09
    // Instruction {_name: "CB N", f: GameBoy::execute_cb},        // 0xcd
];

impl GameBoy {
    fn nop(&mut self) -> u8 {
        4
    }

    fn halt(&mut self) -> u8 {
        self.is_halted = true;
        4
    }

    fn ld_a_n(&mut self) -> u8 {
        self.regs.a = self.fetch_byte();
        4
    }

    fn ld_b_n(&mut self) -> u8 {
        self.regs.b = self.fetch_byte();
        4
    }

    fn ld_c_n(&mut self) -> u8 {
        self.regs.c = self.fetch_byte();
        4
    }

    fn ld_d_n(&mut self) -> u8 {
        self.regs.d = self.fetch_byte();
        4
    }

    fn ld_e_n(&mut self) -> u8 {
        self.regs.e = self.fetch_byte();
        4
    }

    fn ld_h_n(&mut self) -> u8 {
        self.regs.h = self.fetch_byte();
        4
    }

    fn ld_l_n(&mut self) -> u8 {
        self.regs.l = self.fetch_byte();
        4
    }

    fn ld_bc_nn(&mut self) -> u8 {
        let n = self.fetch_word();
        self.regs.set_bc(n);
        12
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

    fn ld_bc_a(&mut self) -> u8 {
        self.mmu.write_byte(self.regs.get_bc(), self.regs.a);
        8
    }

    fn ld_de_a(&mut self) -> u8 {
        self.mmu.write_byte(self.regs.get_de(), self.regs.a);
        8
    }

    fn ld_nn_sp(&mut self) -> u8 {
        let n = self.fetch_word(); 
        self.mmu.write_word(n, self.regs.sp);
        20
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

    fn inc_bc(&mut self) -> u8 {
        self.regs.set_bc(self.regs.get_bc()+1);
        8
    }

    fn dec_b(&mut self) -> u8 {
        let (b, f) = dec_r(self.regs.b, self.regs.f);
        self.regs.b = b;
        self.regs.f = f;
        4
    }

    fn rlca(&mut self) -> u8 {
        let bit_7 = self.regs.a >> 7;
        self.regs.a = (self.regs.a << 1) | bit_7;
        self.regs.f = if bit_7 == 1 { CARRY } else { 0 };
        4
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
            | if ((op1 & 0xfff) + (op2 & 0xfff)) > 0xfff { HALF_CARRY } else { 0 };
    (r as u16, f)
}

fn add(op1: u8, op2: u8, f: u8) -> (u8, u8) {
    add_impl(op1, op2, f, false)
}

fn addc(op1: u8, op2: u8, f: u8) -> (u8, u8) {
    add_impl(op1, op2, f, true)
}

fn add_impl(op1: u8, op2: u8, f: u8, use_carry: bool) -> (u8, u8) {
    let carry = if use_carry {
        ((f & CARRY) >> 4)
    } else {
        0
    };
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
    let borrow = if use_borrow {
        ((f & CARRY) >> 4)
    } else {
        0
    };
    let r16 = op1 as u16 - op2 as u16 - borrow as u16;
    let r = r16 as u8;
    let f = SUB 
        | if r == 0 { ZERO } else { 0 }
        | if r16 > 0xff { CARRY } else { 0 }
        | if (op1 & 0xf) - (op2 & 0xf) - borrow > 0xf { HALF_CARRY } else { 0 };
    (r,f)
}
