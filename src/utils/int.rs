
pub fn get_msb(mut n: usize) -> u32 {
    let mut i: u32 = 0;
    while n > 0 {
        n >>= 1;
        i += 1;
    }
    i
}