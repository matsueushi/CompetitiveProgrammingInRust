use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut l = 0;
    let mut r = 10_usize.pow(9) + 1;
    while r - l > 1 {
        let m = (l + r) / 2;
        let mut cut = 0;
        for x in &a {
            cut += num::Integer::div_ceil(x, &m) - 1;
        }
        if cut <= k {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{}", r);
}
