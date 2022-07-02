use proconio::{fastout, input};
use scc::Graph;

pub mod scc {
    use std::vec;

    #[derive(Debug, Clone)]
    pub struct Graph {
        n: usize,
        data: Vec<Vec<usize>>,
        rev_data: Vec<Vec<usize>>,
    }

    impl Graph {
        pub fn new(n: usize) -> Self {
            Self {
                n,
                data: vec![Vec::new(); n],
                rev_data: vec![Vec::new(); n],
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize) {
            self.data[from].push(to);
            self.rev_data[to].push(from);
        }

        fn dfs(&self, v: usize, used: &mut Vec<bool>, vs: &mut Vec<usize>) {
            used[v] = true;
            for u in &self.data[v] {
                if !used[*u] {
                    self.dfs(*u, used, vs);
                }
            }
            vs.push(v);
        }

        fn rdfs(&self, v: usize, used: &mut Vec<bool>, cmp: &mut Vec<usize>) {
            used[v] = true;
            cmp.push(v);
            for u in &self.rev_data[v] {
                if !used[*u] {
                    self.rdfs(*u, used, cmp);
                }
            }
        }

        pub fn scc(&self) -> Vec<Vec<usize>> {
            let mut used = vec![false; self.n];
            let mut vs = vec![];
            for v in 0..self.n {
                if !used[v] {
                    self.dfs(v, &mut used, &mut vs);
                }
            }
            let mut group = Vec::<Vec<usize>>::new();
            let mut used = vec![false; self.n];
            for v in vs.iter().rev() {
                if !used[*v] {
                    let mut cmp = vec![];
                    self.rdfs(*v, &mut used, &mut cmp);
                    group.push(cmp);
                }
            }
            group
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut graph = Graph::new(n);
    for (a, b) in ab {
        graph.add_edge(a, b);
    }

    let group = graph.scc();
    println!("{}", group.len());
    for g in group {
        println!(
            "{} {}",
            g.len(),
            g.into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        );
    }
}
