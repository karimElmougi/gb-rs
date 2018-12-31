impl GameBoy {
    fn execute(&mut self, op_code: u8) -> usize {
        match op_code {
            0x00 => { 4 },
            0x01 => { let v = self.fetch_word(); self.regs.set_bc(v); 12 },
            0x02 => { let addr = self.regs.get_bc(); self.mmu.write_byte(addr, self.regs.a); 8 }
            0x03 => { self.regs.set_bc(self.regs.get_bc().wrapping_add(1)); 8 },
            0x04 => { 0 },
            0xcb => self.execute_cb(),
            _ => { println!("Unimplemented instruction: 0x{:x}", op_code); 0 }
        }
    }
}
