use crate::misc::util::ceil_lg;
use crate::traits::monoid::Monoid;

impl<M: Monoid> Default for Segtree<M> {
    fn default() -> Self {
        Segtree::new(0)
    }
}
impl<M: Monoid> Segtree<M> {
    /// Creates an array a of length n. All the elements are initialized to the identity element.
    pub fn new(n: usize) -> Segtree<M> {
        vec![M::identity(); n].into()
    }
}
impl<M: Monoid> From<Vec<M::S>> for Segtree<M> {
    /// Creates an array a, initialized to v.
    fn from(v: Vec<M::S>) -> Self {
        let n = v.len();
        let log = ceil_lg(n as u32) as usize;
        let size = 1 << log;
        let mut d = vec![M::identity(); 2 * size];
        d[size..(size + n)].clone_from_slice(&v);
        let mut ret = Segtree { n, size, log, d };
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }
}
impl<M: Monoid> Segtree<M> {
    /// Set `a[p] = x`.
    pub fn set(&mut self, mut p: usize, x: M::S) {
        assert!(p < self.n);
        p += self.size;
        self.d[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    /// Returns `a[p]`.
    pub fn get(&self, p: usize) -> M::S {
        assert!(p < self.n);
        self.d[p + self.size].clone()
    }

    /// Returns `op(a[l], ..., a[r - 1])`, assuming the properties of the monoid. It returns the identity element if l = r.
    pub fn prod(&self, mut l: usize, mut r: usize) -> M::S {
        assert!(l <= r && r <= self.n);
        let mut sml = M::identity();
        let mut smr = M::identity();
        l += self.size;
        r += self.size;

        while l < r {
            if l & 1 != 0 {
                sml = M::binary_operation(&sml, &self.d[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = M::binary_operation(&self.d[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }

        M::binary_operation(&sml, &smr)
    }

    /// Returns `op(a[0], ..., a[n - 1])`, assuming the properties of the monoid. It returns e() if n = 0.
    pub fn all_prod(&self) -> M::S {
        self.d[1].clone()
    }

    /// Returns the maximum r such that `g(op(a[l], a[l + 1], ..., a[r - 1])) = true`.
    /// i.e. first index r such that `g(op(a[l], a[l + 1], ..., a[r])) = false`.
    pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(l <= self.n);
        assert!(f(&M::identity()));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        let mut sm = M::identity();
        while {
            // do
            while l % 2 == 0 {
                l >>= 1;
            }
            if !f(&M::binary_operation(&sm, &self.d[l])) {
                while l < self.size {
                    l *= 2;
                    let res = M::binary_operation(&sm, &self.d[l]);
                    if f(&res) {
                        sm = res;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sm = M::binary_operation(&sm, &self.d[l]);
            l += 1;
            // while
            {
                let l = l as isize;
                (l & -l) != l
            }
        } {}
        self.n
    }

    /// Return the minimum index l such that `g(op(a[l], a[l + 1], ..., a[r - 1])) = true`.
    pub fn min_left<F>(&self, mut r: usize, f: F) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(r <= self.n);
        assert!(f(&M::identity()));
        if r == 0 {
            return 0;
        }
        r += self.size;
        let mut sm = M::identity();
        while {
            // do
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            if !f(&M::binary_operation(&self.d[r], &sm)) {
                while r < self.size {
                    r = 2 * r + 1;
                    let res = M::binary_operation(&self.d[r], &sm);
                    if f(&res) {
                        sm = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = M::binary_operation(&self.d[r], &sm);
            // while
            {
                let r = r as isize;
                (r & -r) != r
            }
        } {}
        0
    }

    fn update(&mut self, k: usize) {
        self.d[k] = M::binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
    }
}

// Maybe we can use this someday
// ```
// for i in 0..=self.log {
//     for j in 0..1 << i {
//         print!("{}\t", self.d[(1 << i) + j]);
//     }
//     println!();
// }
// ```

/// Segment tree using [ac-library-rs](https://github.com/rust-lang-ja/ac-library-rs)'s implementation.
/// See [this](https://atcoder.github.io/ac-library/production/document_en/segtree.html) for the mathmatical 
/// definition of functions that you need to implement.
pub struct Segtree<M>
where
    M: Monoid,
{
    // variable name is _n in original library
    n: usize,
    size: usize,
    log: usize,
    d: Vec<M::S>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::monoids::Max;

    #[test]
    fn test_max_segtree() {
        let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let n = base.len();
        let segtree: Segtree<Max<_>> = base.clone().into();
        check_segtree(&base, &segtree);

        let mut segtree = Segtree::<Max<_>>::new(n);
        let mut internal = vec![i32::min_value(); n];
        for i in 0..n {
            segtree.set(i, base[i]);
            internal[i] = base[i];
            check_segtree(&internal, &segtree);
        }

        segtree.set(6, 5);
        internal[6] = 5;
        check_segtree(&internal, &segtree);

        segtree.set(6, 0);
        internal[6] = 0;
        check_segtree(&internal, &segtree);
    }

    //noinspection DuplicatedCode
    fn check_segtree(base: &[i32], segtree: &Segtree<Max<i32>>) {
        let n = base.len();
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            assert_eq!(segtree.get(i), base[i]);
        }
        for i in 0..=n {
            for j in i..=n {
                assert_eq!(
                    segtree.prod(i, j),
                    base[i..j].iter().max().copied().unwrap_or(i32::min_value())
                );
            }
        }
        assert_eq!(
            segtree.all_prod(),
            base.iter().max().copied().unwrap_or(i32::min_value())
        );
        for k in 0..=10 {
            let f = |&x: &i32| x < k;
            for i in 0..=n {
                assert_eq!(
                    Some(segtree.max_right(i, f)),
                    (i..=n)
                        .filter(|&j| f(&base[i..j]
                            .iter()
                            .max()
                            .copied()
                            .unwrap_or(i32::min_value())))
                        .max()
                );
            }
            for j in 0..=n {
                assert_eq!(
                    Some(segtree.min_left(j, f)),
                    (0..=j)
                        .filter(|&i| f(&base[i..j]
                            .iter()
                            .max()
                            .copied()
                            .unwrap_or(i32::min_value())))
                        .min()
                );
            }
        }
    }
}
