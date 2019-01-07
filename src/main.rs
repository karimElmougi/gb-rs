mod cartridge;
mod cpu;
mod gameboy;
mod gpu;
mod instructions;
mod interrupts;
mod mmu;
mod registers;

extern crate image;
extern crate piston_window;

use piston_window::*;

fn main() {
    let rom_name = "cpu_instrs\\cpu_instrs.gb";
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
    let mut gb = gameboy::new(rom_name);

    let scale = 2;

    let (width, height) = (scale * gpu::SCREEN_WIDTH, scale * gpu::SCREEN_HEIGHT);
    let mut window: PistonWindow = WindowSettings::new("gb_rs", (width, height))
        .exit_on_esc(true)
        .opengl(OpenGL::V3_2)
        .build()
        .unwrap();

    let mut canvas = image::ImageBuffer::new(width, height);
    let mut texture: G2dTexture =
        Texture::from_image(&mut window.factory, &canvas, &TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        if let Some(img) = gb.step() {
            canvas = img;
        }

        if let Some(_) = e.render_args() {
            texture.update(&mut window.encoder, &canvas).unwrap();
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
    }
}
