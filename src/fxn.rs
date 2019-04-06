#![allow(dead_code)]

pub fn build_16bit_parity_lookup() -> Vec<u16> {
    (0..=0xFFFF).map(|i| parity(i) as u16).collect()
}

pub fn parity(n: u64) -> u64 {
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

pub fn swap_bits(n: u64, idx1: u64, idx2: u64) -> u64 {
    /*
    swaps bits at two indicies on the a bit string if they differ
    */
    let mut swapped = n;
    if (n >> idx1 & 1) != (n >> idx2 & 1) {
        swapped = swapped ^ (1u64 << idx1);
        swapped = swapped ^ (&1u64 << idx2);
    }
    swapped
}

pub fn parity_lookup(n: u64, lookup: &Vec<u16>) -> u64 {
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
    (p1 ^ p2 ^ p3 ^ p4) as u64
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

pub fn reverse_64(x: u64) -> u64 {
    let mut n = x;
    for i in 0..32 {
        if (n >> (63 - i) & 1) != ( (n >> i) & 1) {
            n = n ^ (1 << (63 - i));
            n = n ^ (1 << i);
        } 
    }
    n
}

pub fn reverse(x: u16) -> u16 {
    let mut n = x;
    for i in 0..8 {
        if (n >> (15 - i) & 1) != ( (n >> i) & 1) {
            n = n ^ (1 << (15 - i));
            n = n ^ (1 << i);
        } 
    }
    n
}

pub fn closest_int_same_weight(x: u64) -> u64 {
    let mut n = x;
    for i in 0..63 {
        if (n >> i) & 1 != ((x >> (i + 1)) & 1) {
            n = n ^ ((1 << i ) | (1 << i+1));
            break;
        }
    }
    n
}

pub fn multiply(i: u64, j: u64) -> u64 {
    fn add(a: u64, b: u64) -> u64 {
        let mut running_sum = 0;
        let mut carryin = 0;
        let mut k = 1;
        let mut temp_a = a;
        let mut temp_b = b;
        
        while temp_a > 0 || temp_b > 0 {
            let ak = a & k;
            let bk = b & k;
            let carryout = (ak & bk) | (ak & carryin) | (bk & carryin);
            running_sum |= ak ^ bk ^ carryin;
            carryin = carryout << 1;
            k <<= 1;
            temp_a >>= 1;
            temp_b >>= 1;
        }

        running_sum | carryin
    }

    let mut x = i;
    let mut y = j;
    let mut running_sum = 0;
    while x > 0 {
        if x & 1 == 1 {
            running_sum = add(running_sum, y)
        }
        x >>= 1;
        y <<= 1;
    }

    running_sum    
}

pub fn divide(a: u64, b: u64) -> u64 {
    let mut x = a;
    let y = b;
    let mut result = 0;
    let mut power = 32;

    let mut y_power = y << power;
    while x >= y {
        while y_power > x {
            y_power >>= 1;
            power -= 1;
        }
        result += 1 << power;
        x -= y_power;
    }
    result
}
