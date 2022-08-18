use proconio::{fastout, input};

pub mod combination {
    pub struct PrimeModCombination<T> {
        pub iinv: Vec<T>,
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
            Self { iinv }
        }
    }
}

const P: usize = 998244353;

#[fastout]
fn main() {
    use combination::*;

    input! {
        n: usize,
        mut a: [usize; n-1],
    }

    let pmc = PrimeModCombination::new(n, P);
    a.reverse();
    let mut e = 0;
    let mut sum_e = vec![0; n];
    for i in 0..n - 1 {
        sum_e[i + 1] = sum_e[i] + e;
        e = (P + 1 + a[i] + sum_e[i + 1] - sum_e[i + 1 - a[i]]) % P;
        e = e * pmc.iinv[a[i]] % P;
    }
    println!("{}", e);
}
