use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        hs: [usize; n],
    }
    let mut hmax = 0;
    let mut res = 0;
    for h in hs {
        if h >= hmax {
            res += 1;
        }
        hmax = hmax.max(h);
    }
    println!("{}", res);
}
