use std::collections::VecDeque;

use proconio::{fastout, input, marker::Usize1};

struct State {
    cost: usize,
    pos: usize,
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut g = vec![vec![]; n];
    for (a, b) in &ab {
        g[*a].push(*b);
        g[*b].push(*a);
    }

    let mut deque = VecDeque::new();
    let mut dist = vec![std::usize::MAX; n];
    dist[0] = 0;
    deque.push_back(State { cost: 0, pos: 0 });
    while let Some(State { cost, pos }) = deque.pop_front() {
        if cost > dist[pos] {
            continue;
        }

        for x in &g[pos] {
            if cost + 1 < dist[*x] {
                deque.push_back(State {
                    cost: cost + 1,
                    pos: *x,
                });
                dist[*x] = cost + 1;
            }
        }
    }

    for d in dist {
        if d == std::usize::MAX {
            println!("-1");
        } else {
            println!("{}", d);
        }
    }
}
