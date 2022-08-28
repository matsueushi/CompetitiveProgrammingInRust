use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        n: usize,
        a: [usize; n],
    }
    if a.iter().sum::<usize>() < h {
        println!("No");
    } else {
        println!("Yes");
    }
}
