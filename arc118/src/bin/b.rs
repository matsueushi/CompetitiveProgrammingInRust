use num::integer::Integer;
use proconio::{fastout, input};
use std::cmp::Reverse;

#[fastout]
fn main() {
    input! {
        k: usize,
        n: usize,
        m: usize,
        a: [usize; k],
    }
    let mut bs = vec![0; k];
    let mut chgv = Vec::new();
    for i in 0..k {
        let floor = (a[i] * m) / n;
        let ceil = (a[i] * m).div_ceil(&n);
        bs[i] = floor;

        let floor_score = ((floor * n) as i64 - (a[i] * m) as i64).abs();
        let ceil_score = ((ceil * n) as i64 - (a[i] * m) as i64).abs();
        if floor_score > ceil_score {
            chgv.push((0, Reverse(floor_score), i));
        } else {
            chgv.push((1, Reverse(floor_score), i))
        }
    }

    chgv.sort();

    let size: usize = bs.iter().sum();

    for i in 0..(m - size) {
        let (_, _, j) = chgv[i];
        bs[j] += 1;
    }

    for i in 0..bs.len() {
        print!("{}", bs[i]);
        if i != bs.len() - 1 {
            print!(" ")
        }
    }
    println!();
}
