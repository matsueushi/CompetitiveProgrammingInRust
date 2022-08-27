use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize,
    }
    let cost = if 3 * x > y {
        let m = n / 3;
        (n - 3 * m) * x + m * y
    } else {
        n * x
    };
    println!("{}", cost);
}
