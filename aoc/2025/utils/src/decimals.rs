pub fn mask10(n: usize) -> usize {
    // a power of 10 which covers n, as in, has as many zeros as n has digits
    // eg 123 -> 1000, 1234 -> 10000
    let mut n = n;
    let mut pow = 1;
    while n > 0 {
        pow *= 10;
        n /= 10;
    }
    pow
}

pub fn digits(n: usize) -> usize {
    let mut n = n;
    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n /= 10;
    }
    digits
}
