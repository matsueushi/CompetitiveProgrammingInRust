use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn maze(s: &Vec<Vec<char>>, h: usize, w: usize, i: usize, j: usize) -> usize {
    let mut d = vec![vec![std::usize::MAX; w]; h];
    let mut que = VecDeque::new();
    que.push_back((i, j));
    d[i][j] = 0;
    let mut res = 0;
    while !que.is_empty() {
        let (i0, j0) = que.pop_front().unwrap();
        for k in 0..4 {
            let (i1, j1) = (i0 as i32 + DX[k], j0 as i32 + DY[k]);
            if 0 <= i1 && i1 < h as i32 && 0 <= j1 && j1 < w as i32 {
                let iu = i1 as usize;
                let ju = j1 as usize;
                if s[iu][ju] == '.' && d[iu][ju] == std::usize::MAX {
                    que.push_back((iu, ju));
                    d[iu][ju] = d[i0][j0] + 1;
                    res = res.max(d[iu][ju]);
                }
            }
        }
    }
    res
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                res = res.max(maze(&s, h, w, i, j));
            }
        }
    }
    println!("{}", res);
}
