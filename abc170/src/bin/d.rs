use proconio::{fastout, input};

const M: usize = 1_000_000;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut count = vec![0; M + 1];
    for ai in &a {
        count[*ai] += 1;
    }
    let mut valid = vec![true; M + 1];
    for ai in &a {
        if !valid[*ai] {
            continue;
        }
        if count[*ai] >= 2 {
            valid[*ai] = false;
        }
        for i in (2 * ai..=M).step_by(*ai) {
            valid[i] = false;
        }
    }
    let mut res = 0;
    for ai in &a {
        if valid[*ai] {
            res += 1;
        }
    }
    println!("{}", res);
}
