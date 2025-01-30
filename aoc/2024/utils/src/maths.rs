pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Copy
        + Eq
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Neg<Output = T>
        + Ord
        + Default, // assuming default is 0
{
    if a < T::default() {
        a = -a;
    }
    if b < T::default() {
        b = -b;
    }
    while b != T::default() {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy
        + Eq
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::Neg<Output = T>
        + Ord
        + Default, // assuming default is 0
{
    a * b / gcd(a, b)
}
