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

    pub fn divisor(n: usize) -> Vec<usize> {
        let mut res = Vec::new();
        let mut i = 0;
        loop {
            i += 1;
            if i * i > n {
                break;
            }
            if n % i == 0 {
                res.push(i);
                if i != n / i {
                    res.push(n / i);
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::primes::*;

    #[test]
    fn test_primes() {
        assert_eq!(primes(10), vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_divisor() {
        assert_eq!(divisor(2), vec![1, 2]);
        assert_eq!(divisor(10), vec![1, 10, 2, 5]);
        assert_eq!(divisor(9), vec![1, 9, 3]);
    }
}
