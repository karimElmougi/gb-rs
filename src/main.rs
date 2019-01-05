mod cartridge;
mod cpu;
mod gameboy;
mod gpu;
mod interrupts;
mod mmu;
mod registers;

fn main() {
    let rom_name = "";
    let mut gameboy = gameboy::new(rom_name);
    gameboy.step();
}
