fn execute_cb(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let _op_code = cpu.fetch_byte(mmu);
    0
}
