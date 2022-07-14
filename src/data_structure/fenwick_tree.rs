pub struct FenwickTree<T> {
    n: usize,
    ary: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Clone
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Add<Output = T>
        + std::default::Default
        + std::cmp::PartialOrd,
{
    /// Construct a fenwick tree of size n with default value of `T`.
    pub fn new(n: usize) -> Self {
        FenwickTree {
            n,
            ary: vec![Default::default(); n],
        }
    }

    /// Construct a fenwick tree from `a` in linear time.
    pub fn from(a: &[T]) -> Self {
        let n = a.len();
        let mut obj = Self::new(n);
        for i in 1..=n {
            obj.ary[i - 1] += a[i - 1].clone();
            let j = i + (i & i.wrapping_neg());
            if j <= n {
                let tmp = obj.ary[i - 1];
                obj.ary[j - 1] += tmp;
            }
        }
        obj
    }

    /// Returns `data[0] + ... + data[idx - 1]`.
    pub fn pref(&self, mut idx: usize) -> T {
        let mut sum: T = Default::default();
        while idx > 0 {
            sum += self.ary[idx - 1].clone();
            idx &= idx - 1;
        }
        sum
    }

    /// Performs `data[idx] += val;`.
    pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
    where
        T: std::ops::AddAssign<U>,
    {
        let n = self.n;
        idx += 1;
        while idx <= n {
            self.ary[idx - 1] += val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }
    /// Returns `data[l] + ... + data[r - 1]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_cp::data_structure::fenwick_tree::FenwickTree;
    /// let a = [1, 2, 3, 4, 5];
    /// let tr = FenwickTree::from(&a);
    /// assert_eq!(tr.sum(0, 1), 1);
    /// assert_eq!(tr.sum(1, 3), 5);
    /// assert_eq!(tr.sum(0, 5), 15);
    /// ```
    pub fn sum(&self, l: usize, r: usize) -> T
    where
        T: std::ops::Sub<Output = T>,
    {
        self.pref(r) - self.pref(l)
    }

    /// Returns the index of the partition point according to the given predicate (the index of the first element of the second partition) on the **prefix sum array**.
    ///
    /// In another word, let `sum[i] = data[0] + ... + data[i]`, this function is equivalent to
    /// `sum.partition_point(pref);`
    ///
    /// Time complexity: O(log(n))
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_cp::data_structure::fenwick_tree::FenwickTree;
    /// let a = [1, 2, 3, 4, 5];
    /// let tr = FenwickTree::from(&a);
    /// assert_eq!(tr.partition_point(|x| *x < 0), 0);
    /// assert_eq!(tr.partition_point(|x| *x < 3), 1);
    /// assert_eq!(tr.partition_point(|x| *x < 4), 2);
    /// assert_eq!(tr.partition_point(|x| *x < 100), 5);
    /// ```
    pub fn partition_point<P>(&self, mut pred: P) -> usize
    where
        P: FnMut(&T) -> bool,
    {
        let mut pos = 0;
        let mut sum: T = Default::default();
        for i in (0..=(usize::BITS - 1 - self.n.leading_zeros())).rev() {
            let next = pos + (1 << i);
            if next <= self.n && pred(&(sum.clone() + self.ary[next - 1].clone())) {
                sum += self.ary[next - 1].clone();
                pos = next;
            }
        }
        pos
    }

    /// Returns the index of the first element that is greater than or equal to `target` in the **prefix sum array**. This is a shorthand for `partition_point(|x| *x < *target)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_cp::data_structure::fenwick_tree::FenwickTree;
    /// let a = [1, 2, 3, 4, 5];
    /// let tr = FenwickTree::from(&a);
    /// assert_eq!(tr.search(&0), 0);
    /// assert_eq!(tr.search(&3), 1);
    /// assert_eq!(tr.search(&4), 2);
    /// assert_eq!(tr.search(&100), 5);
    /// ```
    pub fn search(&self, target: &T) -> usize {
        self.partition_point(|x| *x < *target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fenwick_tree_workds() {
        let mut tr: FenwickTree<i64> = FenwickTree::new(5);
        for i in 0..5 {
            tr.add(i, i as i64 + 1);
        }
        assert_eq!(tr.sum(0, 5), 15);
        assert_eq!(tr.sum(0, 4), 10);
        assert_eq!(tr.sum(1, 3), 5);
    }
}
