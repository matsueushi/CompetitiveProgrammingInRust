use proconio::{fastout, input};
use two_sat::TwoSat;

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
                data: vec![vec![]; n],
                rev_data: vec![vec![]; n],
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

pub mod two_sat {
    use super::scc::Graph;

    pub struct TwoSat {
        n: usize,
        graph: Graph,
    }

    impl TwoSat {
        pub fn new(n: usize) -> Self {
            Self {
                n,
                // 0, ..., n - 1: x_i = true
                // n, ..., 2n - 1: x_i = false
                graph: Graph::new(n << 1),
            }
        }

        pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
            // add clause (x_i = f) or (x_j = g)
            // -> (x_i != f => x_j = g) and (x_j != g => x_i = f)
            let i0 = if f { i + self.n } else { i };
            let j0 = if g { j } else { j + self.n };
            self.graph.add_edge(i0, j0);

            let j1 = if g { j + self.n } else { j };
            let i1 = if f { i } else { i + self.n };
            self.graph.add_edge(j1, i1);
        }

        pub fn satisfiable(&self) -> bool {
            let group = self.graph.scc();
            let mut cmp = vec![0; self.n << 1];
            for (i, nodes) in group.into_iter().enumerate() {
                for node in nodes {
                    cmp[node] = i;
                }
            }
            for i in 0..self.n {
                if cmp[i] == cmp[i + self.n] {
                    return false;
                }
            }
            true
        }

        pub fn answer(&self) -> Result<Vec<bool>, &str> {
            let group = self.graph.scc();
            let mut cmp = vec![0; self.n << 1];
            for (i, nodes) in group.into_iter().enumerate() {
                for node in nodes {
                    cmp[node] = i;
                }
            }

            let mut res = vec![false; self.n];
            for i in 0..self.n {
                if cmp[i] == cmp[i + self.n] {
                    return Err("No solution");
                } else {
                    res[i] = cmp[i] > cmp[i + self.n];
                }
            }
            Ok(res)
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    }
    let mut two_sat = TwoSat::new(n);
    for i in 0..n - 1 {
        for j in i + 1..n {
            let (xi, yi) = &xy[i];
            let (xj, yj) = &xy[j];
            if (xi - xj).abs() < d {
                two_sat.add_clause(i, false, j, false);
            }
            if (xi - yj).abs() < d {
                two_sat.add_clause(i, false, j, true);
            }
            if (yi - xj).abs() < d {
                two_sat.add_clause(i, true, j, false);
            }
            if (yi - yj).abs() < d {
                two_sat.add_clause(i, true, j, true);
            }
        }
    }

    let res = two_sat.answer();
    match res {
        Err(_) => {
            println!("No");
        }
        Ok(ps) => {
            println!("Yes");
            for i in 0..n {
                let (x, y) = &xy[i];
                let p = if ps[i] { x } else { y };
                println!("{}", p);
            }
        }
    }
}
