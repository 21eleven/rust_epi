pub fn parity(n: usize) -> usize {
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
