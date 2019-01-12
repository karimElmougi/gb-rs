use crate::interrupts::write_interrupt;
use crate::interrupts::InterruptFlag;
use crate::mmu::MMU;
use img::{ImageBuffer, Rgba, RgbaImage};

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;

const GPU_CTRL_ADDR: u16 = 0xff40;
const LCD_STATUS_ADDR: u16 = 0xff41;
const SCANLINE_ADDR: u16 = 0xff44;

const PALETTE: [Rgba<u8>; 4] = [
    Rgba {
        data: [255, 255, 255, 255],
    },
    Rgba {
        data: [192, 192, 192, 255],
    },
    Rgba {
        data: [96, 96, 96, 255],
    },
    Rgba {
        data: [0, 0, 0, 255],
    },
];

#[derive(Copy, Clone, PartialEq)]
enum GPUMode {
    HBLANK = 0x00,
    VBLANK = 0x01,
    OAM = 0x02,
    VRAM = 0x03,
}

impl GPUMode {
    fn from(n: u8) -> GPUMode {
        match n {
            0x00 => GPUMode::HBLANK,
            0x01 => GPUMode::VBLANK,
            0x02 => GPUMode::OAM,
            0x03 => GPUMode::VRAM,
            _ => panic!("Invalid GPUMode value: {}", n),
        }
    }
}

pub struct GPU {
    clock: i32,
    framebuffer: RgbaImage,
}

pub fn new() -> GPU {
    GPU {
        clock: 456,
        framebuffer: ImageBuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT),
    }
}

impl GPU {
    pub fn step(&mut self, mmu: &mut MMU, cycles: i32) -> Option<RgbaImage> {
        if !is_lcd_on(mmu) {
            self.clock = 456;
            mmu.write_byte(SCANLINE_ADDR, 0);
            set_mode(mmu, GPUMode::HBLANK);
            return None;
        }

        let scanline = mmu.read_byte(SCANLINE_ADDR);
        let previous_mode = get_mode(mmu);

        let request_interrupt = if scanline >= 144 {
            set_mode(mmu, GPUMode::VBLANK);
            is_vblank_mode_interrupt_enabled(mmu)
        } else if self.clock >= 456 - 80 {
            set_mode(mmu, GPUMode::OAM);
            is_oam_mode_interrupt_enabled(mmu)
        } else if self.clock >= 456 - 80 - 172 {
            set_mode(mmu, GPUMode::VRAM);
            false
        } else {
            set_mode(mmu, GPUMode::HBLANK);
            is_hblank_mode_interrupt_enabled(mmu)
        };

        if request_interrupt && get_mode(mmu) != previous_mode {
            write_interrupt(mmu, InterruptFlag::LCD);
        }

        if scanline == mmu.read_byte(0xff45) {
            set_coincidence_status(mmu);
            if is_coincidence_interrupt_enabled(mmu) {
                write_interrupt(mmu, InterruptFlag::LCD);
            }
        } else {
            reset_coincidence_status(mmu);
        }

        let mut frame = None;
        self.clock -= cycles;
        if self.clock <= 0 {
            self.clock += 456;
            let scanline = &mut mmu.memory[SCANLINE_ADDR as usize];
            *scanline += 1;
            let scanline = *scanline;
            if scanline == 144 {
                write_interrupt(mmu, InterruptFlag::VBLANK);
            } else if scanline > 153 {
                frame = Some(self.framebuffer.clone());
                mmu.write_byte(SCANLINE_ADDR, 0);
                self.render_scanline(mmu, 0);
            } else if scanline < 144 {
                self.render_scanline(mmu, scanline);
            }
        }
        frame
    }

    fn render_scanline(&mut self, mmu: &mut MMU, current_line: u8) {
        let control = mmu.read_byte(GPU_CTRL_ADDR);
        if control & 0x01 == 0x01 {
            self.render_tiles(mmu, control, current_line);
        }
        if control & 0x02 == 0x02 {
            self.render_sprites(mmu, control, current_line);
        }
    }

    fn render_tiles(&mut self, mmu: &MMU, control: u8, current_line: u8) {
        let scroll_y = mmu.read_byte(0xff42);
        let scroll_x = mmu.read_byte(0xff43);
        let window_y = mmu.read_byte(0xff4a);
        let window_x = mmu.read_byte(0xff4b) - 7;

        let using_window = if control & 0x20 == 0x20 && window_y <= mmu.read_byte(SCANLINE_ADDR) {
            true
        } else {
            false
        };

        let (tile_data, unsig) = if control & 0x10 == 0x10 {
            (0x8000, true)
        } else {
            (0x8800, false)
        };

        let test_mask = if using_window { 0x40 } else { 0x08 };

        let background_mem = if control & test_mask == test_mask {
            0x9c00
        } else {
            0x9800
        };

        let y_pos = if using_window {
            current_line - window_y
        } else {
            scroll_y + current_line
        };

        let tile_row = (y_pos / 8) as u16 * 32;

        for pixel in 0..(SCREEN_WIDTH as u8) {
            let x_pos = if using_window && pixel >= window_x {
                pixel - window_x
            } else {
                pixel + scroll_x
            };

            let tile_col = (x_pos / 8) as u16;
            let tile_addr = background_mem + tile_row + tile_col;
            let tile_location = if unsig {
                tile_data + 16 * (mmu.memory[tile_addr as usize] as u16)
            } else {
                let tile_num = (mmu.memory[tile_addr as usize] as i8) as i16;
                (tile_data as i32 + (16 * (tile_num + 128)) as i32) as u16
            };

            let line = (2 * (y_pos % 8)) as u16;
            let data1 = mmu.memory[(tile_location + line) as usize];
            let data2 = mmu.memory[(tile_location + line + 1) as usize];

            let color_bit = ((((x_pos % 8) - 7) as i8) * -1) as u8;
            let color_num = (((data2 >> color_bit) & 1) << 1) | ((data1 >> color_bit) & 1);
            let color = get_color(mmu, color_num, 0xff47);
            self.framebuffer
                .put_pixel(pixel as u32, current_line as u32, color);
        }
    }

