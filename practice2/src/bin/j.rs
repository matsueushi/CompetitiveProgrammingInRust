use proconio::{fastout, input};
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

    pub struct SegTree<S: Monoid> {
        n: usize,
        size: usize,
        log: usize,
        data: Vec<S>,
    }

    impl<S> SegTree<S>
    where
        S: Clone + Monoid,
    {
        pub fn new(arr: &Vec<S>) -> Self {
            let n = arr.len();
            let log = ceil_pow2(n);
            let size = 1 << log;
            let data = vec![S::e(); size << 1];
            let mut st = Self { n, log, size, data };
            for (i, val) in arr.into_iter().enumerate() {
                st.data[i + size] = (*val).clone();
            }
            for i in (1..size).rev() {
                st.update(i);
            }
            st
        }

        fn update(&mut self, idx: usize) {
            self.data[idx] = self.data[idx << 1].op(&self.data[(idx << 1) + 1]);
        }

        pub fn set(&mut self, idx: usize, val: S) {
            let idx = idx + self.size;
            self.data[idx] = val.clone();
            for k in 1..=self.log {
                let j = idx >> k;
                self.update(j);
            }
        }

        pub fn prod(&self, l: usize, r: usize) -> S {
            let mut sml = S::e();
            let mut smr = S::e();
            let mut l = l + self.size;
            let mut r = r + self.size;
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

    impl<T: Monoid + Clone> Index<usize> for SegTree<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index + self.size]
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    impl Monoid for usize {
        fn op(&self, other: &Self) -> Self {
            std::cmp::max(*self, *other)
        }
        fn e() -> Self {
            0
        }
    }

    let mut seg = SegTree::<usize>::new(&a);

    let mut res = vec![];
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                    v: usize,
                }
                seg.set(x - 1, v);
            }
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }
                let p = seg.prod(l - 1, r);
                res.push(p);
            }
            3 => {
                input! {
                    p: usize,
                    v: usize,
                }
                let pos = seg.max_right(p - 1, |&x| x < v) + 1;
                res.push(pos);
            }
            _ => unreachable!(),
        }
    }

    for x in res {
        println!("{}", x);
    }
}
