use crate::misc::util::lg;

pub struct BinaryLifting {
    pub level: usize,
    pub pa: Vec<Vec<usize>>,
    pub dep: Vec<usize>,
    pub tin: Vec<usize>,
    pub tout: Vec<usize>,
}

impl BinaryLifting {
    pub fn from(g: &Vec<Vec<usize>>, root: usize) -> Self {
        let n = g.len();
        let level = lg(n) as usize + 1;
        let mut ret = Self {
            level,
            pa: vec![vec![0; level]; n],
            dep: vec![0; n],
            tin: vec![0; n],
            tout: vec![0; n],
        };
        let mut timer = 0;
        ret.dfs(g, root, root, &mut timer);
        ret
    }

    pub fn dfs(&mut self, g: &Vec<Vec<usize>>, u: usize, p: usize, timer: &mut usize) {
        self.tin[u] = *timer;
        *timer += 1;
        self.pa[u][0] = p;
        for i in 1..self.level {
            self.pa[u][i] = self.pa[self.pa[u][i - 1]][i - 1];
        }

        for &v in &g[u] {
            if v != p {
                self.dep[v] = self.dep[u] + 1;
                self.dfs(g, v, u, timer)
            }
        }

        self.tout[u] = *timer;
    }

    /// check if u is ancestor of v
    pub fn is_anscestor(&self, u: usize, v: usize) -> bool {
        self.tin[v] >= self.tin[u] && self.tin[v] < self.tout[u]
    }

    pub fn lca(&self, mut u: usize, v: usize) -> usize {
        if self.is_anscestor(u, v) {
            return u;
        }
        if self.is_anscestor(v, u) {
            return v;
        }
        for i in (0..self.level).rev() {
            if !self.is_anscestor(self.pa[u][i], v) {
                u = self.pa[u][i];
            }
        }
        self.pa[u][0]
    }

    pub fn dis(&self, u: usize, v: usize) -> usize {
        let l = self.lca(u, v);
        self.dep[u] - self.dep[l] + self.dep[v] - self.dep[l]
    }
}
