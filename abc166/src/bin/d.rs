use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64,
    }
    for i in -200_i64..200 {
        for j in -200_i64..200 {
            if i.pow(5) - j.pow(5) == x {
                println!("{} {}", i, j);
                return;
            }
        }
    }
}
