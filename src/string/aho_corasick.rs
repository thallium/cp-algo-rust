struct Node<const N: usize = 26> {
    back: usize,
    end: Option<usize>,
    nmatches: usize,
    output: Option<usize>,
    next: [usize; N],
}

impl<const N: usize> Node<N> {
    pub fn new() -> Self {
        Self {
            back: 0,
            end: None,
            nmatches: 0,
            output: None,
            next: [0; N],
        }
    }
}

pub struct AhoCorasick<const OFFSET: u8 = b'a', const N: usize = 26> {
    n: usize,
    nodes: Vec<Node<N>>,
}

impl<const OFFSET: u8, const N: usize> AhoCorasick<OFFSET, N> {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            nodes: Vec::from([Node::new()]),
        }
    }

    pub fn from(patterns: &[&[u8]]) -> Self {
        let mut obj = Self::new(patterns.len());
        for i in 0..patterns.len() {
            obj.insert(patterns[i], i);
        }
        obj.build();
        obj
    }

    pub fn insert(&mut self, s: &[u8], i: usize) {
        assert!(!s.is_empty());
        let mut p = 0;
        for c in s {
            let idx = (*c - OFFSET) as usize;
            if self.nodes[p].next[idx] == 0 {
                self.nodes[p].next[idx] = self.nodes.len();
                self.nodes.push(Node::new());
            }
            p = self.nodes[p].next[idx];
        }
        self.nodes[p].end = Some(i);
        self.nodes[p].nmatches += 1;
    }

    pub fn build(&mut self) {
        let mut q = std::collections::VecDeque::new();
        for i in 0..N {
            if self.nodes[0].next[i] != 0 {
                q.push_back(self.nodes[0].next[i]);
            }
        }
        while let Some(u) = q.pop_front() {
            for i in 0..N {
                let fail = self.nodes[self.nodes[u].back].next[i];
                match self.nodes[u].next[i] {
                    0 => self.nodes[u].next[i] = fail,
                    v => {
                        self.nodes[v].back = fail;
                        self.nodes[v].output = if self.nodes[fail].end.is_some() {
                            Some(fail)
                        } else {
                            self.nodes[fail].output
                        };
                        q.push_back(v);
                    }
                }
            }
        }
    }

    pub fn find(&self, text: &[u8]) -> Vec<Option<usize>> {
        let n = text.len();
        let mut p = 0;
        (0..n)
            .into_iter()
            .map(|i| {
                p = self.nodes[p].next[(text[i] - OFFSET) as usize];
                self.nodes[p].end
            })
            .collect()
    }

    pub fn find_all(&self, text: &[u8]) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; text.len()];
        let mut p = 0;
        for (i, &c) in text.iter().enumerate() {
            p = self.nodes[p].next[(c - OFFSET) as usize];
            if let Some(x) = self.nodes[p].end {
                res[i].push(x);
            }
            let mut idx = self.nodes[p].output;
            while let Some(x) = idx {
                res[i].push(self.nodes[x].end.unwrap());
                idx = self.nodes[x].output;
            }
        }
        res
    }

    pub fn find_cnt(&self, text: &[u8]) -> Vec<usize> {
        let mut cnt = vec![0; self.n];
        let mut p = 0;
        for &c in text {
            p = self.nodes[p].next[(c - OFFSET) as usize];
            if let Some(x) = self.nodes[p].end {
                cnt[x] += 1;
            }
            let mut idx = self.nodes[p].output;
            while let Some(x) = idx {
                cnt[self.nodes[x].end.unwrap()] += 1;
                idx = self.nodes[x].output;
            }
        }
        cnt
    }
}
