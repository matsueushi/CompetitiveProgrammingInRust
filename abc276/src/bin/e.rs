use proconio::{fastout, input, marker::Chars};

const DX: [i64; 4] = [1, 0, -1, 0];
const DY: [i64; 4] = [0, 1, 0, -1];

fn dfs(
    c: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<i64>>,
    (h, w): (i64, i64),
    (sx, sy): (i64, i64),
    (x, y): (i64, i64),
    d: i64,
) -> bool {
    seen[x as usize][y as usize] = d;
    let mut res = false;
    for i in 0..4 {
        let (qx, qy) = (x + DX[i], y + DY[i]);
        if 0 <= qx && qx < h && 0 <= qy && qy < w {
            match c[qx as usize][qy as usize] {
                'S' => {
                    if d + 1 >= 4 {
                        res = true;
                    }
                }
                '#' => {}
                '.' => {
                    if seen[qx as usize][qy as usize] > 0 {
                        continue;
                    }
                    res |= dfs(&c, seen, (h, w), (sx, sy), (qx, qy), d + 1);
                }
                _ => unreachable!(),
            };
        }
    }
    res
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let (mut sx, mut sy) = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 'S' {
                sx = i as i64;
                sy = j as i64;
            }
        }
    }
    let mut seen = vec![vec![-1; w]; h];
    let res = dfs(&c, &mut seen, (h as i64, w as i64), (sx, sy), (sx, sy), 0);
    println!("{}", if res { "Yes" } else { "No" });
}
