use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xu: [(f64, String); n],
    }
    let mut res = 0.0;
    for (x, u) in xu {
        if u == "JPY" {
            res += x;
        } else {
            res += x * 380000.0;
        }
    }
    println!("{}", res);
}