    fn render_sprites(&mut self, mmu: &MMU, control: u8, current_line: u8) {
        let current_line = current_line as i32;
        let y_size = if control & 0x04 == 0x04 { 16 } else { 8 };

        for sprite in 0..40 {
            let index = sprite * 4;
            let y_pos = (mmu.read_byte(0xfe00 + index)) as i32 - 16;
            let x_pos = mmu.read_byte(0xfe00 + index + 1) - 8;
            let tile_location = mmu.read_byte(0xfe00 + index + 2);
            let attributes = mmu.read_byte(0xfe00 + index + 3);

            if current_line < y_pos || current_line >= (y_pos + y_size) {
                continue;
            }

            let line = if attributes & 0x40 == 0x40 {
                -((current_line - y_pos) - y_size)
            } else {
                current_line - y_pos
            };

            let data_addr = 16 * (tile_location as u16) + (2 * line) as u16 + 0x8000;
            let data1 = mmu.read_byte(data_addr);
            let data2 = mmu.read_byte(data_addr + 1);

            for tile in 0..8 {
                let color_bit = if attributes & 0x20 == 0x20 {
                    (-((tile - 7) as i8)) as u8
                } else {
                    tile
                };

                let color_num = (((data2 >> color_bit) & 1) << 1) | ((data1 >> color_bit) & 1);

                if color_num == 0 {
                    continue;
                }

                let pixel = x_pos as u32 + (7 - tile) as u32;
                if pixel < 160 {
                    let priority = attributes & 0x80 != 0x80;
                    let bg_tile_color = *self.framebuffer.get_pixel(pixel, current_line as u32);
                    let background_is_white = bg_tile_color == PALETTE[0];
                    if priority || background_is_white {
                        let palette_addr = if attributes & 0x10 == 0x10 {
                            0xff49
                        } else {
                            0xff48
                        };
                        let color = get_color(mmu, color_num, palette_addr);
                        self.framebuffer
                            .put_pixel(pixel, current_line as u32, color);
                    }
                }
            }
        }
    }
}

fn is_hblank_mode_interrupt_enabled(mmu: &MMU) -> bool {
    mmu.read_byte(LCD_STATUS_ADDR) & 0x08 == 0x08
}

fn is_vblank_mode_interrupt_enabled(mmu: &MMU) -> bool {
    mmu.read_byte(LCD_STATUS_ADDR) & 0x10 == 0x10
}

fn is_oam_mode_interrupt_enabled(mmu: &MMU) -> bool {
    mmu.read_byte(LCD_STATUS_ADDR) & 0x20 == 0x20
}

fn is_coincidence_interrupt_enabled(mmu: &MMU) -> bool {
    mmu.read_byte(LCD_STATUS_ADDR) & 0x40 == 0x40
}

fn get_mode(mmu: &MMU) -> GPUMode {
    GPUMode::from(mmu.read_byte(LCD_STATUS_ADDR) & 0x03)
}

fn set_mode(mmu: &mut MMU, mode: GPUMode) {
    let status = mmu.read_byte(LCD_STATUS_ADDR) & (!0x03);
    mmu.write_byte(LCD_STATUS_ADDR, status | (mode as u8));
}

fn set_coincidence_status(mmu: &mut MMU) {
    mmu.write_byte(LCD_STATUS_ADDR, mmu.read_byte(LCD_STATUS_ADDR) | 0x04);
}

fn reset_coincidence_status(mmu: &mut MMU) {
    mmu.write_byte(LCD_STATUS_ADDR, mmu.read_byte(LCD_STATUS_ADDR) | (!0x04));
}

fn is_lcd_on(mmu: &MMU) -> bool {
    mmu.read_byte(GPU_CTRL_ADDR) & 0x80 == 0x80
}

fn get_color(mmu: &MMU, colour_num: u8, addr: u16) -> Rgba<u8> {
    let custom_palette = mmu.read_byte(addr);
    let i = match colour_num {
        0 => custom_palette & 0x02 | custom_palette & 0x01,
        1 => ((custom_palette & 0x08) >> 2) | ((custom_palette & 0x04) >> 2),
        2 => ((custom_palette & 0x20) >> 4) | ((custom_palette & 0x10) >> 4),
        3 => ((custom_palette & 0x80) >> 6) | ((custom_palette & 0x40) >> 6),
        _ => panic!("Invalid colour value: {}", colour_num),
    };
    PALETTE[i as usize]
}
