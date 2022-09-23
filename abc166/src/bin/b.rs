use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut snuke = vec![false; n];
    for _ in 0..k {
        input! {
            d: usize,
            ai: [Usize1; d],
        }
        for aij in ai {
            snuke[aij] = true;
        }
    }
    let mut res = 0;
    for s in snuke {
        if !s {
            res += 1;
        }
    }
    println!("{}", res);
}
