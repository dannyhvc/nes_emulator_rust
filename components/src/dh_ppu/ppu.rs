// PPU.rs
use crate::{dh_cartridge::Cartridge, KiB};

// Helper constants for screen and debug table dimensions
pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;
pub const PATTERN_TABLE_WIDTH: usize = 128;
pub const PATTERN_TABLE_HEIGHT: usize = 128;

/* ============================ StatusRegister ============================ */
#[derive(Copy, Clone, Default, Debug)]
pub struct StatusRegister(pub u8);

impl StatusRegister {
    // Bits 0-4 are unused in the actual NES hardware

    pub fn sprite_overflow(&self) -> bool {
        (self.0 & (1 << 5)) != 0
    }
    pub fn set_sprite_overflow(&mut self, v: bool) {
        self.0 = if v {
            self.0 | (1 << 5)
        } else {
            self.0 & !(1 << 5)
        };
    }

    pub fn sprite_zero_hit(&self) -> bool {
        (self.0 & (1 << 6)) != 0
    }
    pub fn set_sprite_zero_hit(&mut self, v: bool) {
        self.0 = if v {
            self.0 | (1 << 6)
        } else {
            self.0 & !(1 << 6)
        };
    }

    pub fn vertical_blank(&self) -> bool {
        (self.0 & (1 << 7)) != 0
    }
    pub fn set_vertical_blank(&mut self, v: bool) {
        self.0 = if v {
            self.0 | (1 << 7)
        } else {
            self.0 & !(1 << 7)
        };
    }
}

/* ============================= MaskRegister ============================= */
#[derive(Copy, Clone, Default, Debug)]
pub struct MaskRegister(pub u8);

impl MaskRegister {
    #[inline]
    fn get_bit(&self, bit: u8) -> bool {
        (self.0 & (1 << bit)) != 0
    }
    #[inline]
    fn set_bit(&mut self, bit: u8, v: bool) {
        self.0 = if v {
            self.0 | (1 << bit)
        } else {
            self.0 & !(1 << bit)
        };
    }

    pub fn grayscale(&self) -> bool {
        self.get_bit(0)
    }
    pub fn set_grayscale(&mut self, v: bool) {
        self.set_bit(0, v)
    }

    pub fn render_background_left(&self) -> bool {
        self.get_bit(1)
    }
    pub fn set_render_background_left(&mut self, v: bool) {
        self.set_bit(1, v)
    }

    pub fn render_sprites_left(&self) -> bool {
        self.get_bit(2)
    }
    pub fn set_render_sprites_left(&mut self, v: bool) {
        self.set_bit(2, v)
    }

    pub fn render_background(&self) -> bool {
        self.get_bit(3)
    }
    pub fn set_render_background(&mut self, v: bool) {
        self.set_bit(3, v)
    }

    pub fn render_sprites(&self) -> bool {
        self.get_bit(4)
    }
    pub fn set_render_sprites(&mut self, v: bool) {
        self.set_bit(4, v)
    }

    pub fn enhance_red(&self) -> bool {
        self.get_bit(5)
    }
    pub fn set_enhance_red(&mut self, v: bool) {
        self.set_bit(5, v)
    }

    pub fn enhance_green(&self) -> bool {
        self.get_bit(6)
    }
    pub fn set_enhance_green(&mut self, v: bool) {
        self.set_bit(6, v)
    }

    pub fn enhance_blue(&self) -> bool {
        self.get_bit(7)
    }
    pub fn set_enhance_blue(&mut self, v: bool) {
        self.set_bit(7, v)
    }
}

/* ============================ ControlRegister =========================== */
#[derive(Copy, Clone, Default, Debug)]
pub struct ControlRegister(pub u8);

impl ControlRegister {
    #[inline]
    fn get_bit(&self, bit: u8) -> bool {
        (self.0 & (1 << bit)) != 0
    }
    #[inline]
    fn set_bit(&mut self, bit: u8, v: bool) {
        self.0 = if v {
            self.0 | (1 << bit)
        } else {
            self.0 & !(1 << bit)
        };
    }

    pub fn nametable_x(&self) -> bool {
        self.get_bit(0)
    }
    pub fn set_nametable_x(&mut self, v: bool) {
        self.set_bit(0, v)
    }

    pub fn nametable_y(&self) -> bool {
        self.get_bit(1)
    }
    pub fn set_nametable_y(&mut self, v: bool) {
        self.set_bit(1, v)
    }

    pub fn increment_mode(&self) -> bool {
        self.get_bit(2)
    }
    pub fn set_increment_mode(&mut self, v: bool) {
        self.set_bit(2, v)
    }

    pub fn pattern_sprite(&self) -> bool {
        self.get_bit(3)
    }
    pub fn set_pattern_sprite(&mut self, v: bool) {
        self.set_bit(3, v)
    }

    pub fn pattern_background(&self) -> bool {
        self.get_bit(4)
    }
    pub fn set_pattern_background(&mut self, v: bool) {
        self.set_bit(4, v)
    }

    pub fn sprite_size(&self) -> bool {
        self.get_bit(5)
    }
    pub fn set_sprite_size(&mut self, v: bool) {
        self.set_bit(5, v)
    }

    pub fn slave_mode(&self) -> bool {
        self.get_bit(6)
    }
    pub fn set_slave_mode(&mut self, v: bool) {
        self.set_bit(6, v)
    }

