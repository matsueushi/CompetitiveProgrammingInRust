use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [(usize, usize); n],
    }
    let mut count = 0;
    for (d1, d2) in d {
        if d1 == d2 {
            count += 1;
            if count == 3 {
                println!("Yes");
                return;
            }
        } else {
            count = 0;
        }
    }
    println!("No");
}
