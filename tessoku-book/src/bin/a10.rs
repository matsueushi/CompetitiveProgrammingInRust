use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(Usize1, Usize1); d],
    }

    let mut lmax = vec![0; n + 1];
    let mut rmax = vec![0; n + 1];
    for i in 0..n {
        lmax[i + 1] = lmax[i].max(a[i]);
    }
    for j in (0..n).rev() {
        rmax[j] = rmax[j + 1].max(a[j]);
    }
    for (l, r) in lr {
        println!("{}", lmax[l].max(rmax[r + 1]));
    }
}
