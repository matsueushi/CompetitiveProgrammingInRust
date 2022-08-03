use combination::PrimeModCombination;
use proconio::{fastout, input, marker::Usize1};

const P: u64 = 998244353;

pub mod combination {
    pub struct PrimeModCombination {
        p: u64,
        fact: Vec<u64>,
        fact_inv: Vec<u64>,
    }

    impl PrimeModCombination {
        pub fn new(n: usize, p: u64) -> Self {
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
                let j = i as u64;
                fact[i] = fact[i - 1] * j % p;
                iinv[i] = p - iinv[(p % j) as usize] * (p / j) % p;
                fact_inv[i] = fact_inv[i - 1] * iinv[i] % p;
            }
            Self { p, fact, fact_inv }
        }

        pub fn combination(&self, n: usize, k: usize) -> u64 {
            self.fact[n] * (self.fact_inv[k] * self.fact_inv[n - k] % self.p) % self.p
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut deg = vec![0; n];
    for (u, v) in &uv {
        deg[*u] += 1;
        deg[*v] += 1;
    }
    let n_even = deg.iter().filter(|&x| x % 2 == 0).count();
    let n_odd = n - n_even;

    let mut res = 0;
    let pmc = PrimeModCombination::new(n, P);

    let st = if k >= n_even {
        num::Integer::div_ceil(&(k - n_even), &2)
    } else {
        0
    };
    let en = (k / 2).min(n_odd / 2);
    for i in st..=en {
        res += pmc.combination(n_odd, 2 * i) * pmc.combination(n_even, k - 2 * i);
        res %= P;
    }
    println!("{}", res);
}
