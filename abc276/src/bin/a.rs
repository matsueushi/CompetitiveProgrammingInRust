use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut c = -1;
    for i in 0..s.len() {
        if s[i] == 'a' {
            c = (i + 1) as i64;
        }
    }
    println!("{}", c);
}
