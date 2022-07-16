use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        m: usize,
        x: usize,
        t: usize,
        d: usize,
    }
    let h = if m >= x { t } else { t - (x - m) * d };
    println!("{}", h);
}
