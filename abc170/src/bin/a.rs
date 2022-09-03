use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        xs: [usize; 5],
    }
    for i in 0..5 {
        if xs[i] == 0 {
            println!("{}", i + 1);
            return;
        }
    }
}
