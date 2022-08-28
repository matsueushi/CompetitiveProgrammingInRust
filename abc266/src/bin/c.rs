use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut xs: [(i64, i64); 4],
    }
    xs.push(xs[0]);
    xs.push(xs[1]);
    for i in 0..4 {
        let (ux, uy) = &xs[i];
        let (vx, vy) = &xs[i + 1];
        let (wx, wy) = &xs[i + 2];

        let uvx = *vx - *ux;
        let uvy = *vy - *uy;

        let vwx = *wx - *vx;
        let vwy = *wy - *vy;
        if uvx * vwy - uvy * vwx < 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
