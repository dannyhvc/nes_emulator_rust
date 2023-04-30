
// this is a big boy struct
#[derive(Debug)]
pub struct PPU2C02 {
    table_name: [[u8; 1024]; 2],
    table_pattern: [[u8; 4096]; 2],
    table_palette: [u8; 32],
    scan_line: u16,
    cycle: u16,
}
impl PPU2C02 {}
