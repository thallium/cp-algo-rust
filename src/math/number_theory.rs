use crate::traits::integral::Integral;

pub fn gcd<T: Integral>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm<T: Integral>(a: T, b: T) -> T {
    a * b / gcd(a, b)
}

/// returns a solution (x, y, g) to `a * x + b * y = gcd(a, b) = g`
pub fn ex_gcd<T: Integral>(a: T, b: T) -> (T, T, T) {
    if b == T::zero() {
        return (T::one(), T::zero(), a);
    }
    let (x, y, g) = ex_gcd(b, a % b);
    (y, x - a / b * y, g)
}

/// Extended Chinese Remainder Theorem. It solves systems of linear congruences equations:
///
/// `a[i] * x = b[i] (mod m[i])` (`m[i]` don't need to be pair-wise co-prime)
/// 
/// Returns `(ans, lcm)` where `ans` is the smallest positive integer solution and the solution has
/// general form of `ans + k * lcm`, or `None` if the solution doesn't exist.
pub fn ex_crt(a: &Vec<i64>, b: &Vec<i64>, m: &Vec<i64>) -> Option<(i64, i64)> {
    let n = a.len();
    let (mut x, mut lcm) = (0i64, 1i64);
    for i in 0..n {
        let (a_, mut b_, mut m_) = (a[i] * lcm, b[i] - a[i] * x, m[i]);
        let (y, _, g) = ex_gcd(a_, m_);
        if b_ % g != 0 {
            return None;
        }
        b_ /= g;
        m_ /= g;
        x += lcm * (y as i128 * b_ as i128 % m_ as i128) as i64;
        lcm *= m_;
    }
    x = (x + lcm) % lcm;
    Some((x, lcm))
}
