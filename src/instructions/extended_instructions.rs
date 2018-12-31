impl GameBoy {
    fn execute_cb(&mut self) -> usize {
        let op_code = self.fetch_byte();
        match op_code {
            _ => 0
        }
    }
}
