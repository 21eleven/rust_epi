#![allow(dead_code)]
use std::collections::HashMap;

fn build_16bit_parity_lookup() -> std::collections::HashMap<usize, u8> {
    let mut parity_16_bit_words = HashMap::new();
    for i in 0..=0xFFFF {
        let p = parity(i);
        parity_16_bit_words.insert(i, p);
    }
    parity_16_bit_words

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

pub fn parity_lookup(n: u64) -> u8 {
    /*
    used a cached lookup of table containing
    the parity of all 16bit strings to find the
    parity of a 64bit word efficiently
    */
    let lookup = build_16bit_parity_lookup();
    let bitmask = 0xFFFF;
    let p1 = lookup.get(&(n & bitmask)).expect("parity");
    let p2 = lookup.get(&((n >> 16) & bitmask)).expect("parity");
    let p3 = lookup.get(&((n >> 32) & bitmask)).expect("parity");
    let p4 = lookup.get(&((n >> 48) & bitmask)).expect("parity");
    (p1 ^ p2 ^ p3 ^ p4)
}
