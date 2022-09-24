use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut res = 0;
    for i in 1..n {
        res += n / i;
        if i * (n / i) == n {
            res -= 1;
        }
    }
    println!("{}", res);
}
