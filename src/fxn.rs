#![allow(dead_code)]

pub fn parity(n: usize) -> u8 {
    let mut x = n;
    let mut parity = 0;
    loop {
        if x == 0 {
            break;
        }
        let y = x - 1;
        x = x & y;
        parity = parity ^ 1;
    }
    parity
}

pub fn swap_bits(n: usize, idx1: u8, idx2: u8 ) -> usize {
    let mut swapped = n;
    if (n >> idx1) != (n >> idx2) {
        swapped = swapped ^ (1usize << idx1);
        swapped = swapped ^ (&1usize << idx2);
    }
    swapped
}
