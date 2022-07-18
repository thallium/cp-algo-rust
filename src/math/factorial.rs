use crate::misc::modint::{Modulus, StaticModInt};

pub struct Factorial<T> {
    factorial: Vec<StaticModInt<T>>,
    inv_fac: Vec<StaticModInt<T>>,
}

impl<T: Modulus> Factorial<T> {
    pub fn new(n: usize) -> Self {
        let mut factorial: Vec<StaticModInt<T>> = vec![StaticModInt::from(0i32); n + 1];
        let mut inv_fac: Vec<StaticModInt<T>> = vec![StaticModInt::from(0i32); n + 1];
        factorial[0] = StaticModInt::from(1);
        for i in 1..=n {
            factorial[i] = factorial[i - 1] * i;
        }
        inv_fac[n] = factorial[n].inv();
        for i in (0..n).rev() {
            inv_fac[i] = inv_fac[i + 1] * (i + 1);
        }
        Self {
            factorial,
            inv_fac,
        }
    }

    pub fn choose(&self, n: usize, k: usize) -> StaticModInt<T> {
        assert!(k <= n);
        assert!(n < self.factorial.len());
        self.factorial[n] * self.inv_fac[n - k] * self.inv_fac[k]
    }

    pub fn perm(&self, n: usize, k: usize) -> StaticModInt<T> {
        assert!(k <= n);
        assert!(n < self.factorial.len());
        self.factorial[n] * self.inv_fac[n - k]
    }
}
