use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        k: i64,
    }
    let ka = a.min(k);
    let kb = b.min(k - ka);
    let kc = c.min(k - ka - kb);
    println!("{}", ka - kc);
}
