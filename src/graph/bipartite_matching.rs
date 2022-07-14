pub struct AugmentedPath {
    g: Vec<Vec<usize>>,
    l: Vec<Option<usize>>,
    r: Vec<Option<usize>>,
    vis: Vec<bool>,
}

impl AugmentedPath {
    /// n is the size of one vertex set, m is the size of the other vertex set
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            g: vec![vec![]; n],
            l: vec![None; n],
            r: vec![None; m],
            vis: vec![false; n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
    }

    fn dfs(u: usize, g: &Vec<Vec<usize>>, vis: &mut Vec<bool>, l: &mut Vec<Option<usize>>, r: &mut Vec<Option<usize>>) -> bool {
        if vis[u] {
            return false;
        }
        vis[u] = true;
        for &v in &g[u] {
            if r[v].is_none() {
                l[u] = Some(v);
                r[v] = Some(u);
                return true;
            }
        }

        for &v in &g[u] {
            if Self::dfs(r[v].unwrap(), g, vis, l, r) {
                l[u] = Some(v);
                r[v] = Some(u);
                return true;
            }
        }
        false
    }

    pub fn solve(&mut self) -> usize {
        loop {
            let mut ok = false;
            self.vis.fill(false);
            for i in 0..self.l.len() {
                if self.l[i].is_none() {
                    ok |= Self::dfs(i, &self.g, &mut self.vis, &mut self.l, &mut self.r);
                }
            }
            if !ok {
                break;
            }
        }
        let mut ret = 0;
        for i in 0..self.l.len() {
            if self.l[i].is_some() {
                ret += 1;
            }
        }
        ret
    }
}
