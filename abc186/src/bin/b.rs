use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let mut min = 100;
    let mut total = 0;
    for i in 0..h {
        for j in 0..w {
            total += a[i][j];
            min = min.min(a[i][j]);
        }
    }
    println!("{}", total - min * h * w);
}
