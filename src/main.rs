mod cartridge;
mod mmu;
mod gpu;
mod cpu;

fn main() {
    let rom_name = "";
    let cartridge = cartridge::new(rom_name);
    let mut mmu = mmu::new(Box::new(cartridge));
    let gpu = gpu::new(&mut mmu);
    let cpu = cpu::new(mmu);
}
