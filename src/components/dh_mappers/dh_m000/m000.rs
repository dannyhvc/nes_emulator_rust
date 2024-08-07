use crate::components::dh_mappers::dh_mapper::mapper::Mapper;

// #[derive(Debug, Clone, Copy)]
// pub struct M000(MapperData); // test if i can just alias MapperData
#[derive(Default)]
pub struct MapOfM000;

pub type M000 = Mapper<MapOfM000>;
