struct SCC {
    pos: i32,
    on_stack: Vec<bool>,
    low: Vec<i32>,
    order: Vec<i32>,
    stack: Vec<usize>,
    component: Vec<Vec<usize>>,
}

impl SCC {
    fn new(n: usize) -> Self {
        Self {
            pos: 0,
            on_stack: vec![false; n],
            low: vec![0; n],
            order: vec![-1; n],
            stack: vec![],
            component: vec![],
        }
    }
    fn dfs(&mut self, u: usize, g: &Vec<Vec<usize>>) {
        self.order[u] = self.pos;
        self.low[u] = self.pos;
        self.pos += 1;
        self.stack.push(u);
        self.on_stack[u] = true;
        for &v in &g[u] {
            if self.order[v] == -1 {
                self.dfs(v, g);
            }
            if self.on_stack[v] {
                self.low[u] = self.low[u].min(self.low[v]);
            }
        }
        if self.low[u] == self.order[u] {
            self.component.push(vec![]);
            loop {
                let v = self.stack.pop().unwrap();
                self.on_stack[v] = false;
                self.component.last_mut().unwrap().push(v);
                if u == v {
                    break;
                }
            }
        }
    }
}

/// Find strongly connected components of graph g. Components are numbered in reverse topological
/// order, starting from 0. It returns the number of components and an array which indicates whichcomponent
/// component each vertex belongs to
pub fn scc(g: &Vec<Vec<usize>>) -> (usize, Vec<usize>) {
    let n = g.len();
    let mut graph = SCC::new(n);
    for i in 0..n {
        if graph.order[i] == -1 {
            graph.dfs(i, g);
        }
    }
    let mut color = vec![0; n];
    let cnt = graph.component.len();
    graph
        .component
        .into_iter()
        .enumerate()
        .for_each(|(i, v)| v.into_iter().for_each(|x| color[x] = i));
    (cnt, color)
}
