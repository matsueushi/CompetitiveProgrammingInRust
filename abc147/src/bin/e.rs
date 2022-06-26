use bitset_fixed::BitSet;
use proconio::input;

const M: usize = 80;

fn solve(h: usize, w: usize, a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> i64 {
    let state = 2 * M * (h + w) + 1;
    let mut dp = vec![vec![BitSet::new(state); w]; h];
    let c = a[0][0] - b[0][0];
    let orig = (M * (h + w)) as i64;
    dp[0][0].set((orig + c) as usize, true);
    dp[0][0].set((orig - c) as usize, true);
    for i in 0..h {
        for j in 0..w {
            if i == 0 && j == 0 {
                continue;
            }
            let c = (a[i][j] - b[i][j]).abs() as usize;
            let mut bset = BitSet::new(state);
            if i != 0 {
                bset |= &(dp[i - 1][j].clone() >> c);
                bset |= &(dp[i - 1][j].clone() << c);
            }
            if j != 0 {
                bset |= &(dp[i][j - 1].clone() >> c);
                bset |= &(dp[i][j - 1].clone() << c);
            }
            dp[i][j] = bset;
        }
    }

    let mut res = std::i64::MAX;
    for s in 0..state {
        if dp[h - 1][w - 1][s] {
            res = std::cmp::min(res, (s as i64 - orig).abs());
        }
    }
    res
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
        b: [[i64; w]; h],
    }
    let res = solve(h, w, &a, &b);
    println!("{}", res);
}
