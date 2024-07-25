use crate::components::dh_mappers::mapper_data::types::MapperData;

// #[derive(Debug, Clone, Copy)]
// pub struct M000(MapperData); // test if i can just alias MapperData
#[derive(Default)]
pub struct M000Mapper;

pub type M000 = MapperData<M000Mapper>;
