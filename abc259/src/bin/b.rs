use proconio::{fastout, input};

fn solve(a: f64, b: f64, d: f64) -> (f64, f64) {
    let r = (a * a + b * b).sqrt();
    let rad = b.atan2(a) + (d as f64 / 180.0 * std::f64::consts::PI);
    (r * rad.cos(), r * rad.sin())
}

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        d: i64,
    }
    let (x, y) = solve(a as f64, b as f64, d as f64);
    println!("{} {}", x, y);
}
