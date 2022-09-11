use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut res = true;
    if s.len() <= t.len() {
        for i in 0..s.len() {
            if s[i] != t[i] {
                res = false;
                break;
            }
        }
    } else {
        res = false;
    }
    println!("{}", if res { "Yes" } else { "No" });
}
