use proconio::{fastout, input, marker::Usize1};
use std::collections::VecDeque;

struct State {
    d: usize,
    from: usize,
    to: usize,
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![Vec::new(); n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut prev = vec![0; n];
    let mut dist = vec![std::usize::MAX; n];
    let mut que = VecDeque::new();
    que.push_front(State {
        d: 0,
        from: 0,
        to: 0,
    });
    while let Some(State { d, from, to }) = que.pop_front() {
        dist[to] = d;
        prev[to] = from;
        for v in &graph[to] {
            if d + 1 < dist[*v] {
                que.push_back(State {
                    d: d + 1,
                    from: to,
                    to: *v,
                });
                dist[*v] = d + 1;
                prev[*v] = to;
            }
        }
    }

    println!("Yes");
    for i in 1..n {
        println!("{}", prev[i] + 1);
    }
}
