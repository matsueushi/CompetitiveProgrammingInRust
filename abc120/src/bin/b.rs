use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    }
    let mut count = 0;
    for i in (1..=100).rev() {
        if a % i == 0 && b % i == 0 {
            count += 1;
            if count == k {
                println!("{}", i);
                return;
            }
        }
    }
}
