use std::marker::PhantomData;

// trivially copiable struct on most systems: mov QWORD
#[derive(Debug, Clone, Copy)]
pub struct MapperData<MapperType: Default> {
    pub _marker: PhantomData<MapperType>,
    pub chr_bank: u8,
    pub prg_bank: u8,
}
