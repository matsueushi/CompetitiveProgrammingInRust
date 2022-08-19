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
            let j = i;
            fact[i] = fact[i - 1] * j % p;
            iinv[i] = p - iinv[(p % j) as usize] * (p / j) % p;
            fact_inv[i] = fact_inv[i - 1] * iinv[i] % p;
        }
        Self { p, fact, fact_inv }
    }

    pub fn combination(&self, n: usize, k: usize) -> usize {
        self.fact[n] * (self.fact_inv[k] * self.fact_inv[n - k] % self.p) % self.p
    }
}

const P: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        s: usize,
    }

    let pmc = PrimeModCombination::new(s, P);
    let mut res = 0;
    for j in 1..=(s / 3) {
        res = (res + pmc.combination(s - 3 * j + j - 1, j - 1)) % P;
    }

    println!("{}", res);
}
