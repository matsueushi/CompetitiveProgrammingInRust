use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut tc = vec![0; t + 1];
    for (l, r) in &lr {
        tc[*l] += 1;
        tc[*r] -= 1;
    }
    let mut p = 0;
    for i in 0..t {
        p += tc[i];
        println!("{}", p);
    }
}
