use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut c = n;
    for _ in 0..k {
        if c % 200 == 0 {
            c /= 200;
        } else {
            c = 1000 * c + 200;
        }
    }

    println!("{}", c);
}
