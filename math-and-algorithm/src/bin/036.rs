use proconio::{fastout, input};
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }
    let t0 = 2. * PI * (h + m / 60.) / 12.;
    let t1 = 2. * PI * m / 60.;
    let (xa, ya) = (a * t0.sin(), a * t0.cos());
    let (xb, yb) = (b * t1.sin(), b * t1.cos());
    let (dx, dy) = (xa - xb, ya - yb);
    let d = (dx * dx + dy * dy).sqrt();
    println!("{}", d);
}
