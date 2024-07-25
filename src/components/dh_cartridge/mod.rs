use self::mirroring::Mirroring;

use super::dh_mappers::mapper_data::impls::default;
use super::dh_mappers::mapper_data::types::MapperData;

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
    mapper: MapperData<MapperType>,
}

impl<MapperType: Default> Cartrige<MapperType>
where
    MapperType: Default,
{
    pub fn new() -> Self {
        Self {
            image_valid: false,
            mirror: Mirroring::Horizontal,
            mapper_id: 0u8,
            prg_banks: 0u8,
            chr_banks: 0u8,
            prg_mem: vec![],
            chr_mem: vec![],
            mapper: MapperData::<MapperType>::default(),
        }
    }
}