    pub fn enable_nmi(&self) -> bool {
        self.get_bit(7)
    }
    pub fn set_enable_nmi(&mut self, v: bool) {
        self.set_bit(7, v)
    }
}

/* ============================= LoopRegister ============================= */
#[derive(Copy, Clone, Default, Debug)]
pub struct LoopyRegister(pub u16);

impl LoopyRegister {
    // coarse_x: 5 bits
    pub fn coarse_x(&self) -> u16 {
        self.0 & 0x001F
    }
    pub fn set_coarse_x(&mut self, v: u16) {
        self.0 = (self.0 & !0x001F) | (v & 0x001F);
    }

    // coarse_y: 5 bits (starts at bit 5)
    pub fn coarse_y(&self) -> u16 {
        (self.0 >> 5) & 0x001F
    }
    pub fn set_coarse_y(&mut self, v: u16) {
        self.0 = (self.0 & !(0x001F << 5)) | ((v & 0x001F) << 5);
    }

    // nametable_x: 1 bit (starts at bit 10)
    pub fn nametable_x(&self) -> bool {
        ((self.0 >> 10) & 0x0001) != 0
    }
    pub fn set_nametable_x(&mut self, v: bool) {
        self.0 = (self.0 & !(0x0001 << 10)) | ((v as u16) << 10);
    }

    // nametable_y: 1 bit (starts at bit 11)
    pub fn nametable_y(&self) -> bool {
        ((self.0 >> 11) & 0x0001) != 0
    }
    pub fn set_nametable_y(&mut self, v: bool) {
        self.0 = (self.0 & !(0x0001 << 11)) | ((v as u16) << 11);
    }

    // fine_y: 3 bits (starts at bit 12)
    pub fn fine_y(&self) -> u16 {
        (self.0 >> 12) & 0x0007
    }
    pub fn set_fine_y(&mut self, v: u16) {
        self.0 = (self.0 & !(0x0007 << 12)) | ((v & 0x0007) << 12);
    }
}


/* ================================= PPU ================================= */
#[derive(Debug)]
pub struct PPU {
    table_name: [[u8; KiB(1)]; 2],    // 2* 1KiB
    table_pattern: [[u8; KiB(4)]; 2], // 2* 4KiB
    table_palette: [u8; 32],
    cycle: u16,

    // RENDERING BUFFERS FOR MINIFB
    // We use raw 1D arrays of 32-bit colors (0x00RRGGBB)
    pub screen_buffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],

    // Debug output buffers (optional: use these if you want to render the raw pattern tables to separate windows)
    pub debug_pattern_tables:
        [[u32; PATTERN_TABLE_WIDTH * PATTERN_TABLE_HEIGHT]; 2],
    pub debug_name_tables: [[u32; SCREEN_WIDTH * SCREEN_HEIGHT]; 2],

    pub frame_complete: bool,
    pub nmi: bool,

    // Internal Registers (assume structs are implemented as previously discussed)
    status: StatusRegister,
    mask: MaskRegister,
    control: ControlRegister,
    vram_addr: LoopyRegister,
    tram_addr: LoopyRegister,

    fine_x: u8,
    address_latch: u8,
    ppu_data_buffer: u8,

    scanline: u16,
    bg_next_tile_id: u8,
    bg_next_tile_attrib: u8,
    bg_next_tile_lsb: u8,
    bg_next_tile_msb: u8,
    bg_shifter_pattern_lo: u16,
    bg_shifter_pattern_hi: u16,
    bg_shifter_attrib_lo: u16,
    bg_shifter_attrib_hi: u16,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            table_name: [[0; KiB(1)]; 2],
            table_pattern: [[0; KiB(4)]; 2],
            table_palette: [0; 32],

            // Initialize all buffers to black (0x00000000)
            screen_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            debug_pattern_tables: [[0; 128 * 128]; 2],
            debug_name_tables: [[0; 256 * 240]; 2],

            frame_complete: false,
            nmi: false,

            status: StatusRegister(0),
            mask: MaskRegister(0),
            control: ControlRegister(0),
            vram_addr: LoopyRegister(0),
            tram_addr: LoopyRegister(0),

            // buffers
            fine_x: 0,
            address_latch: 0,
            ppu_data_buffer: 0,
            scanline: 0,
            cycle: 0,
            bg_next_tile_id: 0,
            bg_next_tile_attrib: 0,
            bg_next_tile_lsb: 0,
            bg_next_tile_msb: 0,
            bg_shifter_pattern_lo: 0,
            bg_shifter_pattern_hi: 0,
            bg_shifter_attrib_lo: 0,
            bg_shifter_attrib_hi: 0,
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, colour: u32) {
        if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
            self.screen_buffer[y * SCREEN_WIDTH + x] = colour;
        }
    }

    pub fn clock(&mut self, cart: &mut Cartridge) {
        if self.scanline < 0xF0 && self.cycle <= 256 {
            let colour = 0x00_FF_0000; // example
            self.draw_pixel(
                (self.cycle - 1) as usize,
                self.scanline as usize,
                colour,
            );
        }

        self.cycle += 1;

        if self.scanline == 241 && self.cycle == 1 {
            self.frame_complete = true;
        }
    }
}
