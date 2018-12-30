use crate::mmu::MMU;

pub struct CPU {
    A: u8, 
    B: u8, 
    C: u8, 
    D: u8, 
    E: u8, 
    H: u8, 
    L: u8, 
    F: u8,
    PC: u16, 
    SP: u16,
}

pub fn new() -> CPU {
    CPU {
        A: 1,
        B: 0, 
        C: 19,
        D: 0,
        E: 216,
        H: 1,
        L: 77,
        F: 176,
        PC: 0x100,
        SP: 0xfffe
    }
}

impl CPU {
    pub fn step(&self, mmu: &mut MMU) -> usize {
        if mmu.is_halted {
            4
        } else{
            let op_code = self.fetch_instruction(mmu);
            self.execute(op_code)
        }
    }

    pub fn call(&mut self, mmu: &mut MMU, addr: u16) {
        self.SP -= 2;
        mmu.write_word(self.SP, self.PC);
        self.PC = addr;
    }

    fn fetch_instruction(&self, mmu: &mut MMU) -> u8 {
        mmu.read_byte(self.PC)
    }

    fn execute(&self, op_code: u8) -> usize {
        match op_code {
            _ => 0
        }
    }
}
