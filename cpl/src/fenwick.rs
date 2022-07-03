pub mod fenwick {
    use std::ops::{AddAssign, Sub};

    pub struct Fenwick<T> {
        n: usize,
        data: Vec<T>,
    }

    impl<T> Fenwick<T>
    where
        T: Default + Copy + AddAssign + Sub<Output = T>,
    {
        pub fn new(n: usize) -> Self {
            Self {
                n,
                data: vec![T::default(); n],
            }
        }

        pub fn add(&mut self, idx: usize, val: T) {
            let mut idx = idx + 1;
            while idx <= self.n {
                self.data[idx - 1] += val;
                idx += idx & (!idx + 1);
            }
        }

        pub fn prefix_sum(&self, r: usize) -> T {
            let mut s = T::default();
            let mut idx = r;
            while idx > 0 {
                s += self.data[idx - 1];
                idx -= idx & (!idx + 1);
            }
            s
        }

        pub fn sum(&self, l: usize, r: usize) -> T {
            self.prefix_sum(r) - self.prefix_sum(l)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fenwick::*;

    #[test]
    fn test_fenwick() {
        for n in 0..=50 {
            let mut fenwick = Fenwick::new(n);
            for i in 0..n {
                fenwick.add(i, i * i);
            }
            for l in 0..=n {
                for r in l..=n {
                    let mut s = 0;
                    for i in l..r {
                        s += i * i;
                    }
                    assert_eq!(fenwick.sum(l, r), s);
                }
            }
        }
    }
}
