use fenwick::Fenwick;
use proconio::{fastout, input};

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

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
    }

    let mut fw = Fenwick::new(n);
    for i in 0..n {
        fw.add(i, a[i]);
    }

    let mut res = vec![];
    for _ in 0..q {
        input! {
            t: usize
        }
        match t {
            0 => {
                input! {
                    p: usize,
                    x: u64,
                }
                fw.add(p, x);
            }
            1 => {
                input! {
                    l: usize,
                    r: usize,
                }
                let s = fw.sum(l, r);
                res.push(s);
            }
            _ => unreachable!(),
        }
    }

    for x in res {
        println!("{}", x);
    }
}
