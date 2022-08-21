use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    let mut d = std::f64::MAX;
    for i in 0..n {
        for j in i + 1..n {
            let (x0, y0) = &xy[i];
            let (x1, y1) = &xy[j];
            let dx = *x0 as f64 - *x1 as f64;
            let dy = *y0 as f64 - *y1 as f64;
            d = d.min(dx * dx + dy * dy);
        }
    }
    println!("{}", d.sqrt());
}
