use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut primes = Vec::new();
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = true;
    is_prime[1] = true;
    for i in 2..=n {
        if !is_prime[i] {
            continue;
        }
        primes.push(i);
        for j in (2 * i..=n).step_by(i) {
            is_prime[j] = false;
        }
    }

    println!(
        "{}",
        primes
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
