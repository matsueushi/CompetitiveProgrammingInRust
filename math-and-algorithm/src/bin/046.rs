use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::VecDeque;

struct State {
    px: usize,
    py: usize,
}

const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

#[fastout]
fn main() {
    input! {
        r: usize,
        c: usize,
        sx: Usize1,
        sy: Usize1,
        gx: Usize1,
        gy: Usize1,
        cs: [Chars; r],
    }
    let mut que = VecDeque::new();
    let mut dist = vec![vec![std::usize::MAX; c]; r];
    dist[sx][sy] = 0;
    que.push_back(State { px: sx, py: sy });
    while let Some(State { px, py }) = que.pop_front() {
        for k in 0..4 {
            let (qx, qy) = (px as i32 + DX[k], py as i32 + DY[k]);
            if 0 <= qx && qx < r as i32 && 0 <= qy && qy < c as i32 {
                let qx = qx as usize;
                let qy = qy as usize;
                if cs[qx][qy] == '.' && dist[qx][qy] == std::usize::MAX {
                    que.push_back(State { px: qx, py: qy });
                    dist[qx][qy] = dist[px][py] + 1;
                }
            }
        }
    }
    println!("{}", dist[gx][gy]);
}
