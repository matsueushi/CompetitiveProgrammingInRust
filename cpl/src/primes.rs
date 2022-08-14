pub mod primes {

    pub fn primes(n: usize) -> Vec<usize> {
        let mut primes = Vec::new();
        let mut is_prime = vec![true; n];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..n {
            if is_prime[i] {
                primes.push(i);
                for j in (2 * i..n).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }
        primes
    }
}

#[cfg(test)]
mod tests {
    use super::primes::*;

    #[test]
    fn test_primes() {
        assert_eq!(primes(10), vec![2, 3, 5, 7]);
    }
}
