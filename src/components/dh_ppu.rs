use super::KB;

// this is a big boy struct
#[derive(Debug)]
pub struct PPU {
    table_name: [[u8; KB(1)]; 2],    // 2* 1KB
    table_pattern: [[u8; KB(4)]; 2], // 2* 4KB
    table_palette: [u8; 32],
    scan_line: u16,
    cycle: u16,
}
