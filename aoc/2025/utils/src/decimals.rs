/// return a power of 10 which covers n, as in, has as many zeros as n has digits
/// eg 123 -> 1000, 1234 -> 10000, 10 -> 100
pub fn mask10(n: usize) -> usize {
    let mut n = n;
    let mut pow = 1;
    while n > 0 {
        pow *= 10;
        n /= 10;
    }
    pow
}

/// return a power of 10 which  is >= n
/// eg 123 -> 1000, 1234 -> 10000, 10 -> 10
pub fn next_pow10(n: usize) -> usize {
    let mut n = n;
    let mut pow = 1;
    let mut rem = false;
    while n > 1 || (n > 0 && rem) {
        pow *= 10;
        rem |= n % 10 > 0;
        n /= 10;
    }
    pow
}

/// Return the number of decimal digits of the number, eg:
/// ```
/// use utils::decimals::digits;
/// assert!(digits(0) == 1);
/// assert!(digits(3) == 1);
/// assert!(digits(233) == 3);
/// assert!(digits(1000) == 4);
/// assert!(digits(9999) == 4);
/// ```
pub fn digits(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut n = n;
    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n /= 10;
    }
    digits
}

pub fn from_ascii_digit_skipping(digs: impl Iterator<Item = u8>) -> usize {
    let mut num = 0;
    for d in digs {
        if matches!(d, b'0'..=b'9') {
            num *= 10;
            num += (d - b'0') as usize;
        } // else we skip
    }
    num
}

#[cfg(test)]
mod tests {}
