use self::mirroring::Mirroring;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use crate::dh_mappers::m000::M000;
use crate::dh_mappers::traits::mapper_fn::MapperFn;

pub(super) mod mirroring;

pub struct Cartridge {
    pub mirror: Mirroring,
    mapper_id: u8,
    prg_banks: u8,
    chr_banks: u8,
    prg_mem: Vec<u8>,
    chr_mem: Vec<u8>,
    mapper: Box<dyn MapperFn>,
}

struct Header {
    name: [u8; 4],
    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    prog_ram_size: u8,
    uv_system1: u8,
    uv_system2: u8,
    unused: [u8; 5],
}

impl Cartridge {
    pub fn new(file_name: &str) -> io::Result<Self> {
        let mut file = File::open(file_name)?;
        let mut header = [0u8; 16];
        file.read_exact(&mut header)?;

        let prg_rom_chunks = header[4] as usize;
        let chr_rom_chunks = header[5] as usize;
        let mapper1 = header[6];
        let mapper2 = header[7];

        if mapper1 & 0x04 != 0 {
            file.seek(SeekFrom::Current(512))?;
        }

        let mapper_id = ((mapper2 >> 4) << 4) | (mapper1 >> 4);

        let mirror = if mapper1 & 0x01 != 0 {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        };

        let mut prg_memory = vec![0; prg_rom_chunks * 16_384];
        file.read_exact(&mut prg_memory)?;

        let mut chr_memory;

        if chr_rom_chunks == 0 {
            chr_memory = vec![0; 8192];
        } else {
            chr_memory = vec![0; chr_rom_chunks * 8192];
            file.read_exact(&mut chr_memory)?;
        }

        let mapper = match mapper_id {
            0 => {
                Box::new(M000::new(chr_rom_chunks as u8, prg_rom_chunks as u8))
            }
            _ => panic!("Unsupported Mapper ID: {}", mapper_id),
        };

        Ok(Self {
            mirror,
            mapper_id,
            prg_banks: prg_rom_chunks as u8,
            chr_banks: chr_rom_chunks as u8,
            prg_mem: prg_memory,
            chr_mem: chr_memory,
            mapper,
        })
    }
}
