use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", a.iter().sum::<usize>() % 100);
}
