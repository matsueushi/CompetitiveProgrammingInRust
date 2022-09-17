use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let mut res = false;
    for x in a..=b {
        if 100 % x == 0 {
            res = true;
        }
    }
    println!("{}", if res { "Yes" } else { "No" });
}
