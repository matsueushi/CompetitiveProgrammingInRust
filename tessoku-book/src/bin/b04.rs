use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut res = 0;
    for i in 0..8 {
        let digit = n / 10_usize.pow(i as u32) % 10;
        res += (1 << i) * digit;
    }
    println!("{}", res);
}
