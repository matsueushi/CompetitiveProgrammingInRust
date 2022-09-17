use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(Usize1, Usize1, Usize1, Usize1); n],
    }

    let mut plane = vec![vec![0; w + 1]; h + 1];
    for (a, b, c, d) in abcd {
        plane[a][b] += 1;
        plane[a][d + 1] -= 1;
        plane[c + 1][b] -= 1;
        plane[c + 1][d + 1] += 1;
    }

    let mut cumsum = vec![vec![0; w + 1]; h + 1];

    // horizontal
    for i in 0..h {
        for j in 0..w {
            cumsum[i + 1][j + 1] = cumsum[i + 1][j] + plane[i][j];
        }
    }
    // vertical
    for j in 0..w {
        for i in 0..h {
            cumsum[i + 1][j + 1] += cumsum[i][j + 1];
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            print!("{}", cumsum[i][j]);
            if j != w {
                print!(" ");
            }
        }
        if i != h {
            println!();
        }
    }
}
