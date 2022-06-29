use proconio::input;

fn dfs(graph: &Vec<Vec<usize>>, mut nvs: &mut Vec<usize>, f: usize, t: usize) {
    for u in &graph[t] {
        if u == &f {
            continue;
        }
        nvs[*u] = nvs[t] + 1;
        dfs(&graph, &mut nvs, t, *u);
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1],
    }
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let mut nvs = vec![0; n];
    dfs(&graph, &mut nvs, 0, 0);
    let pos = (0..n).max_by_key(|i| &nvs[*i]).unwrap();
    let mut nvs = vec![0; n];
    dfs(&graph, &mut nvs, pos, pos);
    println!("{}", nvs.iter().max().unwrap() + 1);
}
