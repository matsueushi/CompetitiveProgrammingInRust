use proconio::{fastout, input};
use std::cmp::Ordering::{Greater, Less};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        ab: [(usize, usize); n],
        cd: [(usize, usize); m],
    }
    let mut bh = vec![vec![0]; h + 1];
    let mut bw = vec![vec![0]; w + 1];
    for (c, d) in cd {
        bh[c].push(d);
        bw[d].push(c);
    }
    for i in 0..=h {
        bh[i].push(w + 1);
        bh[i].sort();
    }
    for j in 0..=w {
        bw[j].push(h + 1);
        bw[j].sort();
    }

    let mut area = vec![vec![0; w + 2]; h + 2];
    for (a, b) in ab {
        let ix = bh[a]
            .binary_search_by(|&x| if x <= b { Less } else { Greater })
            .unwrap_or_else(|i| i);
        area[a][bh[a][ix - 1] + 1] += 1;
        area[a][bh[a][ix]] -= 1;
        area[a + 1][bh[a][ix - 1] + 1] -= 1;
        area[a + 1][bh[a][ix]] += 1;

        let iy = bw[b]
            .binary_search_by(|&x| if x <= a { Less } else { Greater })
            .unwrap_or_else(|i| i);
        area[bw[b][iy - 1] + 1][b] += 1;
        area[bw[b][iy]][b] -= 1;
        area[bw[b][iy - 1] + 1][b + 1] -= 1;
        area[bw[b][iy]][b + 1] += 1;
    }

    let mut cumsum = vec![vec![0; w + 2]; h + 2];
    // horizontal
    for i in 0..=h {
        for j in 0..=w {
            if j == 0 {
                cumsum[i][j] = area[i][j];
            } else {
                cumsum[i][j] = cumsum[i][j - 1] + area[i][j];
            }
        }
    }
    // vertical
    let mut res = 0;
    for j in 0..=w {
        for i in 0..=h {
            if i == 0 {
                cumsum[i][j] = area[i][j];
            } else {
                cumsum[i][j] += cumsum[i - 1][j];
            }
            if cumsum[i][j] > 0 {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
