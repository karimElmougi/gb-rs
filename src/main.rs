mod cartridge;
mod gameboy;
mod gpu;
mod mmu;
mod registers;

fn main() {
    let rom_name = "";
    let mut gameboy = gameboy::new(rom_name);
    gameboy.step();
}
