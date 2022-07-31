/// # Flags_6502
///* C = Carry flag
///* Z = Zero flag
///* I = Interrupt mask
///* D = Decimal flag
///* B = Break flag
///* U = Unused flag
///* V = Overflow flag
///* N = Negative flag
#[derive(Debug, Clone, Copy)]
pub enum Flags_6502 {
    C = (1 << 0x0), // Carry flag
    Z = (1 << 0x1), // Zero flag
    I = (1 << 0x2), // Interrupt mask
    D = (1 << 0x3), // Decimal flag
    B = (1 << 0x4), // Break flag
    U = (1 << 0x5), // Unused flag
    V = (1 << 0x6), // Overflow flag
    N = (1 << 0x7), // Negative flag
}
