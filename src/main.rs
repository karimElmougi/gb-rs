mod cartridge;
mod mmu;
mod gpu;
mod cpu;
mod interrupts;
mod gameboy;

fn main() {
    let rom_name = "";
    let mut gameboy = gameboy::new(rom_name);
    gameboy.step();
}
