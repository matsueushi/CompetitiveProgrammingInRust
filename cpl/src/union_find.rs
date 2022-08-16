pub mod union_find {

    #[derive(Debug, Clone)]
    pub struct UnionFind {
        par: Vec<usize>,
        size: Vec<usize>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self {
                par: vec![0; n],
                size: vec![1; n],
            }
        }

        pub fn find_root(&mut self, a: usize) -> usize {
            if self.size[a] > 0 {
                return a;
            }
            self.par[a] = self.find_root(self.par[a]);
            self.par[a]
        }

        pub fn union(&mut self, a: usize, b: usize) -> usize {
            let mut x = self.find_root(a);
            let mut y = self.find_root(b);
            if x == y {
                return x;
            }
            if self.size[x] < self.size[y] {
                std::mem::swap(&mut x, &mut y);
                (x, y) = (y, x);
            }
            self.size[x] += self.size[y];
            self.size[y] = 0;
            self.par[y] = x;
            x
        }

        pub fn in_same_set(&mut self, a: usize, b: usize) -> bool {
            self.find_root(a) == self.find_root(b)
        }

        pub fn group_size(&mut self, a: usize) -> usize {
            let x = self.find_root(a);
            self.size[x]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::union_find::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(2);
        assert!(uf.in_same_set(0, 0));
        assert!(!uf.in_same_set(0, 1));
        assert_eq!(uf.union(0, 1), 0);
        assert_eq!(uf.find_root(1), 0);
        assert_eq!(uf.group_size(1), 2);
    }
}
