use crate::cartridge;
use crate::cpu;
use crate::gpu;
use crate::interrupts::isr;
use crate::mmu;

const CYCLES_PER_SECOND: i32 = 4194304 / 60;

pub struct GameBoy {
    cpu: cpu::CPU,
    mmu: mmu::MMU,
    gpu: gpu::GPU,
}

pub fn new(file_name: &str) -> GameBoy {
    let mmu = mmu::new(cartridge::new(file_name));
    let cpu = cpu::new();
    let gpu = gpu::new();
    GameBoy { cpu, mmu, gpu }
}

impl GameBoy {
    pub fn step(&mut self) {
        let mut cycles_ellapsed = self.cpu.step(&mut self.mmu);
        for _ in (0..CYCLES_PER_SECOND).step_by(cycles_ellapsed as usize) {
            cycles_ellapsed = self.cpu.step(&mut self.mmu);
            self.mmu.increment_counters(cycles_ellapsed as i32);
            self.gpu.step(cycles_ellapsed as i32);
            isr(&mut self.cpu, &mut self.mmu);
        }
    }
}
