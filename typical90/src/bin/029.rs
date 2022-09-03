use proconio::{fastout, input, marker::Usize1};

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

pub trait Action<S: Monoid> {
    fn act(&self, s: &S) -> S;
}

// Lazy Segtree
// S: monoid of elements
// X: monoid of operators
// *: S x X -> S right action

#[derive(Debug, Clone)]
pub struct LazySegTree<S, X> {
    n: usize,
    size: usize,
    log: usize,
    data: Vec<S>,
    lazy: Vec<X>,
}

impl<S, X> LazySegTree<S, X>
where
    S: Monoid + Clone,
    X: Monoid + Action<S> + Clone,
{
    pub fn new(arr: &Vec<S>) -> Self {
        let n = arr.len();
        let log = ceil_pow2(n);
        let size = 1 << log;
        let data = vec![S::e(); size << 1];
        let lazy = vec![X::e(); size];
        let mut lst = Self {
            n,
            size,
            log,
            data,
            lazy,
        };
        for (i, val) in arr.into_iter().enumerate() {
            lst.data[size + i] = (*val).clone();
        }
        for i in (1..size).rev() {
            lst.update(i);
        }
        lst
    }

    fn update(&mut self, i: usize) {
        self.data[i] = self.data[i << 1].op(&self.data[(i << 1) + 1]);
    }

    fn all_apply(&mut self, k: usize, x: &X) {
        // data[k] = data[k] . x
        self.data[k] = x.act(&self.data[k]);
        if k < self.size {
            // lazy[k] = lazy[k] . x
            self.lazy[k] = x.op(&self.lazy[k]);
        }
    }

    fn push(&mut self, k: usize) {
        // push down lazy action at k
        self.all_apply(k << 1, &self.lazy[k].clone());
        self.all_apply((k << 1) + 1, &self.lazy[k].clone());
        self.lazy[k] = X::e();
    }

    pub fn set(&mut self, i: usize, val: S) {
        let i = self.size + i;
        for j in (1..=self.log).rev() {
            self.push(i >> j);
        }
        self.data[i] = val.clone();
        for j in 1..=self.log {
            self.update(i >> j);
        }
    }

    pub fn get(&mut self, i: usize) -> &S {
        let i = self.size + i;
        for j in (1..=self.log).rev() {
            self.push(i >> j);
        }
        &self.data[i]
    }

    pub fn prod(&mut self, l: usize, r: usize) -> S {
        if l == r {
            return S::e();
        }
        let mut l = self.size + l;
        let mut r = self.size + r;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }

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

    pub fn apply(&mut self, i: usize, x: &X) {
        let i = self.size + i;
        for j in (1..=self.log).rev() {
            self.push(i >> j);
        }
        self.data[i] = x.act(&self.data[i]);
        for j in 1..=self.log {
            self.update(i >> j);
        }
    }

    pub fn apply_range(&mut self, l: usize, r: usize, x: &X) {
        if l == r {
            return;
        }
        let mut l = self.size + l;
        let mut r = self.size + r;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }

        let (l2, r2) = (l, r);
        while l < r {
            if l & 1 == 1 {
                self.all_apply(l, x);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.all_apply(r, x);
            }
            l >>= 1;
            r >>= 1;
        }

        l = l2;
        r = r2;

        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    pub fn max_right<F>(&mut self, l: usize, f: F) -> usize
    where
        F: Fn(&S) -> bool,
    {
        if l == self.n {
            return self.n;
        }

        let mut l = l + self.size;
        for i in (1..=self.log).rev() {
            self.push(l >> i);
        }

        let mut sm = S::e();
        loop {
            while l & 1 == 0 {
                l >>= 1;
            }
            if !(f(&sm.op(&self.data[l]))) {
                while l < self.size {
                    self.push(l);
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

    pub fn min_left<F>(&mut self, r: usize, f: F) -> usize
    where
        F: Fn(&S) -> bool,
    {
        if r == 0 {
            return 0;
        }

        let mut r = r + self.size;
        for i in (1..=self.log).rev() {
            self.push((r - 1) >> i);
        }

        let mut sm = S::e();
        loop {
            r -= 1;
            while r > 0 && r & 1 == 0 {
                r >>= 1;
            }
            if !(f(&self.data[r].op(&sm))) {
                while r < self.size {
                    self.push(r);
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

impl<S: std::fmt::Debug, X: std::fmt::Debug> LazySegTree<S, X> {
    pub fn debug(&self) {
        #[cfg(debug_assertions)]
        println!("---");
        for i in 1..self.size {
            if i > 1 && i.count_ones() == 1 {
                eprintln!();
            }
            eprint!("{:?} / {:?}; ", &self.data[i], &self.lazy[i]);
        }
        eprintln!();
        for i in self.size..self.size << 1 {
            eprint!("{:?} / - ; ", &self.data[i]);
        }
        eprintln!();
    }
}

#[fastout]
fn main() {
    input! {
        w: usize,
        n: usize,
        lr: [(Usize1, Usize1); n],
    }

    impl Monoid for usize {
        fn op(&self, other: &Self) -> Self {
            *self.max(&other)
        }
        fn e() -> Self {
            0
        }
    }

    impl Monoid for Option<usize> {
        fn op(&self, other: &Self) -> Self {
            match (&self, &other) {
                (Some(v), _) => Some(*v),
                (None, &x) => *x,
            }
        }
        fn e() -> Self {
            None
        }
    }

    impl Action<usize> for Option<usize> {
        fn act(&self, s: &usize) -> usize {
            match &self {
                Some(v) => *v,
                None => *s,
            }
        }
    }

    let mut seg = LazySegTree::<usize, Option<usize>>::new(&vec![0; w]);
    for (l, r) in lr {
        let m = seg.prod(l, r + 1);
        seg.apply_range(l, r + 1, &(Some(m + 1)));
        println!("{}", m + 1);
    }
}
