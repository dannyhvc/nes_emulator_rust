use once_cell::sync::Lazy;
use std::collections::HashMap;

type HitMap = Lazy<HashMap<u16, Vec<RamAccessType>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RamAccessType {
    Read,
    Write,
}

pub(crate) static mut ADDRESS_HIT_COUNT: HitMap =
    HitMap::new(|| HashMap::new());

fn access_hits(r#type: RamAccessType) -> HashMap<u16, usize> {
    let mut hits = HashMap::new();
    unsafe {
        for (address, access_types) in ADDRESS_HIT_COUNT.iter() {
            let count = access_types.iter().filter(|&&at| at == r#type).count();

            if count > 0 {
                hits.insert(*address, count);
            }
        }
    }
    hits
}

pub fn read_access_hits() -> HashMap<u16, usize> {
    access_hits(RamAccessType::Read)
}

pub fn write_access_hits() -> HashMap<u16, usize> {
    access_hits(RamAccessType::Write)
}
