use std::iter;

#[inline]
pub fn to_hex(n: u32, d: u8) -> String {
    let mut s: Vec<char> = iter::repeat('0').take(d.into()).collect::<Vec<char>>();
    let mut num: u32 = n;
    let hex_alpha: Vec<char> = "0123456789ABCDEF".chars().collect::<Vec<char>>();
    for i in (0..=d - 1).rev() {
        num >>= 4;
        s[i as usize] = hex_alpha[(num & 0xF) as usize];
    }
    s.into_iter().collect()
}
