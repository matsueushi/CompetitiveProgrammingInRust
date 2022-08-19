use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut zmin = std::i64::MAX;
    let mut zmax = std::i64::MIN;
    let mut wmin = std::i64::MAX;
    let mut wmax = std::i64::MIN;

    for i in 0..n {
        let (x, y) = &xy[i];
        let z = x + y;
        let w = x - y;
        zmin = zmin.min(z);
        zmax = zmax.max(z);
        wmin = wmin.min(w);
        wmax = wmax.max(w);
    }
    println!("{}", (zmax - zmin).max(wmax - wmin));
}
