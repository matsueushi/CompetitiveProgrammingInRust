use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        x: i64,
        y: i64,
    }
    if x.abs() + y.abs() <= n && (x + y - n) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
