use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut res = false;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if a[i] + a[j] + a[k] == 1000 {
                    res = true;
                }
            }
        }
    }
    println!("{}", if res { "Yes" } else { "No" });
}
