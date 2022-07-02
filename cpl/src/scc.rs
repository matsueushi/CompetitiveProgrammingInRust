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

#[cfg(test)]
mod tests {
    use super::scc::*;

    #[test]
    fn test_scc() {
        let mut graph = Graph::new(6);
        graph.add_edge(1, 4);
        graph.add_edge(5, 2);
        graph.add_edge(3, 0);
        graph.add_edge(5, 5);
        graph.add_edge(4, 1);
        graph.add_edge(0, 3);
        graph.add_edge(4, 2);
        assert_eq!(graph.scc(), vec![vec![5], vec![1, 4], vec![2], vec![0, 3]]);
    }
}
