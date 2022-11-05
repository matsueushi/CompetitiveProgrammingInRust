use proconio::{fastout, input};

fn distance(u: &(i64, i64), v: &(i64, i64)) -> f64 {
    let (ux, uy) = u;
    let (vx, vy) = v;
    let dx = ux - vx;
    let dy = uy - vy;
    ((dx * dx + dy * dy) as f64).sqrt()
}

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64,i64); n]
    }
    let mut ord = vec![0; n];
    let mut visited = vec![false; n];

    let mut curpos = 0;
    ord[0] = 0;
    visited[0] = true;

    for i in 1..n {
        let mut min_d = 10000.0;
        let mut min_id = 0;

        for j in 0..n {
            if visited[j] {
                continue;
            }
            let d = distance(&xy[curpos], &xy[j]);
            if d < min_d {
                min_d = d;
                min_id = j;
            }
        }

        ord[i] = min_id;
        visited[min_id] = true;
        curpos = min_id;
    }

    for x in ord {
        println!("{}", x + 1);
    }
    println!("{}", 1);
}
