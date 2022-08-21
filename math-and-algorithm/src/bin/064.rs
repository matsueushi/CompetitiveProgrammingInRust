use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let count: usize = a.iter().sum();
    let res = if count <= k && (k - count) % 2 == 0 {
        "Yes"
    } else {
        "No"
    };
    println!("{}", res);
}
