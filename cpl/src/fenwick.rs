pub mod fenwick {
    use std::ops::{AddAssign, Sub};

    // 一点加算、区間取得(sum)
    #[derive(Debug)]
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

    // 区間加算区間取得
    #[derive(Debug)]
    pub struct RangeAddQuery<T> {
        fw0: Fenwick<T>,
        fw1: Fenwick<T>,
    }

    impl RangeAddQuery<i64> {
        pub fn new(n: usize) -> Self {
            Self {
                fw0: Fenwick::new(n),
                fw1: Fenwick::new(n),
            }
        }

        pub fn add(&mut self, l: usize, r: usize, val: i64) {
            // add val to [l, r)
            self.fw0.add(l, -val * l as i64);
            self.fw0.add(r, val * r as i64);
            self.fw1.add(l, val);
            self.fw1.add(r, -val);
        }

        pub fn prefix_sum(&self, r: usize) -> i64 {
            self.fw0.prefix_sum(r) + self.fw1.prefix_sum(r) * r as i64
        }

        pub fn sum(&self, l: usize, r: usize) -> i64 {
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

    #[test]
    fn test_range_add_query() {
        let mut raq = RangeAddQuery::new(3);
        raq.add(0, 2, 1);
        raq.add(1, 3, 2);
        raq.add(2, 3, 3);
        assert_eq!(raq.sum(1, 2), 3);
        assert_eq!(raq.sum(2, 3), 5);
    }
}
