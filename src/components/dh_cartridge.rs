use super::mappers::mapper::*;

#[derive(Debug)]
pub enum Mirroring {
    HORIZONTAL,
    VERTICAL,
    ONESCREAN_LO,
    ONESCREAN_HI,
}

#[derive(Debug)]
pub struct Cartrige {
    pub image_valid: bool,
    pub mirror: Mirroring,
    mapper_id: u8,
    prg_banks: u8,
    chr_banks: u8,
    prg_mem: Vec<u8>,
    chr_mem: Vec<u8>,
    mapper: MapperData,
}
impl Cartrige {
    pub fn new() -> Self {
        Self {
            image_valid: false,
            mirror: Mirroring::HORIZONTAL,
            mapper_id: 0u8,
            prg_banks: 0u8,
            chr_banks: 0u8,
            prg_mem: vec![],
            chr_mem: vec![],
            mapper: MapperData::default(),
        }
    }
}
