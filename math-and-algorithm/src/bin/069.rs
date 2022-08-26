use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let res = *vec![a * c, a * d, b * c, b * d].iter().max().unwrap();
    println!("{}", res);
}
