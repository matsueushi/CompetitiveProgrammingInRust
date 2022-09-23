use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    for ai in &a {
        if *ai == 0 {
            println!("0");
            return;
        }
    }
    let mut res = 1;
    let limit = 10_usize.pow(18);
    for ai in &a {
        if *ai > limit / res {
            println!("-1");
            return;
        }
        res *= ai;
    }
    println!("{}", res);
}
