struct Instruction {
    _name: &'static str,
    f: fn(&mut GameBoy) -> u8
}

const INSTRUCTIONS: [Instruction; 4] = [
    Instruction {_name: "NOP", f: GameBoy::nop},                         // 0x01
    Instruction {_name: "LD BC, NN", f: GameBoy::ld_bc_nn},              // 0x02
    Instruction {_name: "LD (BC), A", f: GameBoy::ld_bc_a},              // 0x02
    Instruction {_name: "INC BC", f: inc_bc},                   // 0x03
    // Instruction {_name: "CB N", f: GameBoy::execute_cb},        // 0xcd
];

impl GameBoy {
    fn nop(&mut self) -> u8 {
        4
    }

    fn ld_bc_nn(&mut self) -> u8 {
        let n = self.fetch_word();
        self.regs.set_bc(n);
        12
    }

    fn ld_bc_a(&mut self) -> u8 {
        self.mmu.write_byte(self.regs.get_bc(), self.regs.a);
        8
    }
}


fn inc_bc(gb: &mut GameBoy) -> u8 {
    let n = gb.fetch_word();
    gb.regs.set_bc(n);
    8
}
