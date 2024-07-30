#[derive(Debug, PartialEq, Eq)]
pub enum AddrModeMneumonic {
    IMP,
    IMM,
    ZP0,
    ZPX,
    ZPY,
    ABS,
    ABX,
    ABY,
    REL,
    IND,
    IZX,
    IZY,
}
