use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut res = 0;
    for i in 1..=n {
        for j in 1..=n {
            let u = k as i64 - (i + j) as i64;
            if 1 <= u && u <= n as i64 {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
