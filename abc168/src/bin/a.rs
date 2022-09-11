use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    match n % 10 {
        2 | 4 | 5 | 7 | 9 => {
            println!("hon");
        }
        0 | 1 | 6 | 8 => {
            println!("pon");
        }
        _ => {
            println!("bon");
        }
    }
}
