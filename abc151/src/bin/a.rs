use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        c: Chars,
    }
    println!("{}", char::from(c[0] as u8 + 1));
}
