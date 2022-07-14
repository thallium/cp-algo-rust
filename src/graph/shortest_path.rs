use crate::data_structure::min_heap::*;
pub fn dijkstra(g: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<Option<usize>> {
    let mut dis = vec![None; g.len()];
    let mut q = MinHeap::new();
    q.push((0, s));
    dis[s] = Some(0);
    while let Some((d, u)) = q.pop() {
        if Some(d) != dis[u] {
            continue;
        }
        for &(v, c) in &g[u] {
            if dis[v].is_none() || unsafe { dis[v].unwrap_unchecked() } > d + c {
                dis[v] = Some(d + c);
                q.push((d + c, v));
            }
        }
    }
    dis
}
