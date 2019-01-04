impl GameBoy {
    fn execute_cb(&mut self) -> u8 {
        let _op_code = self.fetch_byte();
        0
    }
}
