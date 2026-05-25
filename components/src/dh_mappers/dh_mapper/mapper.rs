// trivially copiable struct on most systems: mov QWORD
#[derive(Debug, Clone, Copy, Default)]
pub struct Mapper {
    pub chr_bank: u8,
    pub prg_bank: u8,
}
