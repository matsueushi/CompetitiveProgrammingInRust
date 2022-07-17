use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n :usize,
        sx: i64,
        sy: i64,
        tx: i64,
        ty: i64,
        circles: [(i64, i64, i64); n],
    }
    // println!("{} {} {} {} {} {:?}", n, sx, sy, tx, ty, circles);
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1, r1) = circles[i];
            let (x2, y2, r2) = circles[j];
            let dx = x1 - x2;
            let dy = y1 - y2;
            let d = dx * dx + dy * dy;
            if d >= (r2 - r1).pow(2) && d <= (r1 + r2).pow(2) {
                uf.union(i, j);
            }
        }
    }
    let mut si = 0;
    let mut ti = 0;
    for i in 0..n {
        let (x, y, r) = circles[i];
        let sdx = sx - x;
        let sdy = sy - y;
        let tdx = tx - x;
        let tdy = ty - y;
        let sd = sdx * sdx + sdy * sdy;
        let td = tdx * tdx + tdy * tdy;
        if sd == r * r {
            si = i;
        }
        if td == r * r {
            ti = i;
        }
    }
    let ok = if uf.equiv(si, ti) { "Yes" } else { "No" };
    println!("{}", ok);
}
