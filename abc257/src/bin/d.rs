use proconio::input;

fn dfs(dist: &Vec<Vec<i64>>, mut used: &mut Vec<bool>, x: usize, s: i64) -> usize {
    let mut t = 1;
    used[x] = true;
    for v in 0..dist.len() {
        if !used[v] && dist[x][v] <= s {
            t += dfs(&dist, &mut used, v, s);
        }
    }
    t
}

fn main() {
    input! {
        n: usize,
        xyp: [(i64, i64, i64); n]
    }
    let mut dist = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let (xi, yi, pi) = xyp[i];
            let (xj, yj, _) = xyp[j];
            dist[i][j] = num::Integer::div_ceil(&((xi - xj).abs() + (yi - yj).abs()), &pi);
        }
    }

    let mut res = std::i64::MAX;
    for i in 0..n {
        let mut l = 0;
        let mut r = 4 * 1_000_000_000 + 1;
        while r - l > 1 {
            let m = (l + r) / 2;
            let mut used = vec![false; n];
            let t = dfs(&dist, &mut used, i, m);
            if t == n {
                r = m;
            } else {
                l = m;
            }
        }
        res = std::cmp::min(res, r);
    }
    println!("{}", res);
}
