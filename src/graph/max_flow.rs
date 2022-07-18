use std::collections::VecDeque;

use crate::traits::integral::Integral;

struct Edge<Cap> {
    to: usize,
    cap: Cap,
}
pub struct Dinic<Cap> {
    n: usize,
    e: Vec<Edge<Cap>>,
    g: Vec<Vec<usize>>,
    cur: Vec<usize>,
    h: Vec<i32>,
}

impl<Cap: Integral> Dinic<Cap> {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            e: vec![],
            g: vec![vec![]; n],
            cur: vec![0; n],
            h: vec![-1; n],
        }
    }

    fn bfs(&mut self, s: usize, t: usize) -> bool {
        self.h.fill(-1);
        let mut q = VecDeque::new();
        self.h[s] = 0;
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            for &i in &self.g[u] {
                let Edge{to: v, cap: c} = self.e[i];
                if c > Cap::zero() && self.h[v] == -1 {
                    self.h[v] = self.h[u] + 1;
                    if v == t {
                        return true;
                    }
                    q.push_back(v);
                }
            }
        }
        false
    }

    fn dfs(&mut self, u: usize, t: usize, f: Cap) -> Cap {
        if u == t {
            return f;
        }
        let mut r = f;
        let mut i = self.cur[u];
        while i < self.g[u].len() {
            let j = self.g[u][i];
            let Edge{to: v, cap: c} = self.e[j];
            if c > Cap::zero() && self.h[v] == self.h[u] + 1 {
                let a = self.dfs(v, t, r.min(c));
                self.e[j].cap -= a;
                self.e[j ^ 1].cap += a;
                r -= a;
                if r == Cap::zero() {
                    break;
                }
            }
            i += 1;
            self.cur[u] = i;
        }
        f - r
    }

    pub fn add_edge(&mut self, u: usize, v: usize, cap: Cap) {
        self.g[u].push(self.e.len());
        self.e.push(Edge { to: v, cap});
        self.g[v].push(self.e.len());
        self.e.push(Edge { to: u, cap: Cap::zero()});
    }

    pub fn get_max_flow(&mut self, s: usize, t: usize) -> Cap {
        assert!(s < self.n);
        assert!(t < self.n);
        let mut ans = Cap::zero();
        while self.bfs(s, t) {
            self.cur.fill(0);
            ans += self.dfs(s, t, Cap::max_value());
        }
        ans
    }
}
