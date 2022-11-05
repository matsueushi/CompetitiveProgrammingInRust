use itertools::Itertools;
use proconio::{fastout, input};

pub fn next_permutation<T: PartialOrd>(nums: &mut [T]) -> bool {
    let last_asc = match nums.windows(2).rposition(|w| w[0] > w[1]) {
        None => {
            nums.reverse();
            return false;
        }
        Some(i) => i,
    };
    match nums[last_asc + 1..]
        .iter()
        .rposition(|x| x < &nums[last_asc])
    {
        None => return false,
        Some(i) => {
            nums.swap(last_asc, last_asc + 1 + i);
            nums[last_asc + 1..].reverse();
        }
    }
    true
}

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ps: [usize; n],
    }
    next_permutation(&mut ps);
    println!("{}", ps.into_iter().map(|x| x.to_string()).join(" "));
}
