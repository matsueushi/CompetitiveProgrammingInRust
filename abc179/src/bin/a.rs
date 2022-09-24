use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    if s.chars().last().unwrap() == 's' {
        println!("{}es", s);
    } else {
        println!("{}s", s);
    }
}
