pub fn next_permutation<T: PartialOrd>(nums: &mut [T]) -> bool {
    let last_asc = match nums.windows(2).rposition(|w| w[0] < w[1]) {
        None => {
            nums.reverse();
            return false;
        }
        Some(i) => i,
    };
    match nums[last_asc + 1..]
        .iter()
        .rposition(|x| x > &nums[last_asc])
    {
        None => return false,
        Some(i) => {
            nums.swap(last_asc, last_asc + 1 + i);
            nums[last_asc + 1..].reverse();
        }
    }
    true
}

pub fn pow_mod(x: u64, n: u64, m: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut x = x % m;
    let mut res = 1;
    let mut p = n;
    while p > 0 {
        if p & 1 != 0 {
            res *= x;
            res %= m;
        }
        x *= x;
        x %= m;
        p >>= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_permutation() {
        let mut nums = vec![1, 2, 3];
        assert!(next_permutation(&mut nums));
        assert_eq!(nums, vec![1, 3, 2]);
        assert!(next_permutation(&mut nums));
        assert_eq!(nums, vec![2, 1, 3]);
        assert!(next_permutation(&mut nums));
        assert_eq!(nums, vec![2, 3, 1]);
        assert!(next_permutation(&mut nums));
        assert_eq!(nums, vec![3, 1, 2]);
        assert!(next_permutation(&mut nums));
        assert_eq!(nums, vec![3, 2, 1]);
        assert!(!next_permutation(&mut nums));
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_pow_mod() {
        assert_eq!(pow_mod(5, 3, 19), 11);
        assert_eq!(pow_mod(2, 2, 998244353), 4);
        assert_eq!(pow_mod(2, 3, 7), 1);
        assert_eq!(pow_mod(1, 1, 998244353), 1);
    }
}
