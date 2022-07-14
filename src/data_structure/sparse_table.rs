use crate::misc::util::lg;
pub struct SparseTable<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    data: Vec<Vec<T>>,
    op: F,
}

impl<T, F> SparseTable<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{

    /// Construct a sparse table from slice `v`.
    pub fn from(v: &[T], init: T, op: F) -> Self {
        let n = v.len();
        let count = lg(n) as usize + 1;
        let mut data = vec![vec![init; n]; count];
        data[0].clone_from_slice(v);
        for c in 1..count {
            let bound = n - (1 << c);
            for i in 0..=bound {
                data[c][i] = op(data[c - 1][i], data[c - 1][i + (1 << (c - 1))]);
            }
        }

        Self { data, op }
    }

    /// get the result for [l, r)
    pub fn get(&self, l: usize, r: usize) -> T {
        assert!(l < r);
        let c = lg(r - l) as usize;
        (self.op)(self.data[c][l], self.data[c][r - (1 << c)])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{thread_rng, Rng};
    use std::cmp;

    #[test]
    fn test_small() {
        let v = vec![1, 9, 9, 1, 0, 7, 1, 7];
        let sparse_table = SparseTable::from(&v, 0, |a, b| cmp::min(a, b));
        println!("{:#?}", sparse_table.data);
        for l in 0..8 {
            for r in (l + 1)..9 {
                let &min = v[l..r].iter().min().unwrap();
                println!("{} {}", l, r);
                assert_eq!(sparse_table.get(l, r), min);
            }
        }
    }

    #[test]
    fn test_sparse_table_with_random_array() {
        let n = 1000;
        let v: Vec<u64> = (0..n)
            .map(|_| thread_rng().gen_range(0, 1000000000))
            .collect();
        let sparse_table = SparseTable::from(&v, 0, cmp::min);
        for l in 0..n {
            for r in (l + 1)..(n + 1) {
                let &min = v[l..r].iter().min().unwrap();
                assert_eq!(sparse_table.get(l, r), min);
            }
        }
    }
}
