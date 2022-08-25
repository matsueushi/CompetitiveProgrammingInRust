use proconio::{fastout, input, marker::Usize1};

fn dfs(graph: &Vec<Vec<usize>>, color: &mut Vec<i32>, v: usize, c: i32) -> bool {
    color[v] = c;
    for i in 0..graph[v].len() {
        if color[graph[v][i]] == c {
            return false;
        }
        if color[graph[v][i]] == 0 && !dfs(graph, color, graph[v][i], -c) {
            return false;
        }
    }
    true
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut graph = vec![Vec::new(); n];
    for (a, b) in &ab {
        graph[*a].push(*b);
        graph[*b].push(*a);
    }

    let mut color = vec![0; n];
    for i in 0..n {
        if color[i] == 0 {
            if !dfs(&graph, &mut color, i, 1) {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
