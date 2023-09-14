#[deprecated(note="the format function does this way faster and better than this function")]
#[inline]
pub fn to_hex(n: u32, d: u8) -> String {
    use std::iter;
    let mut s: Box<[char]> = iter::repeat('0').take(d.into()).collect();
    let mut num: u32 = n;
    let hex_alpha: Box<[char]> = "0123456789ABCDEF".chars().collect();
    for i in (0..=d - 1).rev() {
        num >>= 4;
        s[i as usize] = hex_alpha[(num & 0xF) as usize];
    }
    s.into_iter().collect()
}
