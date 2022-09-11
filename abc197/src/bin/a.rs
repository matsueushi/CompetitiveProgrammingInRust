use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    println!("{}{}", &s[1..3], &s[0..1]);
}
