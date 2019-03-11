#![allow(dead_code)]

pub fn build_16bit_parity_lookup() -> Vec<u8> {
    (0..=0xFFFF).map(|i| parity(i)).collect()
}

pub fn parity(n: usize) -> u8 {
    /*
    finds the parity of a n-bit word
    */
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
    /*
    swaps bits at two indicies on the a bit string if they differ
    */
    let mut swapped = n;
    if (n >> idx1) != (n >> idx2) {
        swapped = swapped ^ (1usize << idx1);
        swapped = swapped ^ (&1usize << idx2);
    }
    swapped
}

pub fn parity_lookup(n: u64, lookup: &Vec<u8>) -> u8 {
    /*
    used a cached lookup of table containing
    the parity of all 16bit strings to find the
    parity of a 64bit word efficiently
    */
    let bitmask = 0xFFFF;
    let p1 = lookup[(n & bitmask) as usize];
    let p2 = lookup[((n >> 16) & bitmask) as usize];
    let p3 = lookup[((n >> 32) & bitmask) as usize];
    let p4 = lookup[((n >> 48) & bitmask) as usize];
    (p1 ^ p2 ^ p3 ^ p4)
}

pub fn reverse_bits(n: u64, lookup: &Vec<u16>) -> u64 {
    let bitmask = 0xFFFF;
    let p1 = lookup[(n & bitmask) as usize];
    let p2 = lookup[(n >> 16 & bitmask) as usize];
    let p3 = lookup[(n >> 32 & bitmask) as usize];
    let p4 = lookup[(n >> 48 & bitmask) as usize];
    ((p1 as u64) << 48) ^ ((p2 as u64) << 32) ^ ((p3 as u64) << 16) ^ (p4 as u64) 

}

pub fn build_16bit_reverse_lookup() -> Vec<u16> {
    (0..=0xFFFF).map(|i| reverse(i)).collect()
}

pub fn reverse(x: u16) -> u16 {
    let mut n = x;
    for i in 0..8 {
        if (n >> (15 - 0) & 1) != ( (n >> i) & 1) {
            n = n ^ (1 << (15 - i));
            n = n ^ (1 << i);
        } 
    }
    n
}
