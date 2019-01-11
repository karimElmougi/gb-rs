mod cartridge;
mod cpu;
mod gpu;
mod instructions;
mod interrupts;
mod mmu;
mod registers;

extern crate image as img;
extern crate piston;
extern crate piston_window;

use img::ImageBuffer;
use piston::event_loop::EventLoop;
use piston_window::{
    clear, image, Filter, OpenGL, PistonWindow, RenderEvent, Texture, TextureSettings,
    WindowSettings,
};

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

    let scale = 1;

    let mut mmu = mmu::new(cartridge::new(rom_name));
    let mut cpu = cpu::new();
    let mut gpu = gpu::new();

    let (width, height) = (scale * gpu::SCREEN_WIDTH, scale * gpu::SCREEN_HEIGHT);

    // update opengl?
    let mut window: PistonWindow = WindowSettings::new("gb-rs", (width, height))
        .opengl(OpenGL::V3_2)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_ups(60);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let mut frame = ImageBuffer::new(width, height);
    let mut texture = Texture::from_image(&mut window.factory, &frame, &texture_settings).unwrap();

    while let Some(e) = window.next() {
        let mut cycles = 0;
        while cycles < CYCLES_PER_SECOND {
            let step_cycles = cpu.step(&mut mmu);
            mmu.increment_counters(step_cycles as i32);
            isr(&mut cpu, &mut mmu);
            if let Some(img) = gpu.step(&mut mmu, step_cycles as i32) {
                frame = img;
            }
            cycles += step_cycles as i32;
        }

        if let Some(_) = e.render_args() {
            texture.update(&mut window.encoder, &frame).unwrap();
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
    }
}
