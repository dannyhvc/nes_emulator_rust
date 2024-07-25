use crate::components::dh_mappers::mapper_data::types::MapperData;
// use std::marker::PhantomData;

impl<MapperType> Default for MapperData<MapperType>
where
    MapperType: Default,
{
    fn default() -> Self {
        Default::default()
    }
}
