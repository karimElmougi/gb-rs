mod cartridge;
mod cpu;
mod gpu;
mod instructions;
mod interrupts;
mod mmu;
mod registers;

extern crate sdl2;

use crate::interrupts::isr;

const CYCLES_PER_SECOND: i32 = 4194304 / 60;

fn main() {
    let rom_name = "Pokemon Red.gb";
    // let rom_name = "cpu_instrs\\individual\\01-special.gb";
    // let rom_name = "cpu_instrs\\individual\\02-interrupts.gb";
    // let rom_name = "cpu_instrs\\individual\\03-op sp,hl.gb";
    // let rom_name = "cpu_instrs\\individual\\04-op r,imm.gb";
    // let rom_name = "cpu_instrs\\individual\\05-op rp.gb";
    // let rom_name = "cpu_instrs\\individual\\06-ld r,r.gb";
    // let rom_name = "cpu_instrs\\individual\\07-jr,jp,call,ret,rst.gb";
    // let rom_name = "cpu_instrs\\individual\\08-misc instrs.gb";
    // let rom_name = "cpu_instrs\\individual\\09-op r,r.gb";
    // let rom_name = "cpu_instrs\\individual\\10-bit ops.gb";
    
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let window = video_ctx.window("gb-rs", gpu::SCREEN_WIDTH, gpu::SCREEN_HEIGHT).position_centered().opengl().build();
    let window = match window {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let canvas = window.into_canvas().build().unwrap();

    let mut mmu = mmu::new(cartridge::new(rom_name));
    let mut cpu = cpu::new();
    let mut gpu = gpu::new(canvas);
    loop {
        let mut cycles = 0;
        while cycles < CYCLES_PER_SECOND {
            let step_cycles = cpu.step(&mut mmu);
            mmu.increment_counters(step_cycles as i32);
            isr(&mut cpu, &mut mmu);
            gpu.step(&mut mmu, step_cycles as i32);
            cycles += step_cycles as i32;
        }
        std::thread::sleep(std::time::Duration::from_micros(16666));
    }
}
