pub struct UnionFind {
    n: usize,
    fa: Vec<usize>,
    sz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            n,
            fa: (0..n).into_iter().collect(),
            sz: vec![1; n],
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.fa[i] == i {
            i
        } else {
            self.fa[i] = self.find(i);
            self.fa[i]
        }
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.n);
        assert!(y < self.n);
        self.find(x) == self.find(y)
    }

    pub fn join(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.n);
        assert!(y < self.n);
        let (mut fx, mut fy) = (self.find(x), self.find(y));
        if fx == fy {
            return false;
        }
        if self.sz[x] > self.sz[y] {
            std::mem::swap(&mut fx, &mut fy);
        }
        self.fa[x] = y;
        self.sz[y] += self.sz[x];
        true
    }

    pub fn size(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        let fx = self.find(x);
        self.sz[fx]
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut result = vec![vec![]; self.n];
        for i in 0..self.n {
            result[self.find(i)].push(i);
        }
        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }
}
