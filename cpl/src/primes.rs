pub mod primes {
    use std::collections::HashMap;

    pub fn primes(n: usize) -> Vec<usize> {
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

    pub fn prime_factor(n: usize) -> HashMap<usize, usize> {
        let mut res = HashMap::new();
        let mut i = 1;
        let mut x = n;
        loop {
            i += 1;
            if i * i > n {
                break;
            }
            while x % i == 0 {
                *res.entry(i).or_insert(0) += 1;
                x /= i;
            }
        }
        if x != 1 {
            res.insert(x, 1);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::primes::*;
    use std::collections::HashMap;
    use std::iter::FromIterator;

    #[test]
    fn test_primes() {
        assert_eq!(primes(10), vec![2, 3, 5, 7]);
        assert_eq!(primes(11), vec![2, 3, 5, 7, 11]);
    }

    #[test]
    fn test_divisor() {
        assert_eq!(divisor(2), vec![1, 2]);
        assert_eq!(divisor(10), vec![1, 10, 2, 5]);
        assert_eq!(divisor(9), vec![1, 9, 3]);
    }

    #[test]
    fn test_prime_factor() {
        assert_eq!(prime_factor(1), HashMap::new());
        assert_eq!(prime_factor(7), HashMap::from_iter([(7, 1)]));
        assert_eq!(prime_factor(8), HashMap::from_iter([(2, 3)]));
    }
}
