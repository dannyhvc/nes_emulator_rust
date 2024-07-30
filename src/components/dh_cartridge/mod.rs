use self::mirroring::Mirroring;

use super::dh_mappers::mapper::types::Mapper;

pub(super) mod mirroring;

#[derive(Debug)]
pub struct Cartrige<MapperType: Default> {
    pub image_valid: bool,
    pub mirror: Mirroring,
    mapper_id: u8,
    prg_banks: u8,
    chr_banks: u8,
    prg_mem: Vec<u8>,
    chr_mem: Vec<u8>,
    mapper: Mapper<MapperType>,
}

impl<MapperType: Default> Cartrige<MapperType> {
    pub fn new() -> Self {
        Self {
            image_valid: false,
            mirror: Mirroring::Horizontal,
            mapper_id: 0u8,
            prg_banks: 0u8,
            chr_banks: 0u8,
            prg_mem: vec![],
            chr_mem: vec![],
            mapper: Mapper::<MapperType>::default(),
        }
    }
}
