// use std::marker::PhantomData;

use crate::dh_mappers::dh_mapper::mapper::Mapper;

impl<MapsType> Default for Mapper<MapsType>
where
    MapsType: Default,
{
    fn default() -> Self {
        // TODO: will cause some recursion
        crate::dh_mappers::dh_mapper::mapper::Mapper {
            _marker: std::marker::PhantomData,
            chr_bank: 0,
            prg_bank: 0,
        }
    }
}
