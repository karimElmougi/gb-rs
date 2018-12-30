use crate::cartridge;
use crate::mmu;
use crate::gpu;
use crate::cpu;
use crate::interrupts;

const CYCLES_PER_SECOND: i32 = 4194304 / 60;

pub struct Gameboy {
    mmu: mmu::MMU,
    cpu: cpu::CPU,
    gpu: gpu::GPU
}

pub fn new(file_name: &str) -> Gameboy {
    let mmu = mmu::new(Box::new(cartridge::new(file_name)));
    let cpu = cpu::new();
    let gpu = gpu::new();
    Gameboy {
        mmu,
        cpu, 
        gpu
    }
}

impl Gameboy {
    pub fn step(&mut self) {
        let mut cycles_ellapsed = 0usize;
        for _ in (0..CYCLES_PER_SECOND).step_by(cycles_ellapsed) {
            cycles_ellapsed = self.cpu.step(&mut self.mmu);
            self.mmu.increment_counters(cycles_ellapsed as i32);
            self.gpu.step(cycles_ellapsed as i32);
            interrupts::isr(&mut self.cpu, &mut self.mmu);
        }
    }
}
