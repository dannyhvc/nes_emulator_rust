use crate::components::dh_mappers::mapper::types::Mapper;

// #[derive(Debug, Clone, Copy)]
// pub struct M000(MapperData); // test if i can just alias MapperData
#[derive(Default)]
pub struct MapsM000;

pub type M000 = Mapper<MapsM000>;
