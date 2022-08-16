use proconio::{fastout, input};

pub mod permutation {
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
}

#[fastout]
fn main() {
    use permutation::next_permutation;

    input! {
        n: usize,
        k: usize,
        t: [[usize; n]; n],
    }

    let mut order: Vec<_> = (1..n).collect();
    let mut res = 0;
    loop {
        let mut cost = 0;
        cost += t[0][order[0]];
        for i in 0..n - 2 {
            cost += t[order[i]][order[i + 1]];
        }
        cost += t[order[n - 2]][0];
        if cost == k {
            res += 1;
        }
        if !next_permutation(&mut order) {
            break;
        }
    }
    println!("{}", res);
}
