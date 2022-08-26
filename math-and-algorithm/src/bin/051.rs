use proconio::{fastout, input};

pub struct PrimeModCombination<T> {
    p: T,
    fact: Vec<T>,
    fact_inv: Vec<T>,
}

impl PrimeModCombination<usize> {
    pub fn new(n: usize, p: usize) -> Self {
        // i! mod p
        let mut fact = vec![0; n + 1];
        // (i!)^{-1} mod p
        let mut fact_inv = vec![0; n + 1];
        // i^{-1} mod p (i > 0)
        let mut iinv = vec![0; n + 1];
        fact[0] = 1;
        fact[1] = 1;
        fact_inv[0] = 1;
        fact_inv[1] = 1;
        iinv[1] = 1;
        for i in 2..n + 1 {
            fact[i] = fact[i - 1] * i % p;
            iinv[i] = p - iinv[p % i] * (p / i) % p;
            fact_inv[i] = fact_inv[i - 1] * iinv[i] % p;
        }
        Self { p, fact, fact_inv }
    }

    pub fn combination(&self, n: usize, k: usize) -> usize {
        self.fact[n] * (self.fact_inv[k] * self.fact_inv[n - k] % self.p) % self.p
    }
}

const P: usize = 1000000007;

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
    }
    let pmc = PrimeModCombination::new(x + y, P);
    println!("{}", pmc.combination(x + y, x));
}
