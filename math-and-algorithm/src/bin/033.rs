use num::Integer;
use proconio::{fastout, input};

pub fn inner_product<T>(ax: T, ay: T, bx: T, by: T, cx: T, cy: T) -> T
where
    T: Copy + Integer,
{
    let abx = bx - ax;
    let aby = by - ay;
    let acx = cx - ax;
    let acy = cy - ay;
    abx * acx + aby * acy
}

pub fn dist2<T>(x0: T, y0: T, x1: T, y1: T) -> T
where
    T: Copy + Integer,
{
    let dx = x0 - x1;
    let dy = y0 - y1;
    dx * dx + dy * dy
}

#[fastout]
fn main() {
    input! {
        ax: i64,
        ay: i64,
        bx: i64,
        by: i64,
        cx: i64,
        cy: i64,
    }
    if inner_product(bx, by, ax, ay, cx, cy) >= 0 && inner_product(cx, cy, ax, ay, bx, by) >= 0 {
        let a = cy - by;
        let b = -(cx - bx);
        let c = -bx * cy + cx * by;
        let d = (a * ax + b * ay + c).abs() as f64 / ((a * a + b * b) as f64).sqrt();
        println!("{}", d);
    } else {
        let res = (dist2(ax, ay, bx, by).min(dist2(ax, ay, cx, cy)) as f64).sqrt();
        println!("{}", res);
    }
}
