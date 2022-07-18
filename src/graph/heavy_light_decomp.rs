use std::mem::swap;

use crate::{
    data_structure::lazy_segtree::LazySegtree,
    traits::monoid::{MapMonoid, Monoid},
};

pub struct HLD<T: MapMonoid> {
    pub data: LazySegtree<T>,
    parent: Vec<usize>,
    dep: Vec<usize>,
    heavy_child: Vec<Option<usize>>,
    head: Vec<usize>,
    pub pos: Vec<usize>,
    pos_r: Vec<usize>,
}

impl<T: MapMonoid> HLD<T> {
    pub fn new(g: &Vec<Vec<usize>>, root: usize) -> Self {
        let n = g.len();
        assert!(root < n);
        let mut res = HLD {
            data: LazySegtree::new(n),
            parent: vec![0; n],
            dep: vec![0; n],
            heavy_child: vec![None; n],
            head: vec![0; n],
            pos: vec![0; n],
            pos_r: vec![0; n],
        };
        res.parent[root] = root;
        res.dfs1(root, g);
        let mut timer = 0;
        res.dfs2(root, root, &mut timer, g);
        res
    }

    pub fn with_init_value(g: &Vec<Vec<usize>>, root: usize, v: &Vec<<T::M as Monoid>::S>) -> Self {
        assert_eq!(g.len(), v.len());
        let mut res = HLD::new(g, root);
        let mut a = v.clone();
        for i in 0..v.len() {
            a[res.pos[i]] = v[i].clone();
        }
        res.data = LazySegtree::from(a);
        res
    }

    fn dfs1(&mut self, u: usize, g: &Vec<Vec<usize>>) -> usize {
        let (mut size, mut max_size) = (1, 0);
        for &v in &g[u] {
            if v != self.parent[u] {
                self.parent[v] = u;
                self.dep[v] = self.dep[u] + 1;
                let child_size = self.dfs1(v, g);
                size += child_size;
                if child_size > max_size {
                    max_size = child_size;
                    self.heavy_child[u] = Some(v);
                }
            }
        }
        size
    }

    fn dfs2(&mut self, u: usize, head: usize, timer: &mut usize, g: &Vec<Vec<usize>>) {
        self.head[u] = head;
        self.pos[u] = *timer;
        *timer += 1;
        let v = self.heavy_child[u];
        if v.is_some() {
            self.dfs2(v.unwrap(), head, timer, g);
        }

        for &v in &g[u] {
            if v != self.parent[u] && Some(v) != self.heavy_child[u] {
                self.dfs2(v, v, timer, g);
            }
        }
        self.pos_r[u] = *timer;
    }

    pub fn apply_path(&mut self, mut u: usize, mut v: usize, f: T::F) {
        while self.head[u] != self.head[v] {
            if self.dep[self.head[u]] < self.dep[self.head[v]] {
                swap(&mut u, &mut v);
            }
            self.data.apply_range(self.pos[self.head[u]], self.pos[u] + 1, f.clone());
            u = self.parent[self.head[u]];
        }
        if self.pos[u] > self.pos[v] {
            swap(&mut u, &mut v);
        }
        self.data.apply_range(self.pos[u], self.pos[v] + 1, f.clone());
    }

    pub fn path_prod(&mut self, mut u: usize, mut v: usize) -> <T::M as Monoid>::S {
        let mut res = T::identity_element();
        while self.head[u] != self.head[v] {
            if self.dep[self.head[u]] < self.dep[self.head[v]] {
                swap(&mut u, &mut v);
            }
            res = T::binary_operation(&res, &self.data.prod(self.pos[self.head[u]], self.pos[u] + 1));
            u = self.parent[self.head[u]];
        }
        if self.pos[u] > self.pos[v] {
            swap(&mut u, &mut v);
        }
        res = T::binary_operation(&res, &self.data.prod(self.pos[u], self.pos[v] + 1));
        res
    }

    pub fn apply_subtree(&mut self, u: usize, f: T::F) {
        self.data.apply_range(self.pos[u], self.pos_r[u], f);
    }

    pub fn subtree_prod(&mut self, u: usize) -> <T::M as Monoid>::S {
        self.data.prod(self.pos[u], self.pos_r[u])
    }

    /// Returns the value of node `u`.
    pub fn get(&mut self, u: usize) -> <T::M as Monoid>::S {
        self.data.get(self.pos[u])
    }

    /// Sets the value of node `u` to `x`.
    pub fn set(&mut self, u: usize, x: <T::M as Monoid>::S) {
        self.data.set(self.pos[u], x);
    }
}
