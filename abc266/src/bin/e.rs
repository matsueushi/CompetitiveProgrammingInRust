use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ex = 3.5;
    for _ in 1..n {
        let mut new_ex = 0.0;
        for j in 1..=6 {
            if j as f64 <= ex {
                new_ex += ex;
            } else {
                new_ex += j as f64;
            }
        }
        ex = new_ex / 6.0;
    }
    println!("{}", ex);
}
