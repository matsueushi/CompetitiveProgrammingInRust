use proconio::{fastout, input, marker::Chars};
use segtree::{Monoid, SegTree};

pub mod segtree {
    use std::ops::Index;

    fn ceil_pow2(n: usize) -> usize {
        let mut x: usize = 0;
        while (1 << x) < n {
            x += 1;
        }
        x
    }

    pub trait Monoid {
        fn op(&self, other: &Self) -> Self;
        fn e() -> Self;
    }

    #[derive(Debug, Clone)]
    pub struct SegTree<S: Monoid> {
        n: usize,
        size: usize,
        log: usize,
        data: Vec<S>,
    }

    impl<S> SegTree<S>
    where
        S: Monoid + Clone,
    {
        pub fn new(arr: &Vec<S>) -> Self {
            let n = arr.len();
            let log = ceil_pow2(n);
            let size = 1 << log;
            let data = vec![S::e(); size << 1];
            let mut st = Self { n, log, size, data };
            for (i, val) in arr.into_iter().enumerate() {
                st.data[size + i] = (*val).clone();
            }
            for i in (1..size).rev() {
                st.update(i);
            }
            st
        }

        fn update(&mut self, i: usize) {
            self.data[i] = self.data[i << 1].op(&self.data[(i << 1) + 1]);
        }

        pub fn set(&mut self, i: usize, val: S) {
            let i = self.size + i;
            self.data[i] = val.clone();
            for j in 1..=self.log {
                self.update(i >> j);
            }
        }

        pub fn prod(&self, l: usize, r: usize) -> S {
            if l == r {
                return S::e();
            }
            let mut l = self.size + l;
            let mut r = self.size + r;
            let mut sml = S::e();
            let mut smr = S::e();
            while l < r {
                if l & 1 == 1 {
                    sml = sml.op(&self.data[l]);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    smr = self.data[r].op(&smr);
                }
                l >>= 1;
                r >>= 1;
            }
            sml.op(&smr)
        }

        pub fn all_prod(&self) -> S {
            self.data[1].clone()
        }

        pub fn max_right<F>(&self, l: usize, f: F) -> usize
        where
            F: Fn(&S) -> bool,
        {
            if l == self.n {
                return self.n;
            }

            let mut l = l + self.size;
            let mut sm = S::e();
            loop {
                while l & 1 == 0 {
                    l >>= 1;
                }
                if !(f(&sm.op(&self.data[l]))) {
                    while l < self.size {
                        l <<= 1;
                        let val = sm.op(&self.data[l]);
                        if f(&val) {
                            sm = val;
                            l += 1;
                        }
                    }
                    return l - self.size;
                }
                sm = sm.op(&self.data[l]);
                l += 1;
                if l & ((!l) + 1) == l {
                    break;
                }
            }
            self.n
        }

        pub fn min_left<F>(&self, r: usize, f: F) -> usize
        where
            F: Fn(&S) -> bool,
        {
            if r == 0 {
                return 0;
            }

            let mut r = r + self.size;
            let mut sm = S::e();
            loop {
                r -= 1;
                while r > 0 && r & 1 == 0 {
                    r >>= 1;
                }
                if !(f(&self.data[r].op(&sm))) {
                    while r < self.size {
                        r = (r << 1) + 1;
                        let val = self.data[r].op(&sm);
                        if f(&val) {
                            sm = val;
                            r -= 1;
                        }
                    }
                    return r + 1 - self.size;
                }
                sm = self.data[r].op(&sm);
                if r & ((!r) + 1) == r {
                    break;
                }
            }
            0
        }
    }

    impl<S: Monoid + Clone> Index<usize> for SegTree<S> {
        type Output = S;

        fn index(&self, i: usize) -> &Self::Output {
            &self.data[i + self.size]
        }
    }
}

// monoidal structure
#[derive(Debug, Copy, Clone, PartialEq)]
struct X {
    u: i64,
    v: i64,
}

impl Monoid for X {
    fn op(&self, other: &Self) -> Self {
        Self {
            u: self.u + other.u,
            v: self.v.min(self.u + other.v),
        }
    }
    fn e() -> Self {
        Self { u: 0, v: 0 }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        qs: [(usize, usize, usize); q],
    }
    // println!("{} {} {:?} {:?}", n, q, s, qs);
    let mut arr = s
        .iter()
        .map(|x| {
            if x == &'(' {
                X { u: 1, v: 1 }
            } else {
                X { u: -1, v: -1 }
            }
        })
        .collect::<Vec<_>>();
    // println!("{:?}", arr);

    let mut seg = SegTree::new(&arr);
    for (t, l, r) in qs {
        if t == 1 {
            if &arr[l - 1] == &arr[r - 1] {
                continue;
            }
            arr.swap(l - 1, r - 1);
            seg.set(l - 1, arr[l - 1]);
            seg.set(r - 1, arr[r - 1]);
        } else {
            let prod = seg.prod(l - 1, r);
            // println!("{:?}", prod);
            if prod.u == 0 && prod.v >= 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
