extern crate image;

use crate::cartridge;
use crate::cpu;
use crate::gpu;
use crate::interrupts::isr;
use crate::mmu;
use image::RgbaImage;

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
    pub fn step(&mut self) -> Option<RgbaImage> {
        let mut frame = None;
        let mut cycles = 0;
        while cycles < CYCLES_PER_SECOND {
            let step_cycles = self.cpu.step(&mut self.mmu);
            self.mmu.increment_counters(step_cycles as i32);
            isr(&mut self.cpu, &mut self.mmu);
            frame = self.gpu.step(&mut self.mmu, step_cycles as i32);
            cycles += step_cycles as i32;
        }
        frame
    }
}
