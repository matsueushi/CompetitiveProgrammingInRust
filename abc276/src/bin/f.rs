use proconio::{fastout, input};
const P: usize = 998244353;

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

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut iidx: Vec<usize> = (0..n).collect();
    iidx.sort_by_key(|&i| &a[i]);
    let mut idx = vec![0; n];
    for i in 0..n {
        idx[iidx[i]] = i;
    }

    let mut iinv = vec![0; n + 1];
    iinv[1] = 1;
    for i in 2..n + 1 {
        iinv[i] = P - iinv[P % i] * (P / i) % P;
    }

    let mut v = 0;
    let mut cfw = Fenwick::new(n);
    let mut fw = Fenwick::new(n);
    for i in 0..n {
        let un = cfw.sum(0, idx[i]) * a[i];
        let ov = fw.sum(idx[i], n);
        v += a[i] + 2 * (un + ov);
        v %= P;
        let inv = iinv[i + 1];
        println!("{}", v * (inv * inv % P) % P);
        cfw.add(idx[i], 1);
        fw.add(idx[i], a[i]);
    }
}
