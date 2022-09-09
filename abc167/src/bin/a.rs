use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }
    println!("{}", if &s == &t[..s.len()] { "Yes" } else { "No" });
}
