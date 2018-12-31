mod cartridge;
mod mmu;
mod gpu;
mod registers;
mod gameboy;

fn main() {
    let rom_name = "";
    let mut gameboy = gameboy::new(rom_name);
    gameboy.step();
}
