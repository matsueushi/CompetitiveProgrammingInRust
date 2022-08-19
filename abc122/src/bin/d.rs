use proconio::{fastout, input};

const P: usize = 1_000_000_007;

fn cond(i: usize, j: usize, k: usize) -> bool {
    (i == 0 && j == 1 && k == 2) || (i == 1 && j == 0 && k == 2) || (i == 0 && j == 2 && k == 1)
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut v1: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 4]; 4]; 4];
    let mut v2: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 4]; 4]; 4];

    // A G C T

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                if cond(i, j, k) {
                    continue;
                }
                v2[i][j][k] = 1;
            }
        }
    }

    for _ in 4..=n {
        std::mem::swap(&mut v1, &mut v2);
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    v2[i][j][k] = 0;
                }
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    if cond(i, j, k) {
                        continue;
                    }
                    for l in 0..4 {
                        if (i == 0 && k == 1 && l == 2)
                            || (i == 0 && j == 1 && l == 2)
                            || cond(j, k, l)
                        {
                            continue;
                        }
                        v2[j][k][l] += v1[i][j][k];
                        v2[j][k][l] %= P;
                    }
                }
            }
        }
    }

    let mut res = 0;
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                res += v2[i][j][k];
                res %= P;
            }
        }
    }
    println!("{}", res);
}
