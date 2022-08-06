use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    }
    let asum: i64 = a.iter().sum();

    let mut vl = vec![0; n + 2];
    let mut vr = vec![0; n + 2];
    let mut lcur = 0;
    let mut rcur = 0;
    for i in 0..n {
        lcur += a[i] - l;
        rcur += a[n - 1 - i] - r;
        vl[i + 1] = vl[i].max(lcur);
        vr[n - i] = vr[n - i + 1].max(rcur);
    }

    let mut res = 0;
    for i in 0..n + 1 {
        res = res.max(vl[i] + vr[i + 1]);
    }

    println!("{}", asum - res);
}
