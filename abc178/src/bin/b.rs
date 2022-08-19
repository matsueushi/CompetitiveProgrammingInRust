use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let mut z = std::i64::MIN;
    for x in [a, b].iter() {
        for y in [c, d].iter() {
            z = z.max(x * y);
        }
    }
    println!("{}", z);
}
