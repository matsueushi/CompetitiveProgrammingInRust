use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    let mut res = false;
    for ai in a {
        if ai == x {
            res = true;
        }
    }
    println!("{}", if res { "Yes" } else { "No" });
}
