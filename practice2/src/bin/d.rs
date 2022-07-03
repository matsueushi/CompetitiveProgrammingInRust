use ford_fulkerson::Graph;
use proconio::{fastout, input, marker::Chars};

pub mod ford_fulkerson {
    #[derive(Debug, PartialEq, Eq, Clone)]
    struct GraphEdge {
        to: usize,
        cap: usize,
        rev: usize,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Edge {
        pub from: usize,
        pub to: usize,
        pub cap: usize,
        pub flow: usize,
    }

    #[derive(Debug, Clone)]
    pub struct Graph {
        n: usize,
        data: Vec<Vec<GraphEdge>>,
        edge_pos: Vec<(usize, usize)>,
    }

    impl Graph {
        pub fn new(n: usize) -> Self {
            Self {
                n,
                data: vec![Vec::new(); n],
                edge_pos: vec![],
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
            let rev = self.data[to].len();
            self.data[from].push(GraphEdge { to, cap, rev });
            let rev = self.data[from].len() - 1;
            self.data[to].push(GraphEdge {
                to: from,
                cap: 0,
                rev,
            });
            self.edge_pos.push((from, rev));
        }

        fn dfs(&mut self, used: &mut Vec<bool>, node: usize, target: usize, flow: usize) -> usize {
            if node == target {
                return flow;
            }
            used[node] = true;
            let m = self.data[node].len();
            for i in 0..m {
                let edge = self.data[node][i].clone();
                if used[edge.to] || edge.cap == 0 {
                    continue;
                }
                let d = self.dfs(used, edge.to, target, std::cmp::min(edge.cap, flow));
                if d == 0 {
                    continue;
                }
                self.data[node][i].cap -= d;
                self.data[edge.to][edge.rev].cap += d;
                return d;
            }
            0
        }

        pub fn ford_fulkerson(&mut self, start: usize, target: usize) -> usize {
            let mut total_flow = 0;
            loop {
                let mut used = vec![false; self.n];
                let flow = self.dfs(&mut used, start, target, std::usize::MAX);
                if flow == 0 {
                    break;
                }
                total_flow += flow;
            }
            total_flow
        }

        fn edge(&self, idx: usize) -> Edge {
            let (node, node_pos) = self.edge_pos[idx];
            let edge = &self.data[node][node_pos];
            let rev_edge = &self.data[edge.to][edge.rev];
            Edge {
                from: node,
                to: edge.to,
                cap: edge.cap + rev_edge.cap,
                flow: rev_edge.cap,
            }
        }

        pub fn edges(&self) -> Vec<Edge> {
            let mut edges = vec![];
            for i in 0..self.edge_pos.len() {
                edges.push(self.edge(i));
            }
            edges
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut grid: [Chars; n],
    }

    let mut graph = Graph::new(n * m + 2);
    let s = n * m;
    let t = n * m + 1;

    // s -> even / odd -> t
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '#' {
                continue;
            }
            let v = i * m + j;
            if (i + j) % 2 == 0 {
                graph.add_edge(s, v, 1);
            } else {
                graph.add_edge(v, t, 1);
            }
        }
    }

    // even -> odd
    for i in 0..n {
        for j in 0..m {
            if (i + j) % 2 == 1 || grid[i][j] == '#' {
                continue;
            }
            let v0 = i * m + j;
            if i > 0 && grid[i - 1][j] == '.' {
                let v1 = (i - 1) * m + j;
                graph.add_edge(v0, v1, 1);
            }
            if j > 0 && grid[i][j - 1] == '.' {
                let v1 = i * m + j - 1;
                graph.add_edge(v0, v1, 1);
            }
            if i + 1 < n && grid[i + 1][j] == '.' {
                let v1 = (i + 1) * m + j;
                graph.add_edge(v0, v1, 1);
            }
            if j + 1 < m && grid[i][j + 1] == '.' {
                let v1 = i * m + j + 1;
                graph.add_edge(v0, v1, 1);
            }
        }
    }

    println!("{}", graph.ford_fulkerson(s, t));

    let edges = graph.edges();
    for edge in &edges {
        if edge.from == s || edge.to == t || edge.flow == 0 {
            continue;
        }
        let i0 = edge.from / m;
        let j0 = edge.from % m;
        let i1 = edge.to / m;
        let j1 = edge.to % m;

        if i0 == i1 + 1 {
            grid[i1][j1] = 'v';
            grid[i0][j0] = '^';
        } else if j0 == j1 + 1 {
            grid[i1][j1] = '>';
            grid[i0][j0] = '<';
        } else if i0 == i1 - 1 {
            grid[i0][j0] = 'v';
            grid[i1][j1] = '^';
        } else {
            grid[i0][j0] = '>';
            grid[i1][j1] = '<';
        }
    }

    for i in 0..n {
        println!("{}", grid[i].iter().collect::<String>());
    }
}
