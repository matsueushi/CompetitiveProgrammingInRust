pub mod ford_fulkerson {
    #[derive(Debug, PartialEq, Eq, Clone)]
    struct GraphEdge {
        to: usize,
        cap: usize,
        rev: usize,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Edge {
        from: usize,
        to: usize,
        cap: usize,
        flow: usize,
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
            let rev = self.data[from].len() + 1;
            self.data[to].push(GraphEdge {
                to: from,
                cap: 0,
                rev,
            });
            self.edge_pos.push((from, self.data[from].len()));
        }

        fn dfs(&mut self, used: &mut Vec<bool>, node: usize, target: usize, flow: usize) -> usize {
            if node == target {
                return flow;
            }
            used[node] = true;
            for edge in &mut self.data[node] {
                if used[edge.to] || edge.cap == 0 {
                    continue;
                }
                let d = self.dfs(used, edge.to, target, std::cmp::min(edge.cap, flow));
                if d == 0 {
                    continue;
                }
                edge.cap -= d;
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

#[cfg(test)]
mod tests {
    use super::ford_fulkerson::*;

    #[test]
    fn test_ford_fulkerson() {
        let mut graph = Graph::new(5);
        graph.add_edge(4, 1, 10);
        graph.add_edge(4, 2, 2);
        graph.add_edge(1, 2, 6);
        graph.add_edge(1, 3, 6);
        graph.add_edge(3, 2, 3);
        graph.add_edge(3, 5, 8);
        graph.add_edge(2, 5, 5);
        assert_eq!(graph.ford_fulkerson(4, 5), 11);
        assert_eq!(
            graph.edges(),
            vec![
                Edge {
                    from: 4,
                    to: 1,
                    cap: 10,
                    flow: 10
                },
                Edge {
                    from: 4,
                    to: 2,
                    cap: 2,
                    flow: 1
                },
                Edge {
                    from: 1,
                    to: 2,
                    cap: 5,
                    flow: 4
                },
                Edge {
                    from: 1,
                    to: 3,
                    cap: 6,
                    flow: 6
                },
                Edge {
                    from: 3,
                    to: 2,
                    cap: 3,
                    flow: 0
                },
                Edge {
                    from: 3,
                    to: 5,
                    cap: 8,
                    flow: 6
                },
                Edge {
                    from: 2,
                    to: 5,
                    cap: 5,
                    flow: 5
                },
            ]
        )
    }
}
