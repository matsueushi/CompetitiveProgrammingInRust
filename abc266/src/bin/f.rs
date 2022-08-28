use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;

fn dfs(
    graph: &Vec<Vec<usize>>,
    mut used: &mut Vec<bool>,
    mut par: &mut Vec<usize>,
    from: usize,
    to: usize,
    mut lst: &mut usize,
    mut len: &mut usize,
) {
    par[to] = from;
    used[to] = true;
    for v in &graph[to] {
        if from == *v {
            continue;
        }
        if used[*v] {
            // revisit
            *lst = to;
            *len = *v;
            return;
        }
        dfs(&graph, &mut used, &mut par, to, *v, &mut lst, &mut len);
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n],
        q: usize,
        xy: [(Usize1, Usize1); q],
    }

    let mut graph = vec![vec![]; n];
    for (u, v) in &uv {
        graph[*u].push(*v);
        graph[*v].push(*u);
    }

    let mut used = vec![false; n];
    let mut par = vec![0; n];
    let mut lst = 0;
    let mut len = 0;
    dfs(&graph, &mut used, &mut par, 0, 0, &mut lst, &mut len);

    let mut v = len;
    let mut lp = vec![lst];
    while v != lst {
        lp.push(v);
        v = par[v];
    }

    let hset: HashSet<_> = lp.into_iter().collect();

    let mut uf = UnionFind::new(n);
    for (u, v) in &uv {
        if hset.contains(u) && hset.contains(v) {
            continue;
        }
        uf.union(*u, *v);
    }

    for (x, y) in xy {
        if uf.equiv(x, y) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
