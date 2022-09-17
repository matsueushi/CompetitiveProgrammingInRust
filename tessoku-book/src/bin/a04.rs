use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    for i in (0..10).rev() {
        let div = 1 << i;
        print!("{}", (n / div) % 2);
    }
    println!();
}
